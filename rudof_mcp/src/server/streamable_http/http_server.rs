use std::net::{IpAddr, SocketAddr, ToSocketAddrs};
use std::sync::Arc;

use anyhow::{Context, Result};
use axum::{Router, routing::any_service};
use rmcp::transport::streamable_http_server::{
    StreamableHttpService, session::local::LocalSessionManager, tower::StreamableHttpServerConfig,
};
use tokio_util::sync::CancellationToken;

use crate::server::server_impl::{DEFAULT_ALLOWED_ORIGINS, McpConfig};
use crate::service::RudofMcpService;

/// Run MCP server using the Streamable HTTP transport.
/// This transport is ideal for:
/// - Web-based MCP clients
/// - Remote connections over HTTP/HTTPS
/// - Multiple concurrent client connections
/// - Network-based integrations
///
/// # Security
/// rmcp performs the checks the MCP specification requires on every request:
/// `Origin` validation (403 when present and not allowed), `Host` validation
/// against DNS rebinding (loopback by default), `MCP-Protocol-Version`
/// validation against the versions this server supports (400 when unsupported),
/// and session routing including explicit `DELETE` termination. They are
/// configured here.
///
/// # Note on CORS
/// CORS is not configured here. If you need CORS for browser-based clients,
/// configure it in a reverse proxy (Nginx, Caddy, etc.) for better flexibility.
pub async fn run_mcp_http(config: &McpConfig) -> Result<()> {
    let socket_addr = resolve_socket_addr(&config.bind_address, config.port)?;

    let allowed_origins = if config.allowed_origins.is_empty() {
        DEFAULT_ALLOWED_ORIGINS
            .iter()
            .map(|origin| (*origin).to_string())
            .collect()
    } else {
        config.allowed_origins.clone()
    };
    let allowed_hosts = allowed_hosts(config);

    tracing::debug!(?allowed_origins, ?allowed_hosts, "configured streamable-http guards");

    let cancellation_token = CancellationToken::new();
    let http_config = StreamableHttpServerConfig::default()
        .with_allowed_origins(allowed_origins)
        .with_allowed_hosts(allowed_hosts)
        .with_cancellation_token(cancellation_token.clone());

    let rmcp_service = StreamableHttpService::new(
        || Ok(RudofMcpService::new()),
        Arc::new(LocalSessionManager::default()),
        http_config,
    );

    let router = Router::new().route(&config.route_path, any_service(rmcp_service));

    let listener = std::net::TcpListener::bind(socket_addr)
        .with_context(|| format!("failed to bind the MCP HTTP server to {socket_addr}"))?;
    let server = axum_server::Server::from_tcp(listener).serve(router.into_make_service());

    tracing::info!(
        "MCP HTTP server listening on {}",
        canonical_uri(socket_addr, &config.route_path)
    );

    tokio::select! {
        result = server => {
            if let Err(e) = result {
                tracing::error!("Server error: {}", e);
            }
        }
        _ = tokio::signal::ctrl_c() => {
            tracing::debug!("Shutdown signal received, terminating active sessions...");
            cancellation_token.cancel();
        }
    }

    Ok(())
}

/// The `Host` authorities to accept, on top of rmcp's loopback default.
///
/// A bind address that is a concrete address is added, since that is how
/// clients will reach the server. A wildcard bind cannot be resolved to the
/// authority clients will use, so it needs `--allowed-host`.
fn allowed_hosts(config: &McpConfig) -> Vec<String> {
    let mut hosts: Vec<String> = StreamableHttpServerConfig::default().allowed_hosts;
    hosts.extend(config.allowed_hosts.iter().cloned());

    match config.bind_address.parse::<IpAddr>() {
        Ok(address) if address.is_unspecified() => {
            if config.allowed_hosts.is_empty() {
                tracing::warn!(
                    bind_address = %config.bind_address,
                    "binding to all interfaces, but only loopback Host headers are accepted; \
                     pass --allowed-host for the name or address clients will use"
                );
            }
        },
        _ => hosts.push(config.bind_address.clone()),
    }

    hosts.sort_unstable();
    hosts.dedup();
    hosts
}

/// Resolve the bind address and port into a socket address.
fn resolve_socket_addr(address: &str, port: u16) -> Result<SocketAddr> {
    if let Ok(ip) = address.parse::<IpAddr>() {
        return Ok(SocketAddr::new(ip, port));
    }

    (address, port)
        .to_socket_addrs()
        .with_context(|| format!("failed to resolve the bind address '{address}'"))?
        .next()
        .with_context(|| format!("the bind address '{address}' resolved to no socket address"))
}

/// The URI to show the user, with a wildcard bind reported as loopback.
fn canonical_uri(socket_addr: SocketAddr, route_path: &str) -> String {
    let reachable = if socket_addr.ip().is_unspecified() {
        match socket_addr.ip() {
            IpAddr::V4(_) => SocketAddr::new(IpAddr::V4(std::net::Ipv4Addr::LOCALHOST), socket_addr.port()),
            IpAddr::V6(_) => SocketAddr::new(IpAddr::V6(std::net::Ipv6Addr::LOCALHOST), socket_addr.port()),
        }
    } else {
        socket_addr
    };

    format!("http://{reachable}{route_path}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn config_with(bind_address: &str, allowed_hosts: &[&str]) -> McpConfig {
        McpConfig {
            bind_address: bind_address.to_string(),
            allowed_hosts: allowed_hosts.iter().map(|host| (*host).to_string()).collect(),
            ..McpConfig::default()
        }
    }

    #[test]
    fn loopback_hosts_are_always_accepted() {
        let hosts = allowed_hosts(&config_with("127.0.0.1", &[]));

        assert!(hosts.contains(&"localhost".to_string()));
        assert!(hosts.contains(&"127.0.0.1".to_string()));
        assert!(hosts.contains(&"::1".to_string()));
    }

    #[test]
    fn a_concrete_bind_address_is_accepted_as_a_host() {
        let hosts = allowed_hosts(&config_with("10.0.0.5", &[]));

        assert!(hosts.contains(&"10.0.0.5".to_string()));
    }

    #[test]
    fn a_wildcard_bind_address_is_not_accepted_as_a_host() {
        let hosts = allowed_hosts(&config_with("0.0.0.0", &["example.com"]));

        assert!(!hosts.contains(&"0.0.0.0".to_string()));
        assert!(hosts.contains(&"example.com".to_string()));
    }

    #[test]
    fn ipv6_bind_addresses_resolve_without_manual_bracketing() {
        let socket_addr = resolve_socket_addr("::1", 8000).expect("::1 should resolve");

        assert_eq!(socket_addr.to_string(), "[::1]:8000");
        assert_eq!(canonical_uri(socket_addr, "/rudof"), "http://[::1]:8000/rudof");
    }

    #[test]
    fn a_wildcard_bind_is_reported_as_loopback() {
        let socket_addr = resolve_socket_addr("0.0.0.0", 8080).expect("0.0.0.0 should resolve");

        assert_eq!(canonical_uri(socket_addr, "/rudof"), "http://127.0.0.1:8080/rudof");
    }
}
