use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::Response,
};

const PROTOCOL_HEADER: &str = "MCP-Protocol-Version";

/// Supported MCP protocol versions per specification 2025-11-25
const SUPPORTED_PROTOCOL_VERSIONS: &[&str] = &[
    "2025-11-25",
    "2025-06-18",
    "2025-03-26",
];

/// Default protocol version when header is absent (per spec)
const DEFAULT_PROTOCOL_VERSION: &str = "2025-03-26";

/// Applies all standard MCP middleware layers to the given router.
/// This includes:
/// - MCP protocol version validation (`MCP-Protocol-Version`)
/// - Origin header validation (to prevent DNS rebinding)
pub fn with_guards(router: Router) -> Router {
    router
        .layer(middleware::from_fn(protocol_version_guard))
        .layer(middleware::from_fn(origin_guard))
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

/// Validates the Origin header to prevent DNS rebinding attacks.
///
/// According to the MCP specification (2025-11-25):
/// - Servers MUST validate the Origin header on all incoming connections
/// - If the Origin header is present and invalid, servers MUST respond with HTTP 403 Forbidden
/// - When running locally, servers SHOULD bind only to localhost (127.0.0.1)
///
/// Note: Requests without an Origin header are allowed, as these typically come from:
/// - Non-browser HTTP clients (curl, Postman, CLI tools)
/// - Same-origin requests in some browsers
/// - Server-to-server communication
pub async fn origin_guard(req: Request<Body>, next: Next) -> Response {
    if let Some(origin) = req.headers().get("Origin") {
        match origin.to_str() {
            Ok(origin_str)
                if origin_str.starts_with("http://localhost")
                    || origin_str.starts_with("https://localhost")
                    || origin_str.starts_with("http://127.0.0.1")
                    || origin_str.starts_with("https://127.0.0.1")
                    || origin_str.starts_with("http://[::1]")
                    || origin_str.starts_with("https://[::1]") =>
            {
                tracing::debug!("Accepted Origin header: {}", origin_str);
                // Allowed origin
            }
            Ok(origin_str) => {
                // Per MCP spec: "If the Origin header is present and invalid,
                // servers MUST respond with HTTP 403 Forbidden"
                tracing::warn!(
                    "Rejected request due to non-local Origin header: {}",
                    origin_str
                );
                return Response::builder()
                    .status(StatusCode::FORBIDDEN)
                    .body("origin not allowed - only localhost origins are permitted".into())
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
    } else {
        // No Origin header - allowed per spec (only present + invalid triggers 403)
        tracing::trace!("No Origin header present (non-browser client)");
    }

    next.run(req).await
}
