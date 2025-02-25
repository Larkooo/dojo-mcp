mod example_tool;

use crate::tools::tool_registry::ToolRegistry;

/// Register all available tools
pub fn register_all_tools(registry: &mut ToolRegistry) {
    registry.register(example_tool::ExampleTool::new());
}
