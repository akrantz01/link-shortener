use crate::{
    database::{DbConnection, Link, NewLink, UpdatableLink},
    errors::{to_rejection, ApiError},
};
use diesel::prelude::*;
use warp::http::{StatusCode, Uri};

/// Create a new link to redirect to
pub async fn create_link(
    new_link: NewLink,
    conn: DbConnection,
) -> Result<impl warp::Reply, warp::Rejection> {
    use crate::schema::links;

    // Extract the connection from the error
    let conn = conn.map_err(to_rejection)?;

    // Ensure the link is a valid URI
    if let Err(e) = url_is_valid(&new_link.link) {
        return Err(to_rejection(e));
    }

    // Insert the new link to the database
    let link = diesel::insert_into(links::table)
        .values(&new_link)
        .get_result::<Link>(&conn)
        .map_err(to_rejection)?;

    Ok(warp::reply::with_status(
        warp::reply::json(&json!({ "success": true, "data": link })),
        StatusCode::CREATED,
    ))
}

/// Get a list of all links
pub async fn list_links(conn: DbConnection) -> Result<impl warp::Reply, warp::Rejection> {
    use crate::schema::links::dsl::*;

    // Extract the connection from the error
    let conn = conn.map_err(to_rejection)?;

    // Retrieve all links
    let list = links.get_results::<Link>(&conn).map_err(to_rejection)?;

    Ok(warp::reply::json(&json!({ "success": true, "data": list })))
}

/// Update a given link by its id
pub async fn update_link(
    link_id: i32,
    changeset: UpdatableLink,
    conn: DbConnection,
) -> Result<impl warp::Reply, warp::Rejection> {
    use crate::schema::links::{dsl, table};

    // Extract the connection from the error
    let conn = conn.map_err(to_rejection)?;

    // Check if the provided link is valid
    if let Some(l) = &changeset.link {
        url_is_valid(l).map_err(to_rejection)?;
    }

    // Update the link
    diesel::update(table)
        .filter(dsl::id.eq(link_id))
        .set(&changeset)
        .execute(&conn)
        .map_err(to_rejection)?;

    Ok(warp::reply::with_status(
        warp::reply::reply(),
        StatusCode::NO_CONTENT,
    ))
}

/// Delete a given link by its id
pub async fn delete_link(
    link_id: i32,
    conn: DbConnection,
) -> Result<impl warp::Reply, warp::Rejection> {
    use crate::schema::links::dsl::*;

    // Extract the connection from the error
    let conn = conn.map_err(to_rejection)?;

    // Delete the specified link
    diesel::delete(links.filter(id.eq(link_id)))
        .execute(&conn)
        .map_err(to_rejection)?;

    Ok(warp::reply::with_status(
        warp::reply::reply(),
        StatusCode::NO_CONTENT,
    ))
}

/// Ensure a URL is valid
fn url_is_valid(unvalidated: &str) -> Result<(), ApiError> {
    match unvalidated.parse::<Uri>() {
        // Check if the scheme is valid
        Ok(u) => match u.scheme() {
            Some(_) => Ok(()),
            None => Err(ApiError::new(
                "the link scheme must be http or https".into(),
                StatusCode::BAD_REQUEST,
            )),
        },
        Err(e) => Err(ApiError::new(
            format!("bad link, {}", e),
            StatusCode::BAD_REQUEST,
        )),
    }
}
