use std::time::Duration;

use axum::{error_handling::HandleErrorLayer, http::StatusCode, BoxError, Json};
use serde_json::json;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use utils::ENV_CONFIG;

use crate::{route::new_router, utils::shutdown_signal};

pub async fn start() {
    tracing::debug!("env_config: {:?}", *ENV_CONFIG);

    let app = new_router().layer(
        ServiceBuilder::new()
            .layer(HandleErrorLayer::new(|error: BoxError| async move {
                if error.is::<tower::timeout::error::Elapsed>() {
                    Ok(StatusCode::REQUEST_TIMEOUT)
                } else {
                    Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled internal error: {error}"),
                    ))
                }
            }))
            .timeout(Duration::from_secs(10))
            .layer(TraceLayer::new_for_http())
            .into_inner(),
    );

    // Create a `TcpListener` using tokio.
    let listener = TcpListener::bind(format!("127.0.0.1:{}", ENV_CONFIG.port))
        .await
        .unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    // 404 pages
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
