use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::{json, Value};

#[derive(Debug)]
pub enum ErrorHandler {
    /// A client-sent request was invalid.
    InvalidRequest(String),
    /// An internal processing error occurred.
    ProcessingError(String),
}

impl IntoResponse for ErrorHandler {
    fn into_response(self) -> Response {
        let (status, message): (StatusCode, String) = match self {
            ErrorHandler::InvalidRequest(msg)  => (StatusCode::BAD_REQUEST, msg),
            ErrorHandler::ProcessingError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body: Json<Value> = Json(json!({
            "status":  status.as_u16(),
            "message": message,
        }));

        (status, body).into_response()
    }
}

/// Type alias for handler return types.
pub type ResultHandler<T> = Result<T, ErrorHandler>;
