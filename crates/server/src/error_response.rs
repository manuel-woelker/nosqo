use serde::Serialize;

/// A small JSON error payload for API responses.
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    /// The rendered error message.
    pub error: String,
}

impl ErrorResponse {
    /// Creates an error response payload from a message.
    pub fn new(error: impl Into<String>) -> Self {
        Self {
            error: error.into(),
        }
    }
}
