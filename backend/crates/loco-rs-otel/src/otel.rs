use crate::config::{LogFormat, OtelConfig, OtelTransport};
use loco_rs::prelude::Result;
use opentelemetry::propagation::TextMapCompositePropagator;
use opentelemetry::{KeyValue, global, trace::TracerProvider as _};
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_otlp::{WithExportConfig, WithHttpConfig};
use opentelemetry_sdk::logs::SdkLoggerProvider;
use opentelemetry_sdk::metrics::SdkMeterProvider;
use opentelemetry_sdk::propagation::{BaggagePropagator, TraceContextPropagator};
use opentelemetry_sdk::{
    Resource,
    metrics::{MeterProviderBuilder, PeriodicReader},
    trace::{RandomIdGenerator, Sampler, SdkTracerProvider},
};
use opentelemetry_semantic_conventions::{SCHEMA_URL, attribute::SERVICE_VERSION};
use tracing_core::LevelFilter;
use tracing_opentelemetry::{MetricsLayer, OpenTelemetryLayer};
use tracing_subscriber::{Layer, layer::SubscriberExt, util::SubscriberInitExt};

pub fn init(config: &OtelConfig) -> Result<()> {
    let meter_provider = create_meter_provider(config)?;
    global::set_meter_provider(meter_provider.clone());

    let tracer_provider = create_tracer_provider(config)?;
    global::set_tracer_provider(tracer_provider.clone());

    let logs_provider = init_logs_provider(config)?;

    // global::set_text_map_propagator(TraceContextPropagator::new());

    global::set_text_map_propagator(TextMapCompositePropagator::new(vec![
        Box::new(TraceContextPropagator::new()),
        Box::new(BaggagePropagator::new()),
    ]));

    let tracer = tracer_provider.tracer("tracing-otel-subscriber");

    let fmt_layer: Box<dyn Layer<tracing_subscriber::registry::Registry> + Send + Sync> =
        match config.log_format {
            LogFormat::Json => Box::new(
                tracing_subscriber::fmt::layer()
                    .json()
                    .with_filter(LevelFilter::INFO),
            ),
            LogFormat::Text => {
                Box::new(tracing_subscriber::fmt::layer().with_filter(LevelFilter::INFO))
            }
        };

    tracing_subscriber::registry()
        // The global level filter prevents the exporter network stack
        // from reentering the globally installed OpenTelemetryLayer with
        // its own spans while exporting, as the libraries should not use
        // tracing levels below DEBUG. If the OpenTelemetry layer needs to
        // trace spans and events with higher verbosity levels, consider using
        // per-layer filtering to target the telemetry layer specifically,
        // e.g. by target matching.
        // .with(tracing_subscriber::filter::LevelFilter::from_level(
        //     Level::INFO,
        // ));
        .with(fmt_layer)
        .with(MetricsLayer::new(meter_provider).with_filter(LevelFilter::INFO))
        .with(OpenTelemetryLayer::new(tracer))
        .with(OpenTelemetryTracingBridge::new(&logs_provider).with_filter(LevelFilter::INFO))
        .init();

    Ok(())
}

// Create a Resource that captures information about the entity for which telemetry is recorded.
fn resource() -> Resource {
    Resource::builder()
        .with_service_name(env!("CARGO_PKG_NAME"))
        .with_schema_url(
            [KeyValue::new(SERVICE_VERSION, env!("CARGO_PKG_VERSION"))],
            SCHEMA_URL,
        )
        .build()
}

fn create_meter_provider(config: &OtelConfig) -> Result<SdkMeterProvider> {
    let exporter = opentelemetry_otlp::MetricExporter::builder();

    let exporter = match &config.common.transport {
        OtelTransport::HTTP(http) => exporter
            .with_http()
            .with_endpoint(format!("{}/v1/metrics", &http.url))
            .with_headers(http.headers.clone()),
    }
    .build();

    let exporter = exporter.map_err(|e| loco_rs::Error::Any(Box::new(e)))?;

    let reader = PeriodicReader::builder(exporter)
        .with_interval(std::time::Duration::from_secs(30))
        .build();

    let meter_provider = MeterProviderBuilder::default()
        .with_resource(resource())
        .with_reader(reader)
        .build();

    Ok(meter_provider)
}

fn create_tracer_provider(config: &OtelConfig) -> Result<SdkTracerProvider> {
    let exporter = opentelemetry_otlp::SpanExporter::builder();

    let exporter = match &config.common.transport {
        OtelTransport::HTTP(http) => exporter
            .with_http()
            .with_endpoint(format!("{}/v1/traces", &http.url))
            .with_headers(http.headers.clone()),
    }
    .build();

    let exporter = exporter.map_err(|e| loco_rs::Error::Any(Box::new(e)))?;

    let tracer_provider = SdkTracerProvider::builder()
        // Customize sampling strategy
        .with_sampler(Sampler::ParentBased(Box::new(Sampler::TraceIdRatioBased(
            1.0,
        ))))
        // If export trace to AWS X-Ray, you can use XrayIdGenerator
        .with_id_generator(RandomIdGenerator::default())
        .with_resource(resource())
        .with_batch_exporter(exporter)
        .build();

    Ok(tracer_provider)
}

fn init_logs_provider(config: &OtelConfig) -> Result<SdkLoggerProvider> {
    let exporter = opentelemetry_otlp::LogExporter::builder();
    let exporter = match &config.common.transport {
        OtelTransport::HTTP(http) => exporter
            .with_http()
            .with_endpoint(format!("{}/v1/logs", &http.url))
            .with_headers(http.headers.clone()),
    }
    .build();

    let exporter = exporter.map_err(|e| loco_rs::Error::Any(Box::new(e)))?;

    let logs_provider = SdkLoggerProvider::builder()
        .with_resource(resource())
        .with_batch_exporter(exporter)
        .build();

    Ok(logs_provider)
}
