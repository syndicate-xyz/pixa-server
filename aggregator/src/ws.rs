use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::time::sleep;
use tokio_tungstenite::{
    connect_async, tungstenite::client::IntoClientRequest, tungstenite::protocol::Message,
};
use url::Url;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TradingProgram {
    MeteoraDelMM,
    MeteoraPools,
    LifinitySwapV2,
    LifinitySwapV1,
    OpenbookV2,
    RaydiumV4,
    RaydiumCLMM,
    OrcaWhirlpool,
    Phoenix,
    PumpFun,
}

impl TradingProgram {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MeteoraDelMM => "METEORA_DLMM",
            Self::MeteoraPools => "METEORA_POOLS",
            Self::LifinitySwapV2 => "LIFINITY_SWAP_V2",
            Self::LifinitySwapV1 => "LIFINITY_SWAP_V1",
            Self::OpenbookV2 => "OPENBOOK_V2",
            Self::RaydiumV4 => "RAYDIUM_V4",
            Self::RaydiumCLMM => "RAYDIUM_CLMM",
            Self::OrcaWhirlpool => "ORCA_WHIRPOOL",
            Self::Phoenix => "PHOENIX",
            Self::PumpFun => "PUMP_FUN",
        }
    }

    pub fn program_id(&self) -> &'static str {
        match self {
            Self::MeteoraDelMM => "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo",
            Self::MeteoraPools => "Eo7WjKq67rjJQSZxS6z3YkapzY3eMj6Xy8X5EQVn5UaB",
            Self::LifinitySwapV2 => "2wT8Yq49kHgDzXuPxZSaeLaH1qbmGXtEyPy64bL7aD3c",
            Self::LifinitySwapV1 => "EewxydAPCCVuNEyrVN68PuSYdQ7wKn27V9Gjeoi8dy3S",
            Self::OpenbookV2 => "opnb2LAfJYbRMAHHvqjCwQxanZn7ReEHp1k81EohpZb",
            Self::RaydiumV4 => "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8",
            Self::RaydiumCLMM => "CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK",
            Self::OrcaWhirlpool => "whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc",
            Self::Phoenix => "PhoeNiXZ8ByJGLkxNfZRnkUfjvmuYqLR89jjFHGqdXY",
            Self::PumpFun => "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "METEORA_DLMM" => Some(Self::MeteoraDelMM),
            "METEORA_POOLS" => Some(Self::MeteoraPools),
            "LIFINITY_SWAP_V2" => Some(Self::LifinitySwapV2),
            "LIFINITY_SWAP_V1" => Some(Self::LifinitySwapV1),
            "OPENBOOK_V2" => Some(Self::OpenbookV2),
            "RAYDIUM_V4" => Some(Self::RaydiumV4),
            "RAYDIUM_CLMM" => Some(Self::RaydiumCLMM),
            "ORCA_WHIRPOOL" => Some(Self::OrcaWhirlpool),
            "PHOENIX" => Some(Self::Phoenix),
            "PUMP_FUN" => Some(Self::PumpFun),
            _ => None,
        }
    }
}

// Rust equivalents of TypeScript interfaces
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TradeFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tokenMintAddress")]
    pub token_mint_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "feePayer")]
    pub fee_payer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "programId")]
    pub program_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "authorityAddress")]
    pub authority_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "marketId")]
    pub market_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "quoteMintAddress")]
    pub quote_mint_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "baseMintAddress")]
    pub base_mint_address: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransferFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "feePayer")]
    pub fee_payer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "minAmount")]
    pub min_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "maxAmount")]
    pub max_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "programId")]
    pub program_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "receiverAddress")]
    pub receiver_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "receiverTokenAccount")]
    pub receiver_token_account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "senderAddress")]
    pub sender_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "senderTokenAccount")]
    pub sender_token_account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tokenMintAddress")]
    pub token_mint_address: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OraclePriceFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "priceFeedAccount")]
    pub price_feed_account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "productAccount")]
    pub product_account: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Filters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trades: Option<Vec<TradeFilter>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfers: Option<Vec<TransferFilter>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "oraclePrices")]
    pub oracle_prices: Option<Vec<OraclePriceFilter>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigureMessage {
    pub r#type: String,
    pub filters: Filters,
}

#[derive(Deserialize, Debug, Clone)]
pub struct VybeMessage {
    #[serde(rename = "authorityAddress")]
    pub authority_address: String,
    #[serde(rename = "blockTime")]
    pub block_time: u64,
    #[serde(rename = "iixOrdinal")]
    pub iix_ordinal: u32,
    #[serde(rename = "baseMintAddress")]
    pub base_mint_address: String,
    #[serde(rename = "interIxOrdinal")]
    pub inter_ix_ordinal: u32,
    #[serde(rename = "ixOrdinal")]
    pub ix_ordinal: u32,
    #[serde(rename = "marketId")]
    pub market_id: String,
    #[serde(rename = "quoteMintAddress")]
    pub quote_mint_address: String,
    pub price: String,
    #[serde(rename = "programId")]
    pub program_id: String,
    pub signature: String,
    pub slot: u64,
    #[serde(rename = "txIndex")]
    pub tx_index: u32,
    pub fee: String,
    #[serde(rename = "feePayer")]
    pub fee_payer: String,
    #[serde(rename = "baseSize")]
    pub base_size: String,
    #[serde(rename = "quoteSize")]
    pub quote_size: String,
}

// Callback types for event handlers
pub type MessageCallback = Box<dyn Fn(VybeMessage) + Send + Sync>;
pub type ConnectCallback = Box<dyn Fn() + Send + Sync>;
pub type DisconnectCallback = Box<dyn Fn() + Send + Sync>;
pub type ErrorCallback = Box<dyn Fn(String) + Send + Sync>;

pub struct VybeWebSocketConfig {
    pub websocket_uri: String,
    pub api_key: String,
    pub base_reconnect_delay: u64,
    pub reconnect: bool,
    pub configure_message: ConfigureMessage,
    pub on_message: Option<MessageCallback>,
    pub on_connect: Option<ConnectCallback>,
    pub on_disconnect: Option<DisconnectCallback>,
    pub on_error: Option<ErrorCallback>,
}

impl Default for VybeWebSocketConfig {
    fn default() -> Self {
        Self {
            websocket_uri: "".to_string(),
            api_key: "".to_string(),
            base_reconnect_delay: 1000,
            reconnect: true,
            configure_message: ConfigureMessage {
                r#type: "configure".to_string(),
                filters: Filters {
                    trades: Some(vec![
                        TradeFilter {
                            // token_mint_address: Some(
                            //     "DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263".to_string()),
                            token_mint_address: None,
                            program_id: Some(TradingProgram::RaydiumV4.program_id().to_string()),
                            fee_payer: None,
                            authority_address: None,
                            market_id: None,
                            quote_mint_address: None,
                            base_mint_address: None,
                        },
                        TradeFilter {
                            token_mint_address: None,
                            program_id: Some(TradingProgram::RaydiumV4.program_id().to_string()),
                            fee_payer: None,
                            authority_address: None,
                            market_id: None,
                            quote_mint_address: None,
                            base_mint_address: None,
                        },
                    ]),
                    transfers: None,
                    oracle_prices: None,
                },
            },
            on_message: None,
            on_connect: None,
            on_disconnect: None,
            on_error: None,
        }
    }
}

pub struct VybeWebSocket {
    config: VybeWebSocketConfig,
    shutdown_tx: Option<Sender<()>>,
}

impl VybeWebSocket {
    pub fn new(config: VybeWebSocketConfig) -> Self {
        Self {
            config,
            shutdown_tx: None,
        }
    }

    pub fn connect(
        &mut self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + '_>> {
        // Create a shutdown channel
        let (shutdown_tx, shutdown_rx) = mpsc::channel::<()>(1);
        self.shutdown_tx = Some(shutdown_tx);

        // Default callbacks if not provided
        let on_message = self.config.on_message.take().unwrap_or_else(|| {
            Box::new(|message: VybeMessage| {
                println!(
                    "Trade: {} tokens for {} USDC at price {}, signature: {}",
                    message.base_size, message.quote_size, message.price, message.signature
                );
            })
        });

        let on_connect = self.config.on_connect.take().unwrap_or_else(|| {
            Box::new(|| {
                println!("Connected to WebSocket");
            })
        });

        let on_disconnect = self.config.on_disconnect.take().unwrap_or_else(|| {
            Box::new(|| {
                println!("Disconnected from WebSocket");
            })
        });

        let on_error = self.config.on_error.take().unwrap_or_else(|| {
            Box::new(|error: String| {
                println!("WebSocket error: {}", error);
            })
        });

        Box::pin(self.connect_with_callbacks(
            on_message,
            on_connect,
            on_disconnect,
            on_error,
            shutdown_rx,
        ))
    }

    async fn connect_with_callbacks(
        &mut self,
        on_message: MessageCallback,
        on_connect: ConnectCallback,
        on_disconnect: DisconnectCallback,
        on_error: ErrorCallback,
        mut shutdown_rx: Receiver<()>,
    ) {
        // Parse the WebSocket URI
        let url = match Url::parse(&self.config.websocket_uri) {
            Ok(url) => url,
            Err(e) => {
                on_error(format!("Failed to parse WebSocket URI: {}", e));
                self.handle_reconnect(&on_disconnect, &on_error).await;
                return;
            }
        };

        // Add API key header
        let mut request = url.into_client_request().unwrap();
        request
            .headers_mut()
            .insert("X-API-Key", self.config.api_key.clone().parse().unwrap());

        // Connect to the WebSocket server
        let (ws_stream, _) = match connect_async(request).await {
            Ok((ws_stream, response)) => (ws_stream, response),
            Err(e) => {
                on_error(format!("Failed to connect: {}", e));
                self.handle_reconnect(&on_disconnect, &on_error).await;
                return;
            }
        };

        on_connect();

        // Send configure message
        let (mut write, mut read) = ws_stream.split();
        match serde_json::to_string(&self.config.configure_message) {
            Ok(configure_message) => {
                if let Err(e) = write.send(Message::Text(configure_message)).await {
                    on_error(format!("Failed to send configure message: {}", e));
                    self.handle_reconnect(&on_disconnect, &on_error).await;
                    return;
                }
            }
            Err(e) => {
                on_error(format!("Failed to serialize configure message: {}", e));
                self.handle_reconnect(&on_disconnect, &on_error).await;
                return;
            }
        }

        // Create a future that completes when the shutdown channel receives a message
        let ws_task = async {
            // Handle incoming messages
            while let Some(message_result) = read.next().await {
                match message_result {
                    Ok(message) => match message {
                        Message::Text(text) => match serde_json::from_str::<VybeMessage>(&text) {
                            Ok(parsed_message) => on_message(parsed_message),
                            Err(e) => on_error(format!("Failed to parse message: {}", e)),
                        },
                        Message::Close(_) => {
                            on_disconnect();
                            self.handle_reconnect(&on_disconnect, &on_error).await;
                            break;
                        }
                        _ => {} // Ignore other message types
                    },
                    Err(e) => {
                        on_error(format!("WebSocket error: {}", e));
                        self.handle_reconnect(&on_disconnect, &on_error).await;
                        break;
                    }
                }
            }
        };

        // Use tokio::select to race between the WebSocket task and the shutdown signal
        tokio::select! {
            _ = ws_task => {
                // WebSocket task completed naturally
            }
            _ = shutdown_rx.recv() => {
                // Received shutdown signal, close the WebSocket connection gracefully
                if let Err(e) = write.send(Message::Close(None)).await {
                    on_error(format!("Failed to close WebSocket: {}", e));
                }
                on_disconnect();
            }
        }
    }

    async fn handle_reconnect(
        &mut self,
        on_disconnect: &DisconnectCallback,
        on_error: &ErrorCallback,
    ) {
        if !self.config.reconnect {
            return;
        }

        let delay = self.config.base_reconnect_delay;
        println!("Attempting to reconnect in {}ms...", delay);

        sleep(Duration::from_millis(delay)).await;

        // Clone necessary values for the reconnection
        let websocket_uri = self.config.websocket_uri.clone();
        let api_key = self.config.api_key.clone();
        let base_reconnect_delay = self.config.base_reconnect_delay;
        let reconnect = self.config.reconnect;
        let configure_message = self.config.configure_message.clone();

        // Spawn a new task to handle reconnection
        tokio::spawn(async move {
            let mut new_ws = VybeWebSocket::new(VybeWebSocketConfig {
                websocket_uri,
                api_key,
                base_reconnect_delay,
                reconnect,
                configure_message,
                on_message: None,
                on_connect: None,
                on_disconnect: None,
                on_error: None,
            });
            new_ws.connect().await;
        });
    }

    pub fn disconnect(&mut self) {
        // Send shutdown signal if we have a sender
        if let Some(tx) = self.shutdown_tx.take() {
            // Don't wait for the result, this is a best-effort send
            let _ = tx.try_send(());
            println!("Disconnect signal sent");
        }

        // Set reconnect to false to prevent further reconnection attempts
        self.config.reconnect = false;
    }
}
