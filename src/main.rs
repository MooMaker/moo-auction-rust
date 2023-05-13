use futures::future;
use std::fs::File;
use std::io::prelude::*;
use std::thread;
use std::{collections::HashMap, convert::Infallible, sync::Arc};
use tokio::sync::{mpsc, Mutex};
use warp::{ws::Message, Filter, Rejection};

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

// static clients: Clients = Arc::new(Mutex::new(HashMap::new()));

#[tokio::main]
async fn main() {
    //setup REST endpoint for solver
    tokio::spawn(async move {
        let solver_routes = routes::routes();
        warp::serve(solver_routes).run(([127, 0, 0, 1], 3000)).await;
    });

    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));
    // save_clients_to_json(&clients);

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

// fn save_clients_to_json(clients: &Clients) {
//     // Serialize the 'clients' HashMap to JSON
//     let serialized =
//         serde_json::to_string(&*clients.lock().unwrap()).expect("Failed to serialize to JSON");

//     // Open the file in write mode
//     let mut file = File::create("clients.json").expect("Failed to create file");

//     // Write the JSON string to the file
//     file.write_all(serialized.as_bytes())
//         .expect("Failed to write to file");
// }
