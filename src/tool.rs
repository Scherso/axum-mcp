use async_trait::async_trait;
use serde_json::Value;

use crate::protocol::{CallToolResult, ToolDefinition};

/// Implement this trait once per tool, then register it with
/// [`McpServer::builder`](crate::server::McpServer::builder).
#[async_trait]
pub trait Tool: Send + Sync + 'static {
    /// JSON Schema description surfaced via `tools/list`.
    fn definition(&self) -> ToolDefinition;

    /// Execute the tool with the given arguments.
    async fn call(&self, args: Option<Value>) -> CallToolResult;
}
