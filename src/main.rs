use reqwest::Error;
use std::{collections::HashMap, convert::Infallible, sync::Arc};
use tokio::sync::{mpsc, Mutex};
use warp::{ws::Message, Filter, Rejection};

mod handlers;
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
    let body = get_auction().await;
    println!("{}", body);

    let input_string: String = "{}".to_string();
    let response = post_solution(input_string).await;
    println!("{}", response);

    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));

    println!("Configuring websocket route");
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(with_clients(clients.clone()))
        .and_then(handlers::ws_handler);

    let routes = ws_route.with(warp::cors().allow_any_origin());
    println!("Starting server");
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}

async fn get_auction() -> String {
    //TODO error handling instead of unwrapping (https://stackoverflow.com/questions/58373663/cannot-use-the-operator-in-a-function-that-returns)
    let request_url = "https://api.cow.fi/goerli/api/v1/auction";
    let body = reqwest::get(request_url)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    body
}

async fn post_solution(body: String) -> String {
    let request_url = "https://api.cow.fi/goerli/api/v1/solver_competition/7809184";
    let client = reqwest::Client::new();
    let response = client
        .post(request_url)
        .body(body)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    response
}
