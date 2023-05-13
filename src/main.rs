use std::{collections::HashMap, convert::Infallible, fs, sync::Arc};
use tokio::sync::{mpsc, Mutex};
use warp::{ws::Message, Filter, Rejection};

use std::sync::mpsc::channel;
use std::thread;

mod handlers;
mod models;
mod routes;
mod ws;

#[derive(Debug, Clone)]
pub struct Client {
    pub client_id: String,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}

type Clients = Arc<Mutex<HashMap<String, Client>>>;
type Result<T> = std::result::Result<T, Rejection>;

#[tokio::main]
async fn main() {
    // Create a simple streaming channel
    let (tx, rx) = channel();
    thread::spawn(move || {
        tx.send(10).unwrap();
    });
    assert_eq!(rx.recv().unwrap(), 10);

    //setup REST endpoint for solver
    let solver_routes = routes::routes();
    warp::serve(solver_routes).run(([127, 0, 0, 1], 3000)).await;

    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));
    fs::write("clients.txt", "dsfsf").expect("Unable to write file");

    println!("Configuring websocket route");
    let ws_route = warp::path("marketmaker")
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

fn read_file_string(filepath: &str) -> String {
    let data = fs::read_to_string(filepath);
    data.unwrap()
}
