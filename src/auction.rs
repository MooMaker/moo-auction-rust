use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bid {
    pub bidder_id: String,
    pub amount: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Auction {
    pub auction_id: String,
    pub time_limit: u64,
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
            bids: Vec::new(),
            tokens: HashMap::new(),
            orders: HashMap::new(),
            amms: HashMap::new(),
            metadata: Metadata::default(),
        }
    }

    pub fn add_bid(&mut self, bidder_id: String, amount: u64) {
        let bid = Bid { bidder_id, amount };

        self.bids.push(bid);
    }

    pub 


    // Add more methods as needed for bid validation, handling time limit, etc.
}

pub fn compare_solution(solution_json: String) {
    //TODO get best solution from storage
    let mut best_solution = "{}";
    if compute_value(solution_json) > compute_value(best_solution) {
        best_solution = solution_json;
    }
}

fn compute_value(solution_json: String) -> f64 {
    return 0.0;
}
