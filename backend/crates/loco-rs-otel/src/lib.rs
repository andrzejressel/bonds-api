mod config;
mod otel;

use crate::config::{Otel, RootSettings};
use crate::otel::init;
use axum::routing::Router;
use axum_otel_metrics::HttpMetricsLayerBuilder;
use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};
use loco_rs::app::AppContext;
use loco_rs::prelude::{Initializer, async_trait};
use loco_rs::{Error, Result};

pub struct OtelInitializer {}

impl OtelInitializer {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Initializer for OtelInitializer {
    fn name(&self) -> String {
        "Otel Initializer".to_string()
    }

    async fn before_run(&self, app_context: &AppContext) -> Result<()> {
        let settings = &app_context.config.settings;

        match settings {
            Some(s) => match serde_json::from_value::<RootSettings>(s.clone()) {
                Ok(c) => match c.otel {
                    Otel::Enabled(settings) => init(&settings),
                    Otel::Disabled => {
                        println!("OpenTelemetry is disabled in the configuration.");
                        Ok(())
                    }
                },
                Err(e) => Err(Error::from(e)),
            },
            None => Err(Error::string("Configuration missing required 'settings'")),
        }
    }

    async fn after_routes(&self, router: Router, _ctx: &AppContext) -> Result<Router> {
        let router = router
            .layer(OtelInResponseLayer::default())
            .layer(OtelAxumLayer::default())
            .layer(HttpMetricsLayerBuilder::new().build());
        Ok(router)
    }
}
