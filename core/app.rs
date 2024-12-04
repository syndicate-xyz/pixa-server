use std::time::Duration;

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Json, Router,
};
use serde_json::json;
use tokio::net::TcpListener;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;

use crate::{config::load_env_config, route::new_router, utils::shutdown_signal};

pub async fn start() {
    let env_config = load_env_config().unwrap();

    let app = new_router().layer((
        TraceLayer::new_for_http(),
        // Graceful shutdown will wait for outstanding requests to complete. Add a timeout so
        // requests don't hang forever.
        TimeoutLayer::new(Duration::from_secs(10)),
    ));

    // Create a `TcpListener` using tokio.
    let listener = TcpListener::bind(format!("127.0.0.1:{}", env_config.port))
        .await
        .unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    let app = app.fallback(|| async {
        (
            StatusCode::NOT_FOUND,
            Json(json!({
                "message": "Not Found"
            })),
        )
    });

    // Run the server with graceful shutdown
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}
