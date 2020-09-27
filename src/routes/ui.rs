use rust_embed::RustEmbed;
use warp::{http::header::HeaderValue, path::Tail, reply::Response, Rejection, Reply};

#[derive(RustEmbed)]
#[folder = "ui"]
struct Assets;

/// Serve index.html
pub async fn serve_index() -> Result<impl Reply, Rejection> {
    serve("index.html")
}

/// Serve an arbitrary file at a given path
pub async fn serve_arbitrary(path: Tail) -> Result<impl Reply, Rejection> {
    serve(path.as_str())
}

/// Serve a file at the given path
fn serve(path: &str) -> Result<impl Reply, Rejection> {
    let asset = Assets::get(path).ok_or_else(warp::reject::not_found)?;
    let mime = mime_guess::from_path(path).first_or_octet_stream();

    let mut res = Response::new(asset.into());
    res.headers_mut().insert(
        "Content-Type",
        HeaderValue::from_str(mime.as_ref()).unwrap(),
    );
    Ok(res)
}
