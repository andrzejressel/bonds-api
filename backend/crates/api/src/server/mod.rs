use std::collections::HashMap;

use axum::{body::Body, extract::*, response::Response, routing::*};
use axum_extra::extract::{CookieJar, Host, Query as QueryExtra};
use bytes::Bytes;
use http::{header::CONTENT_TYPE, HeaderMap, HeaderName, HeaderValue, Method, StatusCode};
use tracing::error;
use validator::{Validate, ValidationErrors};

use crate::{header, types::*};

use loco_rs::prelude::{AppContext, Routes};

#[allow(unused_imports)]
use crate::{apis, models};

/// Setup API Server.
pub fn new<I, A, E>(ctx: &AppContext, api_impl: I) -> Routes
where
    I: AsRef<A> + Send + Sync + 'static,
    A: apis::default::Default<E> + Send + Sync + 'static,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    ctx.shared_store.insert(api_impl);

    // build our application with a route
    Routes::new()
        .add("/bonds", get(get_bonds::<I, A, E>))
        .add("/bonds/{id}", get(get_bond::<I, A, E>))
        .add("/bonds/{id}/csv", get(get_bond_csv::<I, A, E>))
}

#[tracing::instrument(skip_all)]
fn get_bond_validation(
    path_params: models::GetBondPathParams,
) -> std::result::Result<(models::GetBondPathParams,), ValidationErrors> {
    path_params.validate()?;

    Ok((path_params,))
}
/// GetBond - GET /bonds/{id}
#[tracing::instrument(skip_all)]
async fn get_bond<I, A, E>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    Path(path_params): Path<models::GetBondPathParams>,
    State(app_context): State<AppContext>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync + 'static,
    A: apis::default::Default<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    // SAFETY - We know that I is in shared store, because the only way to get here is through the `new` function which inserts it into the shared store.
    let api_impl = unsafe { app_context.shared_store.get_ref::<I>().unwrap_unchecked() };

    let validation = get_bond_validation(path_params);

    let Ok((path_params,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .get_bond(&method, &host, &cookies, &path_params)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::default::GetBondResponse::Status200_ASingleBondObject(body) => {
                let mut response = response.status(200);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?,
                    );
                }

                let body_content = serde_json::to_vec(&body).map_err(|e| {
                    error!(error = ?e);
                    StatusCode::INTERNAL_SERVER_ERROR
                })?;
                response.body(Body::from(body_content))
            }
            apis::default::GetBondResponse::Status404_BondNotFound(body) => {
                let mut response = response.status(404);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?,
                    );
                }

                let body_content = serde_json::to_vec(&body).map_err(|e| {
                    error!(error = ?e);
                    StatusCode::INTERNAL_SERVER_ERROR
                })?;
                response.body(Body::from(body_content))
            }
        },
        Err(why) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            return api_impl
                .as_ref()
                .handle_error(&method, &host, &cookies, why)
                .await;
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn get_bond_csv_validation(
    path_params: models::GetBondCsvPathParams,
) -> std::result::Result<(models::GetBondCsvPathParams,), ValidationErrors> {
    path_params.validate()?;

    Ok((path_params,))
}
/// GetBondCsv - GET /bonds/{id}/csv
#[tracing::instrument(skip_all)]
async fn get_bond_csv<I, A, E>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    Path(path_params): Path<models::GetBondCsvPathParams>,
    State(app_context): State<AppContext>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync + 'static,
    A: apis::default::Default<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    // SAFETY - We know that I is in shared store, because the only way to get here is through the `new` function which inserts it into the shared store.
    let api_impl = unsafe { app_context.shared_store.get_ref::<I>().unwrap_unchecked() };

    let validation = get_bond_csv_validation(path_params);

    let Ok((path_params,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .get_bond_csv(&method, &host, &cookies, &path_params)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::default::GetBondCsvResponse::Status200_BondDataInCSVFormat(body) => {
                let mut response = response.status(200);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("text/csv").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?,
                    );
                }

                let body_content = body;
                response.body(Body::from(body_content))
            }
            apis::default::GetBondCsvResponse::Status404_BondNotFound(body) => {
                let mut response = response.status(404);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?,
                    );
                }

                let body_content = serde_json::to_vec(&body).map_err(|e| {
                    error!(error = ?e);
                    StatusCode::INTERNAL_SERVER_ERROR
                })?;
                response.body(Body::from(body_content))
            }
        },
        Err(why) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            return api_impl
                .as_ref()
                .handle_error(&method, &host, &cookies, why)
                .await;
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn get_bonds_validation() -> std::result::Result<(), ValidationErrors> {
    Ok(())
}
/// GetBonds - GET /bonds
#[tracing::instrument(skip_all)]
async fn get_bonds<I, A, E>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    State(app_context): State<AppContext>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync + 'static,
    A: apis::default::Default<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    // SAFETY - We know that I is in shared store, because the only way to get here is through the `new` function which inserts it into the shared store.
    let api_impl = unsafe { app_context.shared_store.get_ref::<I>().unwrap_unchecked() };

    let validation = get_bonds_validation();

    let Ok(()) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl.as_ref().get_bonds(&method, &host, &cookies).await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::default::GetBondsResponse::Status200_AJSONArrayOfBondNames(body) => {
                let mut response = response.status(200);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?,
                    );
                }

                let body_content = serde_json::to_vec(&body).map_err(|e| {
                    error!(error = ?e);
                    StatusCode::INTERNAL_SERVER_ERROR
                })?;
                response.body(Body::from(body_content))
            }
        },
        Err(why) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            return api_impl
                .as_ref()
                .handle_error(&method, &host, &cookies, why)
                .await;
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}
