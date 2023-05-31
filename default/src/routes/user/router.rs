use axum::{extract::Path, response::IntoResponse, routing::get, Json, Router};

use super::dtos::user::User;

pub(crate) async fn get_router() -> Router {
    Router::new()
        .route("/", get(get_user_list))
        .route("/:user_id", get(find_user_by_id))
}

async fn get_user_list() -> impl IntoResponse {
    let user_list = vec![
        User {
            user_id: 1,
            user_name: "test".to_string(),
        },
        User {
            user_id: 2,
            user_name: "test2".to_string(),
        },
    ];

    Json(user_list).into_response()
}

async fn find_user_by_id(Path(user_id): Path<i32>) -> impl IntoResponse {
    let user = User {
        user_id,
        user_name: "test".to_string(),
    };

    Json(user).into_response()
}
