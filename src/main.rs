#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use anyhow::Context;
use dotenv::dotenv;
use warp::Filter;

mod database;
mod errors;
mod schema;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    // Connect to the database
    let database_pool = database::connect().context("failed to connect to the database")?;
    let database_filter = warp::any().map(move || database_pool.clone());

    Ok(())
}
