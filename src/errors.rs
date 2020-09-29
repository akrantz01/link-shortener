use diesel::result::{DatabaseErrorKind, Error as DieselError};
use r2d2::Error as R2D2Error;
use std::{convert::Infallible, error::Error as StdError};
use warp::{
    http::StatusCode,
    reject::{self, Rejection},
    reply::{self, Reply},
};

/// Reject an error that can be converted to an API error
pub fn to_rejection<T: Into<Error>>(e: T) -> Rejection {
    reject::custom(e.into())
}

/// Return responses for unhandled errors
pub async fn handle_rejection(
    err: Rejection,
) -> Result<reply::WithStatus<Box<dyn Reply>>, Infallible> {
    // Convert error to JSON
    if let Some(e) = err.find::<Error>() {
        Ok(e.to_http())

    // Payload too large
    } else if let Some(_) = err.find::<reject::PayloadTooLarge>() {
        Ok(Error::custom("payload too large", StatusCode::PAYLOAD_TOO_LARGE).to_http())

    // Body deserialization error
    } else if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
        Ok(Error::custom(format!("{}", e.source().unwrap()), StatusCode::BAD_REQUEST).to_http())

    // Method not allowed
    } else if let Some(_) = err.find::<reject::MethodNotAllowed>() {
        Ok(Error::custom("method not allowed", StatusCode::METHOD_NOT_ALLOWED).to_http())

    // Unknown error
    } else {
        Ok(Error::custom_with_log(
            "internal server error",
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("unhandled error: {:?}", err),
        )
        .to_http())
    }
}

#[derive(Debug)]
enum ErrorKind {
    Database,
    UniqueViolation,
    InvalidLink,
    Authentication(String),
    Custom(String),
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    status_code: StatusCode,
    log_message: Option<String>,
}

impl Error {
    /// Create an authentication error that will prompt the user for a password
    pub fn authentication<S: Into<String>>(realm: S) -> Self {
        Self {
            kind: ErrorKind::Authentication(realm.into()),
            status_code: StatusCode::UNAUTHORIZED,
            log_message: None,
        }
    }

    /// Create an arbitrary database error
    fn database<S: Into<String>>(message: S) -> Self {
        Self {
            kind: ErrorKind::Database,
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            log_message: Some(message.into()),
        }
    }

    /// Create an error from a unique constraint being violated
    fn unique_violation() -> Self {
        Self {
            kind: ErrorKind::UniqueViolation,
            status_code: StatusCode::CONFLICT,
            log_message: None,
        }
    }

    /// Create a message from an invalid link
    pub fn invalid_link() -> Self {
        Self {
            kind: ErrorKind::InvalidLink,
            status_code: StatusCode::BAD_REQUEST,
            log_message: None,
        }
    }

    /// Create a custom error with a message
    pub fn custom<S: Into<String>>(message: S, status_code: StatusCode) -> Self {
        Self {
            kind: ErrorKind::Custom(message.into()),
            status_code,
            log_message: None,
        }
    }

    /// Create a custom error with a message to be logged a the ERROR level
    pub fn custom_with_log<S: Into<String>>(
        message: S,
        status_code: StatusCode,
        log_message: String,
    ) -> Self {
        Self {
            kind: ErrorKind::Custom(message.into()),
            status_code,
            log_message: Some(log_message),
        }
    }

    /// Convert the error to a HTTP response
    fn to_http(&self) -> reply::WithStatus<Box<dyn Reply>> {
        // Log message if necessary
        if let Some(log) = &self.log_message {
            error!("{}", log);
        }

        // Assemble the response body
        let body: Box<dyn Reply> = match &self.kind {
            ErrorKind::Database => Box::new(reply::json(
                &json!({ "success": false, "message": "a database error occurred" }),
            )),
            ErrorKind::UniqueViolation => Box::new(reply::json(
                &json!({ "success": false, "message": "a field was not unique" }),
            )),
            ErrorKind::InvalidLink => Box::new(reply::json(
                &json!({ "success": false, "message": "the provided link was invalid" }),
            )),
            ErrorKind::Authentication(realm) => {
                let body = reply::html(format!("401 Unauthorized: {}", realm));
                let header = reply::with_header(
                    body,
                    "WWW-Authenticate",
                    format!("Basic realm=\"{}\", charset=\"UTF-8\"", realm),
                );
                Box::new(header)
            }
            ErrorKind::Custom(message) => Box::new(reply::json(
                &json!({ "success": false, "message": message }),
            )),
        };

        // Attach the status code
        reply::with_status(body, self.status_code)
    }
}

impl reject::Reject for Error {}

impl From<DieselError> for Error {
    fn from(error: DieselError) -> Error {
        match error {
            DieselError::DatabaseError(kind, err) => match kind {
                DatabaseErrorKind::UniqueViolation => Error::unique_violation(),
                _ => Error::database(err.message()),
            },
            err => Error::database(err.to_string()),
        }
    }
}

impl From<R2D2Error> for Error {
    fn from(error: R2D2Error) -> Error {
        Error::database(error.to_string())
    }
}
