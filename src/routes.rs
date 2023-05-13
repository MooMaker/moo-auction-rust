use warp::{self, Filter};

use super::models;
use crate::handlers;

// All routes
pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    post_cow_auction()
}

fn post_cow_auction() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("cow_auction")
        .and(warp::post())
        .and(json_body())
        .and_then(handlers::auction_handler)
}

fn json_body() -> impl Filter<Extract = (models::Auction,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 512).and(warp::body::json())
    // TODO remove payload size limit completely (for larger orders)
}

fn post_announce_winner() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    warp::path("announce_winner")
        .and(warp::post())
        .and(json_body())
        .and_then(handlers::announce_winner)
}
