use std::time::Duration;

use axum::body::{Body, BoxBody};
use axum::{
    http::{Request, Response},
    response::{Html, IntoResponse},
    routing::get,
    Json, Router,
};
use tower_http::trace::TraceLayer;
use tracing::Span;

pub(crate) async fn get_router() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/", get(health))
        .layer(trace)
}

async fn index() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

use super::dtos::health_response::HealthReponse;

async fn health() -> impl IntoResponse {
    let server_ok = true;

    Json(HealthReponse { server_ok }).into_response()
}
