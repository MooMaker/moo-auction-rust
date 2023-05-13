use crate::{models::Auction, ws, Clients, Result};
// use rabbitmq_    stream_client::{types::Message, Environment};
// use reqwest::StatusCode;
use warp::Reply;
use warp::{http::StatusCode, reply, Filter};

pub async fn ws_handler(ws: warp::ws::Ws, clients: Clients) -> Result<impl Reply> {
    println!("ws_handler");

    Ok(ws.on_upgrade(move |socket| ws::client_connection(socket, clients)))
}

pub async fn auction_handler(auction: Auction) -> Result<impl warp::Reply> {
    let json: String = serde_json::to_string(&auction).unwrap();
    println!("auction_handler");

    //TODO: get information from header
    // let mooAuction = MooAuction::new("auction1".into(), 30);

    //TODO: fwd to MM via ws
    // ws::publish_auction(json_input, &clients.clone());

    // Run the auction
    // if let Some(best_bid) = mooAuction.run().await {
    //     println!("Auction closed. Best bid: {:?}", best_bid);
    // } else {
    //     println!("Auction closed. No bids were placed.");
    // }

    //TODO: send 200 with response from the auction (via listener?)
    Ok(warp::reply::json(&json))
    // Ok(StatusCode::OK)
}

pub async fn announce_winner(auction: Auction) -> Result<impl warp::Reply> {
    println!("announcing winning auction");

    //TODO: fwd to MM via ws

    Ok(StatusCode::NO_CONTENT)
}
