use std::time::Duration;

use axum::body::{Body, BoxBody};
use axum::Json;
use axum::{
    http::{Request, Response},
    response::{Html, IntoResponse},
    routing::{get, post, put, delete},
    Router,
};

pub(crate) async fn get_router() -> Router {
    let router = Router::new()
        .route("/", get(get_list))
        .route("/:id", get(health))
        .route("/:id", post(health))
        .route("/:id", put(health))
        .route("/:id", delete(health))
        .layer(trace);

    router
}

async fn get_list() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

use super::service::#Name#Service;

async fn health() -> impl IntoResponse {
    let service = #Name#Service::new();

    let response = service.get_health();

    Json(response).into_response()
}
