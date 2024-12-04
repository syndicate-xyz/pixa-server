use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde_json::json;

use crate::{handlers::telegram_hook, utils::route};

pub fn new_router() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .merge(hook_routes())
}

async fn hello_world() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({
            "message": "Hello, World!"
        })),
    )
}

fn hook_routes() -> Router {
    route("/hook", post(telegram_hook))
}
