use crate::{ws, Clients, Result};
use reqwest::StatusCode;
use warp::Reply;

pub async fn ws_handler(ws: warp::ws::Ws, clients: Clients) -> Result<impl Reply> {
    println!("ws_handler");

    Ok(ws.on_upgrade(move |socket| ws::client_connection(socket, clients)))
}

pub async fn auction_handler() -> Result<impl warp::Reply> {
    println!("auction_handler");
    //query parameters is metadata for auction

    //TODO: fwd to MM via ws
    // ws::publish_auction(json_input, &clients.clone());

    //TODO
    Ok(StatusCode::NO_CONTENT)
}

pub async fn announce_winner() -> Result<impl warp::Reply> {
    println!("announcing winning auction");

    //TODO: fwd to MM via ws

    Ok(StatusCode::NO_CONTENT)
}
