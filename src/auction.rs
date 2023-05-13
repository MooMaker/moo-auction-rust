use crate::models::{Amm, Bid, Metadata, Order, Token};
use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MooAuction {
    pub auction_id: String,
    pub time_limit: u64,
    pub deadline: Instant,
    pub winning_bid: Option<Bid>,
    pub bids: Vec<Bid>,
    pub tokens: HashMap<String, Token>,
    pub orders: HashMap<u64, Order>,
    pub amms: HashMap<u64, Amm>,
    pub metadata: Metadata,
}

impl MooAuction {
    pub fn new(auction_id: String, time_limit: u64) -> Self {
        Self {
            auction_id,
            time_limit,
            deadline: Instant::now() + Duration::from_secs(time_limit),
            winning_bid: None,
            bids: Vec::new(),
            tokens: HashMap::new(),
            orders: HashMap::new(),
            amms: HashMap::new(),
            metadata: Metadata::default(),
        }
    }

    pub fn add_bid(&mut self, bid: Bid) {
        // Check if the bid matches the order
        if let Some(order) = self.orders.get(&order_id) {
            if bid.bidder_id == order.sell_token && bid.bidder_id == order.buy_token {
                self.bids.push(bid.clone());

                // Check and update the winning bid
                if self.winning_bid.is_none() {
                    self.winning_bid = Some(bid);
                } else if order.is_sell_order
                    && bid.amount > self.winning_bid.as_ref().unwrap().amount
                {
                    self.winning_bid = Some(bid);
                } else if !order.is_sell_order
                    && bid.amount < self.winning_bid.as_ref().unwrap().amount
                {
                    self.winning_bid = Some(bid);
                }
            }
        }
    }

    pub fn check_auction_end(&self) -> Option<Bid> {
        if Instant::now() >= self.deadline {
            return self.winning_bid.clone();
        }
        None
    }
}
