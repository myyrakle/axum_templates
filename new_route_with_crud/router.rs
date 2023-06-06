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
use super::dtos::create_#name#_response::Create#Name#Response;
use super::dtos::get_#name#_response::Get#Name#Response;
use super::dtos::list_#name#_response::List#Name#Response;
use super::dtos::update_#name#_request::Update#Name#Request;
use super::dtos::update_#name#_response::Update#Name#Response;
use super::dtos::delete_#name#_response::Delete#Name#Response;

pub(crate) async fn get_router() -> Router {
    let router = Router::new()
        .route("/", get(get_list))
        .route("/:id", get(get_one))
        .route("/:id", post(health))
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
