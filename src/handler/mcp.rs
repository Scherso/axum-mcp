use std::sync::Arc;

use axum::{Json, extract::State};
use serde_json::json;

use crate::constant;
use crate::result::ResultHandler;
use crate::protocol::{JsonRpcRequest, JsonRpcResponse};
use crate::server::McpServer;

/// Handles an incoming MCP JSON-RPC 2.0 request.
///
/// Dispatches to the appropriate MCP method based on
/// the `method` field of the JSON-RPC request.
/// 
/// See https://www.jsonrpc.org/specification#request_object
pub async fn handle_rpc(
    State(server): State<Arc<McpServer>>,
    Json(req): Json<JsonRpcRequest>,
) -> ResultHandler<Json<JsonRpcResponse>> {
    let response: JsonRpcResponse = dispatch(&server, &req).await;
    Ok(Json(response))
}

async fn dispatch(server: &McpServer, req: &JsonRpcRequest) -> JsonRpcResponse {
    match req.method.as_str() {
        "initialize" => {
            let result = json!({
                "protocolVersion": constant::PROTOCOL_VERSION,
                "serverInfo": {
                    "name":    server.name,
                    "version": server.version,
                },
                "capabilities": {
                    "tools": { "listChanged": false },
                },
            });
            JsonRpcResponse::success(req.id.clone(), result)
        }

        "tools/list" => {
            let tools = server.list_tools();
            let result = json!({ "tools": tools });
            JsonRpcResponse::success(req.id.clone(), result)
        }

        "tools/call" => {
            let params = req.params.as_ref();
            let name: &str = params
                .and_then(|p| p["name"].as_str())
                .unwrap_or_default();
            let args = params.and_then(|p| p.get("arguments")).cloned();

            let tool_result = server.call_tool(name, args).await;
            let result = serde_json::to_value(tool_result).unwrap();
            JsonRpcResponse::success(req.id.clone(), result)
        }

        // An MCP client sends notifications to the server but don't expect 
        // "meaningful" responses. 
        // An example of this would be "notifications/initialized" which 
        // the client may send right after the initialize method (handshake)
        // completes to tell the server that it's ready to use MCP.
        // 
        // Since HTTP requires every request to have a response, we send an
        // empty response as an ack.
        method if method.starts_with("notifications/") => {
            JsonRpcResponse::success(req.id.clone(), json!({}))
        }

        _ => JsonRpcResponse::method_not_found(req.id.clone()),
    }
}
