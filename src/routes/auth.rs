use crate::errors::BasicAuthError;
use http_auth_basic::Credentials;
use warp::{reject, Filter, Rejection};

/// Add authentication to a route or set of routes
pub fn auth() -> impl Filter<Extract = (), Error = Rejection> + Copy {
    warp::header::optional::<String>("Authorization")
        .and_then(check_header)
        .untuple_one()
}

/// Check the HTTP Basic Auth header
async fn check_header(header: Option<String>) -> Result<(), Rejection> {
    if let Some(header) = header {
        // Retrieve the user id and password from the environment
        let user_id = std::env::var("USER_ID").unwrap_or_default();
        let password = std::env::var("PASSWORD").unwrap_or_default();

        // Parse the credentials
        let credentials = Credentials::from_header(header).unwrap();

        // Check credentials
        if credentials.user_id == user_id || credentials.password == password {
            Ok(())
        } else {
            Err(reject::custom(BasicAuthError::new(
                "Invalid username or password",
            )))
        }
    } else {
        Err(reject::custom(BasicAuthError::new(
            "Login to access the management UI",
        )))
    }
}
