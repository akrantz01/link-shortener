#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use anyhow::Context;
use dotenv::dotenv;
use warp::Filter;

mod database;
mod errors;
mod routes;
mod schema;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    // Connect to the database
    let database_pool = database::connect().context("failed to connect to the database")?;

    // Generate system routes
    let routes = routes::build(database_pool).recover(errors::handle_rejection);

    // Start the server
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}