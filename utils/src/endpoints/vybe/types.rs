use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VybeTokenDetails {
    pub symbol: String,
    #[serde(rename = "mintAddress")]
    pub mint_address: String,
    pub price: f64,
    #[serde(rename = "price1d")]
    pub price_1d: f64,
    #[serde(rename = "price7d")]
    pub price_7d: f64,
    pub decimal: i32,
    pub verified: bool,
    #[serde(rename = "updateTime")]
    pub update_time: i64,
    #[serde(rename = "currentSupply")]
    pub current_supply: f64,
    #[serde(rename = "marketCap")]
    pub market_cap: f64,

    pub category: Option<String>,
    #[serde(rename = "logoUrl")]
    pub logo_url: Option<String>,
    pub name: Option<String>,
    pub subcategory: Option<String>,
    #[serde(rename = "tokenAmountVolume24h")]
    pub token_amount_volume_24h: Option<f64>,
    #[serde(rename = "usdValueVolume24h")]
    pub usd_value_volume_24h: Option<f64>,
}
