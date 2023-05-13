use reqwest::Error;
use std::{collections::HashMap, convert::Infallible, fs, path::Path, sync::Arc};
use tokio::sync::{mpsc, Mutex};
use warp::{ws::Message, Filter, Rejection};

mod handlers;
mod models;
mod routes;
mod ws;

pub const API_HOST: &str = "http://127.0.0.1:8080";
pub const ORDERS_ENDPOINT: &str = "/api/v1/orders";
pub const QUOTING_ENDPOINT: &str = "/api/v1/quote";
pub const ACCOUNT_ENDPOINT: &str = "/api/v1/account";
pub const AUCTION_ENDPOINT: &str = "/api/v1/auction";
pub const TRADES_ENDPOINT: &str = "/api/v1/trades";
pub const VERSION_ENDPOINT: &str = "/api/v1/version";
pub const SOLVER_COMPETITION_ENDPOINT: &str = "/api/v1/solver_competition";

#[derive(Debug, Clone)]
pub struct Client {
    pub client_id: String,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}

type Clients = Arc<Mutex<HashMap<String, Client>>>;
type Result<T> = std::result::Result<T, Rejection>;

#[tokio::main]
async fn main() {
    //setup REST endpoint for solver
    let solver_routes = routes::routes();

    warp::serve(solver_routes).run(([127, 0, 0, 1], 3000)).await;

    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));

    let json_input = read_file_string(
        "C:/Users/Arjan van Hoogdalem/git/moo-offchain/examples/single_order_instance1_input.json",
    );

    println!("Configuring websocket route");
    let ws_route = warp::path("marketmaker")
        .and(warp::ws())
        .and(with_clients(clients.clone()))
        .and_then(handlers::ws_handler);

    let routes = ws_route.with(warp::cors().allow_any_origin());
    println!("Starting server");
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;

    ws::publish_auction(json_input, &clients.clone());
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}

fn read_file_string(filepath: &str) -> String {
    let data = fs::read_to_string(filepath);
    data.unwrap()
}
