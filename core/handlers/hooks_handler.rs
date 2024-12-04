use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use telegram::telegram_bot_entrypoint;
use tokio;

pub async fn telegram_hook(Json(body): Json<serde_json::Value>) -> impl IntoResponse {
    tokio::spawn(async move {
        telegram_bot_entrypoint();
    });

    tracing::info!("Received telegram webhook request: {:?}", body);

    (
        StatusCode::OK,
        Json(json!({
            "message": "Telegram worker service spawned!"
        })),
    )
}
