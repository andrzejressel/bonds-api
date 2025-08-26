mod config;
mod otel;

use crate::config::OtelConfig;
use crate::otel::init;
use axum::routing::Router;
use axum_otel_metrics::HttpMetricsLayerBuilder;
use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};
use loco_rs::app::AppContext;
use loco_rs::prelude::{Initializer, async_trait};
use loco_rs::{Error, Result};
use tracing::log::warn;

#[derive(Default)]
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
        let settings = &app_context
            .config
            .initializers
            .as_ref()
            .and_then(|s| s.get("otel"))
            .cloned();

        match settings {
            Some(s) => match serde_json::from_value::<OtelConfig>(s.clone()) {
                Ok(v) => init(&v),
                Err(e) => Err(Error::from(e)),
            },
            None => {
                warn!(
                    "OpenTelemetry configuration not found in the settings. Please ensure it is properly configured."
                );
                Ok(())
            }
        }
    }

    async fn after_routes(&self, router: Router, _ctx: &AppContext) -> Result<Router> {
        let router = router
            .layer(OtelInResponseLayer)
            .layer(OtelAxumLayer::default())
            .layer(HttpMetricsLayerBuilder::new().build());
        Ok(router)
    }
}
