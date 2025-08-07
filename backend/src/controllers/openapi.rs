use crate::services::bonds::{BondId, BondsService, BondsServiceImpl};
use anyhow::{Context, Error};
use async_trait::async_trait;
use axum::http::Method;
use axum_extra::extract::{CookieJar, Host};
use loco_rs::app::AppContext;
use loco_rs::controller::Routes;
use openapi::apis::ErrorHandler;
use openapi::apis::default::GetBondsResponse::Status200_AJSONArrayOfBondNames;
use openapi::apis::default::{GetBondCsvResponse, GetBondResponse, GetBondsResponse};
use openapi::models::{GetBond404Response, GetBondCsvPathParams, GetBondPathParams};

struct ServerImpl {
    bonds_service: Box<dyn BondsService + Send + Sync>,
}

impl AsRef<ServerImpl> for ServerImpl {
    fn as_ref(&self) -> &ServerImpl {
        self
    }
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
    #[tracing::instrument(err(Debug), skip(self, method, host, cookies), name = "get_bond")]
    async fn get_bond(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        path_params: &GetBondPathParams,
    ) -> Result<GetBondResponse, Error> {
        Err(anyhow::anyhow!("TEST").context("Failed to get bond"))
    }

    #[tracing::instrument(err(Debug), skip(self, method, host, cookies), name = "get_bond_csv")]
    async fn get_bond_csv(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        path_params: &GetBondCsvPathParams,
    ) -> Result<GetBondCsvResponse, Error> {
        let bond_id = BondId::new(path_params.id.clone());

        match self.bonds_service.get_bond(&bond_id) {
            Some(bond) => {
                let csv_data = bond.to_csv();
                Ok(GetBondCsvResponse::Status200_BondDataInCSVFormat(csv_data))
            }
            None => Ok(GetBondCsvResponse::Status404_BondNotFound(
                GetBond404Response::new(format!(
                    "Bond with ID {} not found",
                    path_params.id.clone()
                )),
            )),
        }
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

impl ErrorHandler for ServerImpl {}

pub(crate) fn get_routes(ctx: &AppContext) -> loco_rs::Result<Routes> {
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

    let app = openapi::server::new(ctx, ServerImpl::new(bonds_service));
    Ok(app)
}
