use crate::{database::DbConnection, database::Link, errors::to_rejection};
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use warp::http::Uri;

/// Handle a redirection to a given URL
/// This is the core handler that enables the short link functionality
pub async fn redirect_link(
    path: String,
    conn: DbConnection,
) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    use crate::schema::links::dsl::*;

    // Extract the connection from the error
    let conn = conn.map_err(to_rejection)?;

    // Fetch all the link data
    let found_result = links.filter(name.eq(path)).first::<Link>(&conn);
    let found = match found_result {
        Ok(l) => l,
        Err(DieselError::NotFound) => {
            return Ok(Box::new(super::ui::serve_arbitrary("404.html").await?))
        }
        Err(e) => return Err(to_rejection(e)),
    };

    // Fail if the link is disabled
    if !found.enabled {
        return Ok(Box::new(super::ui::serve_arbitrary("404.html").await?));
    }

    // Convert the plain link to a URI
    let uri = found.link.parse::<Uri>().unwrap();

    // Increment the number of times the link was used
    diesel::update(&found)
        .set(times_used.eq(times_used + 1))
        .execute(&conn)
        .map_err(to_rejection)?;

    Ok(Box::new(warp::redirect::temporary(uri)))
}
