use std::collections::HashMap;

use serde_json::Value;

use crate::protocol::{CallToolResult, ToolDefinition};
use crate::tool::Tool;

pub struct McpServer {
    pub name:    String,
    pub version: String,
    tools:       HashMap<String, Box<dyn Tool>>,
}

impl McpServer {
    /// Start building a new server with the given name and version.
    pub fn builder(name: impl Into<String>, version: impl Into<String>) -> McpServerBuilder {
        McpServerBuilder {
            name:    name.into(),
            version: version.into(),
            tools:   HashMap::new(),
        }
    }

    /// List all registered tool definitions.
    pub fn list_tools(&self) -> Vec<ToolDefinition> {
        self.tools.values().map(|t| t.definition()).collect()
    }

    /// Call a tool by name.
    pub async fn call_tool(&self, name: &str, args: Option<Value>) -> CallToolResult {
        match self.tools.get(name) {
            Some(tool) => tool.call(args).await,
            None       => CallToolResult::err(format!("Unknown tool: {name}")),
        }
    }
}

/// Builder for [`McpServer`].
pub struct McpServerBuilder {
    name:    String,
    version: String,
    tools:   HashMap<String, Box<dyn Tool>>,
}

impl McpServerBuilder {
    pub fn tool(mut self, tool: impl Tool) -> Self {
        let name: String = tool.definition().name.clone();
        self.tools.insert(name, Box::new(tool));
        self
    }

    pub fn build(self) -> McpServer {
        McpServer {
            name:    self.name,
            version: self.version,
            tools:   self.tools,
        }
    }
}
