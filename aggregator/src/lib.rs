mod ws;

use utils::ENV_CONFIG;
use ws::{VybeWebSocket, VybeWebSocketConfig};

pub async fn aggregate() {
    let config = VybeWebSocketConfig {
        websocket_uri: "wss://api.vybenetwork.xyz/live".to_string(),
        api_key: ENV_CONFIG.vibe_api_key.to_string(),
        on_message: Some(Box::new(|message| {
            println!("Received message: {:?}", message);
        })),
        ..Default::default()
    };

    let mut ws = VybeWebSocket::new(config);
    // ws.connect().await;
    //  Spawn the websocket connection on a separate task
    // let ws_handle = tokio::spawn(async move {
    ws.connect().await;
    println!("WebSocket connection closed");
    // });

    println!("WebSocket client started in background");
}
