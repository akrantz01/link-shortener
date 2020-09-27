use crate::database::Pool;
use warp::{Filter, Rejection};

mod auth;
mod redirect;
mod ui;

use auth::auth;

/// Build the routing table for the API
pub fn build(pool: Pool) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
    // Create a filter for the pool
    let pool_collection = warp::any().map(move || pool.get());

    // Redirect handler
    let redirect = warp::path::param()
        .and(warp::path::end())
        .and(pool_collection.clone())
        .and_then(redirect::redirect_link);

    // UI static files handlers
    let ui_index = warp::path("ui")
        .and(warp::path::end())
        .and_then(ui::serve_index);
    let ui_arbitrary = warp::path("ui")
        .and(warp::path::tail())
        .and_then(ui::serve_arbitrary);

    // The set of routes to be protected
    let protected = ui_index.or(ui_arbitrary);

    auth().and(protected).or(redirect)
}
