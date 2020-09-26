use crate::{database::DbConnection, database::Link, errors::to_rejection};
use diesel::prelude::*;
use warp::http::Uri;

/// Handle a redirection to a given URL
/// This is the core handler that enables the short link functionality
pub async fn redirect_link(
    path: String,
    conn: DbConnection,
) -> Result<impl warp::Reply, warp::Rejection> {
    use crate::schema::links::dsl::*;

    // Extract the connection from the error
    let conn = conn.map_err(to_rejection)?;

    // Fetch all the link data
    let found = links
        .filter(name.eq(path))
        .first::<Link>(&conn)
        .map_err(to_rejection)?;

    // Convert the plain link to a URI
    let uri = found.link.parse::<Uri>().unwrap();

    // Increment the number of times the link was used
    diesel::update(&found)
        .set(times_used.eq(times_used + 1))
        .execute(&conn)
        .map_err(to_rejection)?;

    Ok(warp::redirect::temporary(uri))
}
