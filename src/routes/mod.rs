use crate::database::Pool;
use warp::Filter;

mod redirect;

/// Build the routing table for the API
pub fn build(
    pool: Pool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // Create a filter for the pool
    let pool_collection = warp::any().map(move || pool.get());

    // Build the route list
    warp::path::param()
        .and(warp::path::end())
        .and(pool_collection.clone())
        .and_then(redirect::redirect_link)
}
