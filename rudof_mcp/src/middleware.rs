use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::Response,
    Router,
};
use tracing::warn;

const PROTOCOL_HEADER: &str = "MCP-Protocol-Version";
const SUPPORTED_PROTOCOL_VERSIONS: &[&str] = &[
    "2025-06-18", // Streamable HTTP transport (current)
    "2025-03-26", // Fallback per spec
    "2024-11-05", // Deprecated HTTP+SSE transport
];

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
/// According to the MCP-SPEC:
/// - Clients MUST send `MCP-Protocol-Version` on all HTTP requests.
/// - Servers MUST reject requests with unsupported versions.
/// - Servers MAY fall back if the header is absent.
pub async fn protocol_version_guard(req: Request<Body>, next: Next) -> Response {
    if let Some(v) = req.headers().get(PROTOCOL_HEADER) {
        match v.to_str() {
            Ok(s) if SUPPORTED_PROTOCOL_VERSIONS.contains(&s) => {
                // OK — continue
            }
            Ok(s) => {
                warn!("Unsupported MCP-Protocol-Version: {s}");
                return Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body("unsupported MCP-Protocol-Version".into())
                    .unwrap();
            }
            Err(_) => {
                return Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body("invalid MCP-Protocol-Version header".into())
                    .unwrap();
            }
        }
    } else {
        // No header — per spec, assume 2025-03-26
        tracing::info!("No MCP-Protocol-Version header; assuming 2025-03-26");
    }

    next.run(req).await
}

/// Validates the Origin header to enforce local-only connections.
/// According to the MCP-SPEC:
/// - Servers MUST validate `Origin` for all incoming HTTP requests.
/// - Only `localhost` and `127.0.0.1` origins should be allowed for local transports.
/// - Rejects other origins to mitigate DNS rebinding attacks.
pub async fn origin_guard(req: Request<Body>, next: Next) -> Response {
    if let Some(origin) = req.headers().get("Origin") {
        match origin.to_str() {
            Ok(origin_str)
                if origin_str.starts_with("http://localhost")
                    || origin_str.starts_with("https://localhost")
                    || origin_str.starts_with("http://127.0.0.1")
                    || origin_str.starts_with("https://127.0.0.1") =>
            {
                // Allowed origin
            }
            Ok(origin_str) => {
                warn!(%origin_str, "Rejected request due to invalid Origin header");
                return Response::builder()
                    .status(StatusCode::FORBIDDEN)
                    .body("origin not allowed".into())
                    .unwrap();
            }
            Err(_) => {
                return Response::builder()
                    .status(StatusCode::FORBIDDEN)
                    .body("invalid Origin header".into())
                    .unwrap();
            }
        }
    }

    next.run(req).await
}