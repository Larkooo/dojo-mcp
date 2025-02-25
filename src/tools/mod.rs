pub mod tool_registry;
pub mod implementations;

use async_mcp::types::{Tool, CallToolRequest, CallToolResponse};
use std::future::Future;
use std::pin::Pin;
use anyhow::Result;

/// Trait that all tool implementations must implement
pub trait ToolImplementation: Send + Sync + 'static {
    /// Get the name of the tool
    fn name(&self) -> String;
    
    /// Get the description of the tool (optional)
    fn description(&self) -> Option<String>;
    
    /// Get the input schema for the tool
    fn input_schema(&self) -> serde_json::Value;
    
    /// Create the Tool struct for registration
    fn to_tool(&self) -> Tool {
        Tool {
            name: self.name(),
            description: self.description(),
            input_schema: self.input_schema(),
        }
    }
    
    /// Execute the tool with the given request
    fn execute(&self, request: CallToolRequest) -> Pin<Box<dyn Future<Output = Result<CallToolResponse>> + Send>>;
} 