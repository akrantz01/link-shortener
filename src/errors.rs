use diesel::result::{DatabaseErrorKind, Error as DieselError};
use r2d2::Error as R2D2Error;
use serde::Serialize;
use std::fmt;
use warp::{
    http::StatusCode,
    reply::{json, with_status, Reply},
};

#[derive(Debug, Serialize)]
pub struct ApiError {
    #[serde(skip)]
    status_code: StatusCode,
    success: bool,
    message: String,
}

impl ApiError {
    /// Create a new API error
    pub fn new(message: String, status_code: StatusCode) -> ApiError {
        ApiError {
            status_code,
            message,
            success: status_code.is_success(),
        }
    }

    /// Convert the error into a HTTP response
    pub fn into_http(self) -> impl Reply {
        with_status(json(&self), self.status_code)
    }
}

impl std::error::Error for ApiError {}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.message)
    }
}

impl From<DieselError> for ApiError {
    fn from(error: DieselError) -> ApiError {
        match error {
            DieselError::DatabaseError(kind, err) => match kind {
                DatabaseErrorKind::UniqueViolation => {
                    ApiError::new(err.message().to_string(), StatusCode::CONFLICT)
                }
                DatabaseErrorKind::ForeignKeyViolation => ApiError::new(
                    format!("foreign key constraint violated: {}", err.message()),
                    StatusCode::INTERNAL_SERVER_ERROR,
                ),
                _ => ApiError::new(
                    format!("an unexpected database error occurred: {}", err.message()),
                    StatusCode::INTERNAL_SERVER_ERROR,
                ),
            },
            DieselError::NotFound => {
                ApiError::new("record not found".to_string(), StatusCode::NOT_FOUND)
            }
            err => ApiError::new(
                format!("a database error occurred: {}", err),
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
        }
    }
}

impl From<R2D2Error> for ApiError {
    fn from(error: R2D2Error) -> ApiError {
        ApiError::new(format!("{}", error), StatusCode::INTERNAL_SERVER_ERROR)
    }
}
