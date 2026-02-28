use async_trait::async_trait;
use serde_json::{Value, json};

use crate::protocol::{CallToolResult, Content, ToolDefinition};
use crate::tool::Tool;

pub struct Add;

#[async_trait]
impl Tool for Add {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name:         "add".into(),
            description:  "Adds two numbers together and returns the sum.".into(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "a": { "type": "number", "description": "First number"  },
                    "b": { "type": "number", "description": "Second number" }
                },
                "required": ["a", "b"]
            }),
        }
    }

    async fn call(&self, args: Option<Value>) -> CallToolResult {
        let params = args.as_ref();
        let a:   f64 = params.and_then(|p| p["a"].as_f64()).unwrap_or(0.0);
        let b:   f64 = params.and_then(|p| p["b"].as_f64()).unwrap_or(0.0);
        let sum: f64 = a + b;
        CallToolResult::ok(vec![Content::text(sum.to_string())])
    }
}
