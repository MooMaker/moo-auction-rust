use crate::auction::MooAuction;
use crate::MOOAUCTION;
use crate::{models::Auction, ws, Clients, Result, CLIENTS};
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
    MOOAUCTION = MooAuction::new("auction1".into(), 30);
    ws::publish_auction(json, &CLIENTS.clone()).await;

    let no_bid: String = String::from("{}");
    // Run the auction
    // if let Some(best_bid) = mooAuction.run().await {
    //     println!("Auction closed. Best bid: {:?}", best_bid);
    //     Ok(warp::reply::json(&best_bid))
    // } else {
    Ok(warp::reply::json(&no_bid))
    // }
}

pub async fn announce_winner(auction: Auction) -> Result<impl warp::Reply> {
    println!("announcing winning auction:");

    //TODO: fwd to MM via ws

    Ok(StatusCode::NO_CONTENT)
}
