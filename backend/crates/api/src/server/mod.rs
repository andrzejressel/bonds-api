use std::collections::HashMap;

use axum::{body::Body, extract::*, response::Response, routing::*};
use axum_extra::extract::{CookieJar, Host, Query as QueryExtra};
use bytes::Bytes;
use http::{header::CONTENT_TYPE, HeaderMap, HeaderName, HeaderValue, Method, StatusCode};
use tracing::error;
use validator::{Validate, ValidationErrors};

use crate::{header, types::*};

#[allow(unused_imports)]
use crate::{apis, models};

/// Setup API Server.
pub fn new<I, A, E>(api_impl: I) -> Router
where
    I: AsRef<A> + Clone + Send + Sync + 'static,
    A: apis::default::Default<E> + Send + Sync + 'static,
    E: std::fmt::Debug + Send + Sync + 'static,
{
    // build our application with a route
    Router::new()
        .route("/bonds", get(get_bonds::<I, A, E>))
        .route("/bonds/{id}", get(get_bond::<I, A, E>))
        .route("/bonds/{id}/csv", get(get_bond_csv::<I, A, E>))
        .with_state(api_impl)
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
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::default::Default<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
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
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::default::Default<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
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
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::default::Default<E> + Send + Sync,
    E: std::fmt::Debug + Send + Sync + 'static,
{
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
