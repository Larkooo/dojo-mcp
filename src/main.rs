use anyhow::Result;
use async_mcp::{run_http_server, server::Server, types::ServerCapabilities};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        // needs to be stderr due to stdio transport
        .with_writer(std::io::stderr)
        .init();

    run_http_server(4040, None, |transport| async move {
        let server = Server::builder(transport).capabilities(ServerCapabilities {
            tools: Some(json!({})),
            ..Default::default()
        }).build();
        Ok(server)
    })
    .await?;

    Ok(())
}
