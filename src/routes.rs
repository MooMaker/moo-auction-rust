use std::convert::Infallible;
use warp::{self, Filter};

use crate::handlers;

// All routes
pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    post_cow_auction()
}

fn post_cow_auction() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("cow_auction")
        .and(warp::post())
        .and_then(handlers::auction_handler)
}
