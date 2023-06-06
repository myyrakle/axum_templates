use std::time::Duration;

use axum::body::{Body, BoxBody};
use axum::Json;
use axum::extract::Path,
use axum::{
    http::{Request, Response},
    response::{Html, IntoResponse},
    routing::{get, post, put, delete},
    Router,
};

use super::service::#Name#Service;

use super::dtos::create_#name#_request::Create#Name#Request;
use super::dtos::update_#name#_request::Update#Name#Request;

pub(crate) async fn get_router() -> Router {
    let router = Router::new()
        .route("/", get(get_list))
        .route("/:id", get(get_one))
        .route("/:id", post(post_one))
        .route("/:id", put(health))
        .route("/:id", delete(health))
        .layer(trace);

    router
}

async fn get_list() -> impl IntoResponse {
    let service = #Name#Service::new();

    let response = service.find_all();

    Json(response).into_response()
}


async fn get_one(Path(id): Path<i32>) -> impl IntoResponse {
    let service = #Name#Service::new();

    let response = service.find_one(id);

    Json(response).into_response()
}

async fn post_one(Json(body): Json<Create#Name#Request>) -> impl IntoResponse {
    let service = #Name#Service::new();

    let response = service.create_one(id);

    Json(response).into_response()
}
