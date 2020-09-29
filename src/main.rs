#[deny(unused_variables, warnings, dead_code)]
// OpenSSL must be before diesel to statically compile
// See https://github.com/emk/rust-musl-builder/issues/69
extern crate openssl;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate warp;

use anyhow::Context;
use dotenv::dotenv;
use std::{env, net::SocketAddr};
use warp::Filter;

mod database;
mod errors;
mod routes;
mod schema;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    // Initialize logging
    pretty_env_logger::init();
    let logger = warp::log("link_shortener::api");

    // Connect to the database
    let database_pool = database::connect().context("failed to connect to the database")?;

    // Generate system routes
    let routes = routes::build(database_pool)
        .with(logger)
        .recover(errors::handle_rejection)
        .boxed();

    // Parse the serve address
    let address = env::var("ADDRESS")
        .unwrap_or_else(|_| "127.0.0.1:3030".into())
        .parse::<SocketAddr>()
        .context("invalid listen address")?;

    // Start the server
    warp::serve(routes).run(address).await;

    Ok(())
}
