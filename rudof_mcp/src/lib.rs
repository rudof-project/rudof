pub mod rudof_mcp_service;

use anyhow::Result;
use rmcp::transport::streamable_http_server::{
    StreamableHttpService, session::local::LocalSessionManager,
};

use rudof_mcp_service::RudofMcpService;

#[tokio::main]
pub async fn run_mcp(route_name: &str, port: &str, host: &str) -> Result<()> {
    let port = port.parse::<u16>()?;
    let bind_address = format!("{}:{}", host, port);

    tracing::info!("Starting MCP server at {bind_address}/{route_name}");

    let service = StreamableHttpService::new(
        || Ok(RudofMcpService::new()),
        LocalSessionManager::default().into(),
        Default::default(),
    );

    let router = axum::Router::new().nest_service(&format!("/{route_name}"), service);
    let tcp_listener = tokio::net::TcpListener::bind(bind_address).await?;
    let _ = axum::serve(tcp_listener, router)
        .with_graceful_shutdown(async { tokio::signal::ctrl_c().await.unwrap() })
        .await;
    Ok(())
}
