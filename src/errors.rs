use diesel::result::{DatabaseErrorKind, Error as DieselError};
use r2d2::Error as R2D2Error;
use serde::Serialize;
use std::{convert::Infallible, fmt};
use warp::{
    http::StatusCode,
    reject,
    reply::{self, Reply},
    Rejection,
};

/// Reject an error that can be converted to an API error
pub fn to_rejection<T: Into<ApiError>>(e: T) -> Rejection {
    reject::custom(e.into())
}

/// Return responses for unhandled errors
pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    if err.is_not_found() {
        Ok(ApiError::new("not found".into(), StatusCode::NOT_FOUND).to_http())
    } else if let Some(e) = err.find::<ApiError>() {
        Ok(e.to_http())
    } else {
        eprintln!("unhandled internal error: {:?}", err);
        Ok(ApiError::new(
            "internal server error".into(),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .to_http())
    }
}

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
    pub fn to_http(&self) -> impl Reply {
        reply::with_status(reply::json(self), self.status_code)
    }
}

impl std::error::Error for ApiError {}
impl warp::reject::Reject for ApiError {}

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
