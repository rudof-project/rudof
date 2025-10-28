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

// Aux fn for testing
async fn run_mcp_aux(route_name: &str, port: &str, host: &str) -> Result<()> {
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

// Run the MCP server. This function centralizes transport/session wiring so the rest of the crate focuses on service logic.
#[tokio::main]
pub async fn run_mcp(route_name: &str, port: &str, host: &str) -> Result<()> {
    run_mcp_aux(route_name, port, host).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{Duration, timeout};

    #[tokio::test]
    async fn test_bind_address() {
        let cfg = ServerConfig {
            route_name: "test".to_string(),
            host: "127.0.0.1".to_string(),
            port: 8080,
        };
        assert_eq!(cfg.bind_address(), "127.0.0.1:8080");
    }

    #[tokio::test]
    async fn test_run_mcp_invalid_port() {
        let result = run_mcp_aux("api", "not_a_port", "127.0.0.1").await;
        assert!(
            result.is_err(),
            "Expected run_mcp_aux to fail with invalid port"
        );
    }

    #[tokio::test]
    async fn test_run_mcp_invalid_host() {
        let result = run_mcp_aux("api", "8080", "not_a_host").await;
        assert!(
            result.is_err(),
            "Expected run_mcp_aux to fail with invalid host"
        );
    }

    #[tokio::test]
    async fn test_run_mcp_valid() {
        let route_name = "test_api";
        let host = "127.0.0.1";
        let port = "8000";

        let result = timeout(Duration::from_secs(5), run_mcp_aux(route_name, port, host)).await;

        assert!(
            result.is_err() || result.unwrap().is_ok(),
            "Expected server to start"
        );
    }
}
