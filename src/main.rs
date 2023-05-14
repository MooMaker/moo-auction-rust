use auction::MooAuction;
use once_cell::sync::Lazy;
use std::{collections::HashMap, convert::Infallible, sync::Arc};
use tokio::sync::{mpsc, Mutex};
use warp::{ws::Message, Filter, Rejection};

mod auction;
mod handlers;
mod models;
mod routes;
mod ws;

#[derive(Debug, Clone)]
pub struct Client {
    pub client_id: String,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}

pub type Clients = Arc<Mutex<HashMap<String, Client>>>;
type Result<T> = std::result::Result<T, Rejection>;

pub static CLIENTS: Lazy<Clients> = Lazy::new(Default::default);

//Original auction has 0 timelimit because it's not an actual auction but represents an empty init
pub static MOOAUCTION: Lazy<MooAuction> = Lazy::new(|| MooAuction::new("auction1".into(), 0));

#[tokio::main]
async fn main() {
    //setup REST endpoint for solver
    tokio::spawn(async move {
        let solver_routes = routes::routes();
        warp::serve(solver_routes).run(([127, 0, 0, 1], 3000)).await;
    });

    println!("Configuring websocket route");
    let ws_route = warp::path("marketmaker")
        .and(warp::ws())
        .and(with_clients(CLIENTS.clone()))
        .and_then(handlers::ws_handler);

    let routes = ws_route.with(warp::cors().allow_any_origin());
    println!("Starting server");
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}
