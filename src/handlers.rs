use crate::{models::Auction, ws, Clients, Result};
use reqwest::StatusCode;
use warp::Reply;

pub async fn ws_handler(ws: warp::ws::Ws, clients: Clients) -> Result<impl Reply> {
    println!("ws_handler");

    Ok(ws.on_upgrade(move |socket| ws::client_connection(socket, clients)))
}

pub async fn auction_handler(auction: Auction) -> Result<impl warp::Reply> {
    let json = serde_json::to_string(&auction);
    println!("{}", json.unwrap());
    println!("auction_handler");

    //query parameters is metadata for auction

    //TODO: fwd to MM via ws
    // ws::publish_auction(json_input, &clients.clone());

    //TODO
    Ok(StatusCode::NO_CONTENT)
}

pub async fn announce_winner(auction: Auction) -> Result<impl warp::Reply> {
    println!("announcing winning auction");

    //TODO: fwd to MM via ws

    Ok(StatusCode::NO_CONTENT)
}
