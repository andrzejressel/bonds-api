use async_trait::async_trait;
use axum::Router;
use axum::http::Method;
use axum_extra::extract::{CookieJar, Host};
use loco_rs::app::AppContext;
use loco_rs::prelude::Initializer;
use openapi::apis::default::GetBondsResponse;
use openapi::apis::default::GetBondsResponse::Status200_AJSONArrayOfBondNames;
use std::sync::Arc;
use tracing::info;

struct ServerImpl {
    // database: sea_orm::DbConn,
}

#[allow(unused_variables)]
#[async_trait]
impl openapi::apis::default::Default for ServerImpl {
    #[tracing::instrument(skip_all, name = "get_bonds")]
    async fn get_bonds(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
    ) -> Result<GetBondsResponse, ()> {
        info!("get_bonds");
        Ok(Status200_AJSONArrayOfBondNames(vec![
            "Bond1".to_string(),
            "Bond2".to_string(),
        ]))
    }
}

impl openapi::apis::ErrorHandler for ServerImpl {}

pub(crate) struct OpenApiInitializer;

impl OpenApiInitializer {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Initializer for OpenApiInitializer {
    fn name(&self) -> String {
        "OpenAPI Initializer".to_string()
    }

    async fn before_run(&self, _app_context: &AppContext) -> loco_rs::Result<()> {
        Ok(())
    }

    async fn after_routes(&self, router: Router, _ctx: &AppContext) -> loco_rs::Result<Router> {
        let app = openapi::server::new(Arc::new(ServerImpl {}));
        Ok(router.merge(app))
    }
}

//
// let app = openapi::server::new(Arc::new(ServerImpl {}));
//
// Ok(router.merge(app))
//
