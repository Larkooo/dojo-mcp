use std::future::Future;
use std::pin::Pin;
use anyhow::Result;
use async_mcp::types::{CallToolRequest, CallToolResponse, ToolResponseContent};
use serde_json::{json, Value};

use crate::tools::ToolImplementation;

pub struct ExampleTool;

impl ExampleTool {
    pub fn new() -> Self {
        Self
    }
}

impl ToolImplementation for ExampleTool {
    fn name(&self) -> String {
        "example_tool".to_string()
    }
    
    fn description(&self) -> Option<String> {
        Some("An example tool that demonstrates the tool interface".to_string())
    }
    
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "message": {
                    "type": "string",
                    "description": "A message to echo back"
                }
            },
            "required": ["message"]
        })
    }
    
    fn execute(&self, request: CallToolRequest) -> Pin<Box<dyn Future<Output = Result<CallToolResponse>> + Send>> {
        Box::pin(async move {
            let message = match &request.arguments {
                Some(args) => args.get("message")
                    .and_then(|v| v.as_str())
                    .unwrap_or("No message provided"),
                None => "No arguments provided",
            };
            
            Ok(CallToolResponse {
                content: vec![ToolResponseContent::Text { 
                    text: format!("Echo: {}", message) 
                }],
                is_error: None,
                meta: None,
            })
        })
    }
} 