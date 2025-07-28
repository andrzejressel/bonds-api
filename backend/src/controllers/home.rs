use crate::views::home::HomeResponse;
use axum::debug_handler;
use loco_rs::prelude::*;
use tracing::{Level, info, span};

#[debug_handler]
async fn current() -> Result<Response> {
    info!("current");
    test();
    format::json(HomeResponse::new("loco"))
}

#[tracing::instrument]
fn test() {
    let my_span = span!(Level::INFO, "my_span", answer = 42);
    let _ = my_span.enter();

    info!("test function")
}

pub fn routes() -> Routes {
    Routes::new().prefix("/api").add("/", get(current))
}
