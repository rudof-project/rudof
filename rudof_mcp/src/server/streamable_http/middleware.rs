use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::Response,
};
use ipnetwork::IpNetwork;
use std::{net::IpAddr, str::FromStr, sync::Arc};

/// Header name for MCP protocol version
const PROTOCOL_HEADER: &str = "MCP-Protocol-Version";

/// Supported MCP protocol versions per specification 2025-11-25
const SUPPORTED_PROTOCOL_VERSIONS: &[&str] = &["2025-11-25", "2025-06-18", "2025-03-26"];

/// Default protocol version when header is absent (per spec)
const DEFAULT_PROTOCOL_VERSION: &str = "2025-03-26";

/// Configuration for allowed origins with IP/network-based validation
#[derive(Clone)]
pub struct OriginConfig {
    /// List of allowed IP networks (CIDR notation supported)
    allowed_networks: Arc<Vec<IpNetwork>>,
    /// Whether to allow requests without Origin header (non-browser clients)
    allow_missing_origin: bool,
}

impl OriginConfig {
    /// Creates a new OriginConfig with the specified allowed networks
    ///
    /// # Arguments
    /// * `allowed_cidrs` - List of IP addresses or CIDR ranges (e.g., "127.0.0.1", "192.168.1.0/24", "::1")
    pub fn new(allowed_cidrs: Vec<String>) -> Result<Self, String> {
        let mut networks = Vec::new();

        for cidr in allowed_cidrs {
            let network = IpNetwork::from_str(&cidr)
                .map_err(|e| format!("Invalid CIDR notation '{}': {}", cidr, e))?;
            networks.push(network);
        }

        if networks.is_empty() {
            return Err("At least one allowed network must be specified".to_string());
        }

        Ok(Self {
            allowed_networks: Arc::new(networks),
            allow_missing_origin: true,
        })
    }

    /// Creates a localhost-only configuration (default behavior)
    pub fn localhost_only() -> Self {
        Self {
            allowed_networks: Arc::new(vec![
                IpNetwork::from_str("127.0.0.0/8").unwrap(), // IPv4 localhost
                IpNetwork::from_str("::1/128").unwrap(),     // IPv6 localhost
            ]),
            allow_missing_origin: true,
        }
    }

    /// Sets whether to allow requests without Origin header
    #[allow(dead_code)]
    pub fn allow_missing_origin(mut self, allow: bool) -> Self {
        self.allow_missing_origin = allow;
        self
    }

    /// Checks if an origin URL is allowed based on the configured networks
    pub fn is_allowed_origin(&self, origin: &str) -> bool {
        // Parse the origin URL to extract the host
        let host = match Self::extract_host_from_origin(origin) {
            Some(h) => h,
            None => return false,
        };

        // Try to parse as IP address
        if let Ok(ip_addr) = IpAddr::from_str(host) {
            return self.is_ip_allowed(&ip_addr);
        }

        // Handle hostname (localhost)
        if host == "localhost" {
            // Check if any localhost IP is in the allowed networks
            let localhost_v4 = IpAddr::from_str("127.0.0.1").unwrap();
            let localhost_v6 = IpAddr::from_str("::1").unwrap();
            return self.is_ip_allowed(&localhost_v4) || self.is_ip_allowed(&localhost_v6);
        }

        false
    }

    /// Checks if an IP address is within any of the allowed networks
    fn is_ip_allowed(&self, ip: &IpAddr) -> bool {
        self.allowed_networks
            .iter()
            .any(|network| network.contains(*ip))
    }

    /// Extracts the host portion from an origin URL
    /// Returns the host without protocol, port, or path
    fn extract_host_from_origin(origin: &str) -> Option<&str> {
        // Remove protocol
        let without_protocol = origin
            .strip_prefix("https://")
            .or_else(|| origin.strip_prefix("http://"))
            .unwrap_or(origin);

        // Handle IPv6 addresses in brackets
        if let Some(start) = without_protocol.find('[')
            && let Some(end) = without_protocol.find(']')
        {
            return Some(&without_protocol[start + 1..end]);
        }

        // Split by : or / to remove port and path
        let host_end = without_protocol
            .find(':')
            .or_else(|| without_protocol.find('/'))
            .unwrap_or(without_protocol.len());

        Some(&without_protocol[..host_end])
    }
}

/// Checks if a protocol version is supported
#[allow(dead_code)]
pub fn is_valid_protocol_version(version: &str) -> bool {
    SUPPORTED_PROTOCOL_VERSIONS.contains(&version)
}

/// Applies all standard MCP middleware layers to the given router.
/// This includes:
/// - MCP protocol version validation (`MCP-Protocol-Version`)
/// - Origin header validation with configurable IP/network filtering
pub fn with_guards(router: Router, origin_config: OriginConfig) -> Router {
    router
        .layer(middleware::from_fn(protocol_version_guard))
        .layer(middleware::from_fn(move |req, next| {
            origin_guard(req, next, origin_config.clone())
        }))
}

/// Validates the MCP-Protocol-Version header.
///
/// According to the MCP specification (2025-11-25):
/// - Clients MUST include `MCP-Protocol-Version: <protocol-version>` on all HTTP requests
/// - The protocol version sent SHOULD be the one negotiated during initialization
/// - If the server does not receive an `MCP-Protocol-Version` header, it SHOULD assume
///   protocol version 2025-03-26 for backwards compatibility
/// - If the server receives an invalid or unsupported version, it MUST respond with 400 Bad Request
pub async fn protocol_version_guard(req: Request<Body>, next: Next) -> Response {
    if let Some(v) = req.headers().get(PROTOCOL_HEADER) {
        match v.to_str() {
            Ok(s) if SUPPORTED_PROTOCOL_VERSIONS.contains(&s) => {
                tracing::debug!("Accepted MCP-Protocol-Version: {}", s);
                // OK — continue
            }
            Ok(s) => {
                tracing::warn!("Unsupported MCP-Protocol-Version: {}", s);
                return Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(
                        format!(
                            "unsupported MCP-Protocol-Version: {}. Supported versions: {:?}",
                            s, SUPPORTED_PROTOCOL_VERSIONS
                        )
                        .into(),
                    )
                    .unwrap();
            }
            Err(_) => {
                tracing::warn!("Invalid MCP-Protocol-Version header encoding");
                return Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body("invalid MCP-Protocol-Version header encoding".into())
                    .unwrap();
            }
        }
    } else {
        // No header — per spec, assume 2025-03-26 for backwards compatibility
        tracing::debug!(
            "No MCP-Protocol-Version header; assuming {}",
            DEFAULT_PROTOCOL_VERSION
        );
    }

    next.run(req).await
}

/// Validates the Origin header against configured allowed IP networks.
///
/// According to the MCP specification (2025-11-25):
/// - Servers MUST validate the Origin header on all incoming connections
/// - If the Origin header is present and invalid, servers MUST respond with HTTP 403 Forbidden
/// - When running locally, servers SHOULD bind only to localhost (127.0.0.1)
///
/// This implementation extends the spec by allowing configurable IP networks.
///
/// Note: Requests without an Origin header are allowed by default, as these typically come from:
/// - Non-browser HTTP clients (curl, Postman, CLI tools)
/// - Same-origin requests in some browsers
/// - Server-to-server communication
pub async fn origin_guard(req: Request<Body>, next: Next, config: OriginConfig) -> Response {
    if let Some(origin) = req.headers().get("Origin") {
        match origin.to_str() {
            Ok(origin_str) if config.is_allowed_origin(origin_str) => {
                tracing::debug!("Accepted Origin header: {}", origin_str);
                // Allowed origin
            }
            Ok(origin_str) => {
                // Per MCP spec: "If the Origin header is present and invalid,
                // servers MUST respond with HTTP 403 Forbidden"
                tracing::warn!(
                    "Rejected request due to non-allowed Origin header: {}",
                    origin_str
                );
                return Response::builder()
                    .status(StatusCode::FORBIDDEN)
                    .body("origin not allowed - origin IP not in allowed networks".into())
                    .unwrap();
            }
            Err(_) => {
                tracing::warn!("Rejected request due to invalid Origin header encoding");
                return Response::builder()
                    .status(StatusCode::FORBIDDEN)
                    .body("invalid Origin header encoding".into())
                    .unwrap();
            }
        }
    } else if !config.allow_missing_origin {
        tracing::warn!("Rejected request due to missing Origin header");
        return Response::builder()
            .status(StatusCode::FORBIDDEN)
            .body("missing Origin header".into())
            .unwrap();
    } else {
        // No Origin header - allowed per config
        tracing::trace!("No Origin header present (non-browser client)");
    }

    next.run(req).await
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that supported protocol versions should be accepted
    #[test]
    fn test_accepts_supported_protocol_version() {
        let supported_versions = vec!["2025-11-25", "2025-06-18", "2025-03-26"];

        for version in supported_versions {
            assert!(
                is_valid_protocol_version(version),
                "Version {} should be valid",
                version
            );
        }
    }

    /// Test that unsupported protocol versions should be rejected
    #[test]
    fn test_rejects_unsupported_protocol_version() {
        let unsupported_versions = vec![
            "2020-01-01",
            "1.0",
            "invalid",
            "",
            "2024-01-01",
            "2025-01-01",
        ];

        for version in unsupported_versions {
            assert!(
                !is_valid_protocol_version(version),
                "Version {} should be invalid",
                version
            );
        }
    }

    #[test]
    fn test_localhost_config() {
        let config = OriginConfig::localhost_only();

        // Test localhost variations
        assert!(config.is_allowed_origin("http://localhost"));
        assert!(config.is_allowed_origin("https://localhost:8080"));
        assert!(config.is_allowed_origin("http://127.0.0.1"));
        assert!(config.is_allowed_origin("https://127.0.0.1:3000"));
        assert!(config.is_allowed_origin("http://[::1]"));
        assert!(config.is_allowed_origin("https://[::1]:8080"));

        // Test non-localhost
        assert!(!config.is_allowed_origin("http://192.168.1.1"));
        assert!(!config.is_allowed_origin("https://example.com"));
    }

    #[test]
    fn test_custom_networks() {
        let config =
            OriginConfig::new(vec!["192.168.1.0/24".to_string(), "10.0.0.0/8".to_string()])
                .unwrap();

        // Test allowed networks
        assert!(config.is_allowed_origin("http://192.168.1.1"));
        assert!(config.is_allowed_origin("https://192.168.1.254"));
        assert!(config.is_allowed_origin("http://10.0.0.1"));
        assert!(config.is_allowed_origin("http://10.255.255.255"));

        // Test disallowed
        assert!(!config.is_allowed_origin("http://192.168.2.1"));
        assert!(!config.is_allowed_origin("http://127.0.0.1"));
        assert!(!config.is_allowed_origin("http://localhost"));
    }
}
