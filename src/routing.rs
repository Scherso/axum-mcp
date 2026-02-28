use std::sync::Arc;

use axum::{
    Router,
    routing::post,
    routing::get,
};

use crate::constant::{
    MCP_ENDPOINT,
    HEALTH_ENDPOINT,
};
use crate::handler::{
    mcp::handle_rpc,
    health::health_check,
};
use crate::server::McpServer;

/// Creates and configures the main application router.
///
/// Binds:
/// * `/mcp`    to `handler::mcp::handle_rpc`      (POST, JSON-RPC 2.0).
/// * `/health` to `handler::health::health_check` (GET).
pub fn app(server: McpServer) -> Router {
    Router::new()
        .route(MCP_ENDPOINT,    post(handle_rpc))
        .route(HEALTH_ENDPOINT, get(health_check))
        .with_state(Arc::new(server))
}
