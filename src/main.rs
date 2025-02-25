use anyhow::Result;
use async_mcp::{run_http_server, server::Server, types::ServerCapabilities};
use serde_json::json;

mod tools;
use tools::implementations::register_all_tools;
use tools::tool_registry::ToolRegistry;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        // needs to be stderr due to stdio transport
        .with_writer(std::io::stderr)
        .init();

    run_http_server(4040, None, |transport| async move {
        // Create and populate the tool registry
        let mut tool_registry = ToolRegistry::new();
        register_all_tools(&mut tool_registry);

        let mut server = Server::builder(transport).capabilities(ServerCapabilities {
            tools: Some(json!({})),
            ..Default::default()
        });

        // Register all tools with the server
        tool_registry.register_with_server(&mut server);

        Ok(server.build())
    })
    .await?;

    Ok(())
}
