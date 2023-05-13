pub struct MarketMakerAuction {
    pub uuid: String,
    pub tokens: Vec<Token>,
    pub orders: Vec<Order>,
    pub amms: Vec<Amms>,
    pub metadata: Vec<Metadata>,
}

pub struct Token {
    pub address: String,
    pub decimals: u16,
    pub alias: String,
    pub external_price: f64,
    pub normalize_priority: u16,
    pub internal_buffer: String,
}

pub struct Order {
    pub order_id: u64,
}

pub struct Amms {
    pub amms_id: u64,
}

pub struct Metadata {
    pub environment: String,
}
