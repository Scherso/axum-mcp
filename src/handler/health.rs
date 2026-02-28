use axum::{
    Json,
    http::StatusCode,
};
use serde_json::{json, Value};

use crate::constant;
use crate::result::ResultHandler;

/// Health check endpoint.
///
/// # Returns
/// * `Json<Value>`: A JSON object containing the
///                  health status,
///                  service name,
///                  version.
pub async fn health_check() -> ResultHandler<Json<Value>> {
    Ok(Json(json!({
        "status":  StatusCode::OK.as_u16(),
        "service": constant::SERVICE_NAME,
        "version": constant::VERSION,
    })))
}
