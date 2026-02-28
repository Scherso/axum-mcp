use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Incoming.
#[derive(Debug, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id:      Option<Value>,
    pub method:  String,
    #[serde(default)]
    pub params:  Option<Value>,
}

/// Outgoing.
#[derive(Debug, Serialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id:      Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result:  Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error:   Option<JsonRpcError>,
}

#[derive(Debug, Serialize)]
pub struct JsonRpcError {
    pub code:    i64,
    pub message: String,
}

impl JsonRpcResponse {
    pub fn success(
        id:     Option<Value>, 
        result: Value
    ) -> Self {
        Self { 
            jsonrpc: "2.0", 
            id, 
            result: Some(result), 
            error:  None 
        }
    }

    pub fn error(id: 
        Option<Value>, 
        code:    i64, 
        message: impl Into<String>
    ) -> Self {
        Self {
            jsonrpc: "2.0",
            id,
            result:  None,
            error:   Some(JsonRpcError { 
                code, 
                message: message.into() 
            }),
        }
    }

    /// "Method not found" Error.
    /// This "magic number" here is the code for "Method not found",
    /// where the method does not exist / is not available.
    /// See https://www.jsonrpc.org/specification#error_object
    pub fn method_not_found(id: Option<Value>) -> Self {
        Self::error(id, -32601, "Method not found")
    }
}

/// Describes a tool that can be listed and called via MCP.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub name:         String,
    pub description:  String,
    #[serde(rename = "inputSchema")]
    pub input_schema: Value,
}

/// A single piece of content returned by a tool call.
#[derive(Debug, Clone, Serialize)]
pub struct Content {
    #[serde(rename = "type")]
    pub kind: &'static str,
    pub text: String,
}

impl Content {
    pub fn text(text: impl Into<String>) -> Self {
        Self { kind: "text", text: text.into() }
    }
}

/// The result returned when a tool is called.
#[derive(Debug, Clone, Serialize)]
pub struct CallToolResult {
    pub content:  Vec<Content>,
    #[serde(rename = "isError", skip_serializing_if = "std::ops::Not::not")]
    pub is_error: bool,
}

impl CallToolResult {
    pub fn ok(content: Vec<Content>) -> Self {
        Self { 
            content, 
            is_error: false 
        }
    }

    pub fn err(message: impl Into<String>) -> Self {
        Self {
            content:  vec![Content::text(message)],
            is_error: true,
        }
    }
}
