use crate::errors::{to_rejection, Error};
use rust_embed::RustEmbed;
use warp::{
    http::{header::HeaderValue, StatusCode},
    reply::Response,
    Rejection, Reply,
};

#[derive(RustEmbed)]
#[folder = "ui"]
struct Assets;

/// Serve index.html
pub async fn serve_index() -> Result<impl Reply, Rejection> {
    serve("index.html")
}

/// Serve an arbitrary file at a given path
pub async fn serve_arbitrary<S: AsRef<str>>(path: S) -> Result<impl Reply, Rejection> {
    serve(path.as_ref())
}

/// Serve a file at the given path
fn serve(path: &str) -> Result<impl Reply, Rejection> {
    // Find the file or return 404
    let (asset, mime) = if let Some(a) = Assets::get(path) {
        (a, mime_guess::from_path(path).first_or_octet_stream())
    } else {
        let a = Assets::get("404.html").ok_or_else(|| {
            to_rejection(Error::custom_with_log(
                "expected file not found",
                StatusCode::INTERNAL_SERVER_ERROR,
                "could not find file '404.html'".into(),
            ))
        })?;
        (a, mime_guess::from_ext("html").first_or_octet_stream())
    };

    // Build the response
    let mut res = Response::new(asset.into());
    res.headers_mut().insert(
        "Content-Type",
        HeaderValue::from_str(mime.as_ref()).unwrap(),
    );
    Ok(res)
}
