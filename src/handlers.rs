use crate::{models::Auction, ws, Clients, Result};
use rabbitmq_stream_client::{types::Message, Environment};
use reqwest::StatusCode;
use warp::Reply;

pub async fn ws_handler(ws: warp::ws::Ws, clients: Clients) -> Result<impl Reply> {
    println!("ws_handler");

    Ok(ws.on_upgrade(move |socket| ws::client_connection(socket, clients)))
}

pub async fn auction_handler(auction: Auction) -> Result<impl warp::Reply> {
    // pub async fn auction_handler(auction: Auction) {
    let json = serde_json::to_string(&auction).unwrap();
    println!("auction_handler");

    let environment = Environment::builder().build().await.unwrap();
    let producer = environment.producer().build("mystream").await.unwrap();

    producer
        .send_with_confirm(Message::builder().body(format!("{}", json)).build())
        .await
        .unwrap();
    producer.close().await.unwrap();
    //TODO: parse query parameters is metadata for auction

    //TODO: fwd to MM via ws
    //publish to rabbitmq queue
    // ws::publish_auction(json, &clients.clone());

    //TODO
    Ok(StatusCode::NO_CONTENT)
}

pub async fn announce_winner(auction: Auction) -> Result<impl warp::Reply> {
    println!("announcing winning auction");

    //TODO: fwd to MM via ws

    Ok(StatusCode::NO_CONTENT)
}
