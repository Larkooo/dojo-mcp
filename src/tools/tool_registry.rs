use std::collections::HashMap;
use async_mcp::types::CallToolResponse;
use async_mcp::server::ServerBuilder;
use async_mcp::transport::Transport;
use tracing::{info, debug};
use super::ToolImplementation;

pub(crate) const LOG_TARGET: &str = "tool_registry";

/// Registry for all available tools
pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn ToolImplementation>>,
}

impl ToolRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        info!(target: LOG_TARGET, "Tool registry initialized.");
        Self {
            tools: HashMap::new(),
        }
    }
    
    /// Register a tool
    pub fn register<T: ToolImplementation>(&mut self, tool: T) {
        let name = tool.name();
        debug!("Registering tool: {}", name);
        self.tools.insert(name, Box::new(tool));
    }
    
    /// Register all tools with the server
    pub fn register_with_server<T: Transport>(self, server: &mut ServerBuilder<T>) {
        for (name, tool_impl) in self.tools {
            let tool = tool_impl.to_tool();
            
            info!(
                target: LOG_TARGET,
                name = %name.clone(),
                "Registering tool with server."
            );
            
            server.register_tool(tool, move |request| {
                info!(
                    target: LOG_TARGET,
                    name = %name.clone(),
                    "Executed tool."
                );
                if request.name == name {
                    tool_impl.execute(request)
                } else {
                    // This shouldn't happen due to how the server routes requests
                    info!("Tool name mismatch: expected {}, got {}", name, request.name);
                    let error_response = CallToolResponse {
                        content: vec![async_mcp::types::ToolResponseContent::Text { 
                            text: format!("Tool name mismatch: expected {}, got {}", name, request.name) 
                        }],
                        is_error: Some(true),
                        meta: None,
                    };
                    Box::pin(async move { Ok(error_response) })
                }
            });
        }
        
        info!("Finished registering all tools with server");
    }
} 