use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use nosqo_base::error::NosqoError;

use crate::error_response::ErrorResponse;

/// An HTTP-facing query request failure.
#[derive(Debug)]
pub enum QueryRequestError {
    /// The client submitted an invalid query.
    InvalidQuery(String),
    /// The server failed while executing a valid query.
    Internal(NosqoError),
}

impl IntoResponse for QueryRequestError {
    fn into_response(self) -> Response {
        match self {
            Self::InvalidQuery(message) => {
                (StatusCode::BAD_REQUEST, Json(ErrorResponse::new(message))).into_response()
            }
            Self::Internal(error) => {
                tracing::error!("failed to execute NQL query: {error:?}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse::new("internal server error")),
                )
                    .into_response()
            }
        }
    }
}
