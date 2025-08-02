use crate::services::bonds::{BondsService, BondsServiceImpl};
use anyhow::{Context, Error};
use async_trait::async_trait;
use axum::Router;
use axum::http::Method;
use axum_extra::extract::{CookieJar, Host};
use loco_rs::app::AppContext;
use loco_rs::prelude::Initializer;
use openapi::apis::ErrorHandler;
use openapi::apis::default::GetBondsResponse::Status200_AJSONArrayOfBondNames;
use openapi::apis::default::{GetBondResponse, GetBondsResponse};
use openapi::models::GetBondPathParams;
use std::sync::Arc;

struct ServerImpl {
    bonds_service: Box<dyn BondsService + Send + Sync>,
}

impl ServerImpl {
    fn new(bonds_service: impl BondsService + Send + Sync + 'static) -> Self {
        Self {
            bonds_service: Box::new(bonds_service),
        }
    }
}

impl ErrorHandler<Error> for ServerImpl {}

#[allow(unused_variables)]
#[async_trait]
impl openapi::apis::default::Default<Error> for ServerImpl {
    // Instrument and skip everything except for path_params
    #[tracing::instrument(err(Debug), ret, skip(self, method, host, cookies), name = "get_bond")]
    async fn get_bond(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        path_params: &GetBondPathParams,
    ) -> Result<GetBondResponse, Error> {
        Err(anyhow::anyhow!("TEST").context("Failed to get bond"))
    }

    #[tracing::instrument(err(Debug), ret, skip(self, method, host, cookies), name = "get_bonds")]
    async fn get_bonds(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
    ) -> Result<GetBondsResponse, Error> {
        Ok(Status200_AJSONArrayOfBondNames(
            self.bonds_service
                .get_bonds()
                .into_iter()
                .map(|bond_id| bond_id.value())
                .collect(),
        ))
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

    async fn after_routes(&self, router: Router, ctx: &AppContext) -> loco_rs::Result<Router> {
        let settings = &ctx
            .config
            .settings
            .clone()
            .context("Setting key in settings not found")
            .map_err(|e| loco_rs::Error::from(e.into_boxed_dyn_error()))?;

        let settings = settings
            .get("bonds_location")
            .context("Setting->bonds_location setting not found")
            .map_err(|e| loco_rs::Error::from(e.into_boxed_dyn_error()))?;

        let settings = settings
            .as_str()
            .context("Setting->bonds_location is not a string")
            .map_err(|e| loco_rs::Error::from(e.into_boxed_dyn_error()))?;

        let bonds_service = BondsServiceImpl::new(settings)
            .context("Failed to create BondsService")
            .map_err(|e| loco_rs::Error::from(e.into_boxed_dyn_error()))?;

        let app = openapi::server::new(Arc::new(ServerImpl::new(bonds_service)));
        Ok(router.merge(app))
    }
}
