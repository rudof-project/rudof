use std::sync::Arc;

use anyhow::Result;
use rmcp::transport::streamable_http_server::{
    StreamableHttpService, session::local::LocalSessionManager,
};

use crate::rudof_mcp_service::RudofMcpService;

pub struct ServerConfig {
    pub route_name: String,
    pub host: String,
    pub port: u16,
}

impl ServerConfig {
    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

// Run the MCP server. This function centralizes transport/session wiring so the rest of the crate focuses on service logic.
#[tokio::main]
pub async fn run_mcp(route_name: &str, port: &str, host: &str) -> Result<()> {
    let port = port.parse::<u16>()?;
    let cfg = ServerConfig {
        route_name: route_name.to_string(),
        host: host.to_string(),
        port,
    };

    tracing::info!(
        "Starting MCP server at {}/{}",
        cfg.bind_address(),
        cfg.route_name
    );

    let service = StreamableHttpService::new(
        || Ok(RudofMcpService::new()),
        Arc::new(LocalSessionManager::default()),
        Default::default(),
    );

    let router = axum::Router::new().nest_service(&format!("/{}", cfg.route_name), service);
    let tcp_listener = tokio::net::TcpListener::bind(cfg.bind_address()).await?;
    let _ = axum::serve(tcp_listener, router)
        .with_graceful_shutdown(async { tokio::signal::ctrl_c().await.unwrap() })
        .await;
    Ok(())
}
