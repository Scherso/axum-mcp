use async_trait::async_trait;
use serde_json::{Value, json};

use crate::protocol::{CallToolResult, Content, ToolDefinition};
use crate::tool::Tool;

pub struct Echo;

#[async_trait]
impl Tool for Echo {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name:         "echo".into(),
            description:  "Returns its input unchanged.".into(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "message": { "type": "string", "description": "Text to echo" }
                },
                "required": ["message"]
            }),
        }
    }

    async fn call(&self, args: Option<Value>) -> CallToolResult {
        let msg: &str = args
            .as_ref()
            .and_then(|a| a["message"].as_str())
            .unwrap_or("(empty)");
        CallToolResult::ok(vec![Content::text(msg)])
    }
}
