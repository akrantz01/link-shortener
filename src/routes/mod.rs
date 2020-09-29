use crate::database::Pool;
use warp::{Filter, Rejection};

mod api;
mod auth;
mod redirect;
mod ui;

use auth::auth;

/// Build the routing table for the API
pub fn build(pool: Pool) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
    // Create a filter for the pool
    let pool_filter = warp::any().map(move || pool.get());

    // Redirect handler
    let redirect = warp::get()
        .and(warp::path::param())
        .and(warp::path::end())
        .and(pool_filter.clone())
        .and_then(redirect::redirect_link);

    // UI static files handlers
    let ui_index = warp::get()
        .and(warp::path::path("ui"))
        .and(warp::path::end())
        .and_then(ui::serve_index);
    let ui_arbitrary = warp::get()
        .and(warp::path::path("ui"))
        .and(warp::path::tail())
        .and_then(|t: warp::path::Tail| ui::serve_arbitrary(t.as_str().to_owned()));

    // API handlers
    let api_create = warp::post()
        .and(warp::path::path("api"))
        .and(warp::path::end())
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and(pool_filter.clone())
        .and_then(api::create_link);
    let api_list = warp::get()
        .and(warp::path::path("api"))
        .and(warp::path::end())
        .and(pool_filter.clone())
        .and_then(api::list_links);
    let api_update = warp::put()
        .and(path!("api" / i32))
        .and(warp::path::end())
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and(pool_filter.clone())
        .and_then(api::update_link);
    let api_delete = warp::delete()
        .and(path!("api" / i32))
        .and(warp::path::end())
        .and(pool_filter)
        .and_then(api::delete_link);

    // The set of routes to be protected
    let protected = ui_index
        .or(ui_arbitrary)
        .or(api_create)
        .or(api_list)
        .or(api_update)
        .or(api_delete);

    auth().and(protected).or(redirect)
}
