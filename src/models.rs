use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Auction {
    pub tokens: HashMap<String, Token>,
    pub orders: HashMap<u64, Order>,
    pub amms: HashMap<u64, Amm>,
    pub metadata: Metadata,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub decimals: i64,
    pub alias: Option<String>,
    #[serde(rename = "external_price")]
    pub external_price: Option<f64>,
    #[serde(rename = "normalize_priority")]
    pub normalize_priority: i64,
    #[serde(rename = "internal_buffer")]
    pub internal_buffer: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    #[serde(rename = "sell_token")]
    pub sell_token: String,
    #[serde(rename = "buy_token")]
    pub buy_token: String,
    #[serde(rename = "sell_amount")]
    pub sell_amount: String,
    #[serde(rename = "buy_amount")]
    pub buy_amount: String,
    #[serde(rename = "allow_partial_fill")]
    pub allow_partial_fill: bool,
    #[serde(rename = "is_sell_order")]
    pub is_sell_order: bool,
    pub fee: Fee,
    pub cost: Cost,
    #[serde(rename = "is_liquidity_order")]
    pub is_liquidity_order: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fee {
    pub amount: String,
    pub token: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cost {
    pub amount: String,
    pub token: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Amm {
    pub kind: String,
    // pub reserves: Reserves,
    pub fee: String,
    pub cost: Cost,
    pub mandatory: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reserves {
    pub n0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48: String,
    pub n0xc00e94cb662c3520282e6f5717214004a7f26888: String,
}
// TODO implement this somehow

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub environment: String,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct maker_bid {
    pub id: u64,
    pub token_in: String,
    pub token_out: String,
    pub amount_in: String,
    pub amount_out: String,
    pub valid_to: u64,
    pub maker_address: String,
    pub maker_signature: String,
}
