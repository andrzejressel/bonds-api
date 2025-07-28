#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::views::obligacje::ObligacjeResponse;
use axum::debug_handler;
use loco_rs::prelude::*;
use tracing::info;

// static OBLIGACJE: Dir = include_dir!("F:\\MojeProgramy\\ObligacjeSkarboweScraping\\output");

#[debug_handler]
#[tracing::instrument(name = "index_obligacje")]
pub async fn index() -> Result<Response> {
    info!("Fetching obligacje data");
    // let response = ObligacjeResponse::new(
    //     OBLIGACJE
    //         .files()
    //         .map(|f| f.path().to_string_lossy().to_string())
    //         .collect(),
    // );
    format::json(ObligacjeResponse::new(vec!["test".to_string()]))
}

pub fn routes() -> Routes {
    Routes::new().prefix("/api/obligacje").add("/", get(index))
}
