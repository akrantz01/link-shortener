use crate::schema::links;
use anyhow::Context;
use diesel::{pg::PgConnection, r2d2::ConnectionManager};
use r2d2::{Error as R2D2Error, Pool as R2D2Pool, PooledConnection};

pub type Pool = R2D2Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = Result<PooledConnection<ConnectionManager<PgConnection>>, R2D2Error>;

embed_migrations!();

/// Connect to the database
pub fn connect() -> anyhow::Result<Pool> {
    // Retrieve the database URL
    let database_url = std::env::var("DATABASE_URL")
        .context("the environment variable 'DATABASE_URL' must be set")?;

    // Connect to the database
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::new(manager).context("failed to create connection pool")?;

    // Retrieve a connection from the pool
    let conn = pool
        .get()
        .context("failed to retrieve connection from the pool")?;

    // Run the database migrations
    embedded_migrations::run(&conn).context("failed to run database migrations")?;

    Ok(pool)
}

/// Retrieve a link from the database
#[derive(Identifiable, Queryable)]
pub struct Link {
    pub id: i32,
    pub name: String,
    pub link: String,
    pub enabled: bool,
    pub times_used: i32,
}

/// Generate a new link from the name and URL
#[derive(Insertable)]
#[table_name = "links"]
pub struct NewLink {
    pub name: String,
    pub link: String,
}
