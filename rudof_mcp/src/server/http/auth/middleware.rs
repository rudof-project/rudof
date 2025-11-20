use axum::{
    body::Body,
    http::{HeaderValue, Request, Response, StatusCode, header},
    middleware::Next,
};
use std::sync::Arc;

use super::config::AuthConfig;

// ============================================================================
// Authorization Middleware
// ============================================================================

/// Authorization guard middleware
pub async fn authorization_guard(
    axum::extract::State(auth_cfg): axum::extract::State<Arc<AuthConfig>>,
    mut req: Request<Body>,
    next: Next,
) -> Response<Body> {
    let path = req.uri().path().to_string();

    tracing::debug!("Authorization guard processing request to: {}", path);

    // Well-known endpoints are always public
    if path.starts_with("/.well-known/") {
        tracing::debug!("Allowing public access to well-known endpoint: {}", path);
        return next.run(req).await;
    }

    // Skip auth if not required
    if !auth_cfg.require_auth {
        tracing::debug!("Authentication not required, allowing request");
        return next.run(req).await;
    }

    // Extract Authorization header
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok());

    tracing::debug!("Authorization header present: {}", auth_header.is_some());

    match auth_header {
        Some(h) if h.to_lowercase().starts_with("bearer ") => {
            let token = match h.split_whitespace().nth(1) {
                Some(t) => {
                    tracing::debug!("Bearer token extracted (length: {})", t.len());
                    t
                }
                None => {
                    tracing::warn!("Malformed Bearer token");
                    return unauthorized_response(
                        &auth_cfg.resource_metadata_url(),
                        Some("invalid_request"),
                        Some("Malformed Authorization header".to_string()),
                    );
                }
            };

            // Verify and validate token
            tracing::debug!("Starting token verification...");
            match auth_cfg.verify_token(token).await {
                Ok(claims) => {
                    tracing::debug!("Token validated successfully for subject: {:?}", claims.sub);
                    tracing::debug!("Token audience: {:?}", claims.aud);
                    tracing::debug!("Token issuer: {}", claims.iss);
                    req.extensions_mut().insert(claims);
                    tracing::debug!("Passing request to next handler");
                    next.run(req).await
                }
                Err(e) => {
                    tracing::error!("Token validation failed: {:#}", e);
                    tracing::error!("Error details: {:?}", e);
                    unauthorized_response(
                        &auth_cfg.resource_metadata_url(),
                        Some("invalid_token"),
                        Some(format!("Token validation failed: {}", e)),
                    )
                }
            }
        }
        Some(_) => {
            tracing::warn!("Invalid Authorization header format");
            bad_request_response("Authorization header must use Bearer scheme")
        }
        None => {
            tracing::warn!(
                "Missing Authorization header for protected endpoint: {}",
                path
            );
            unauthorized_response(
                &auth_cfg.resource_metadata_url(),
                Some("missing_token"),
                Some("Authorization header required".to_string()),
            )
        }
    }
}

// ============================================================================
// Response Helpers
// ============================================================================

fn unauthorized_response(
    resource_metadata: &str,
    error: Option<&str>,
    description: Option<String>,
) -> Response<Body> {
    let mut resp = Response::new("Unauthorized".into());
    *resp.status_mut() = StatusCode::UNAUTHORIZED;

    let mut header_value = format!(r#"Bearer realm="mcp", url="{}""#, resource_metadata);
    if let Some(e) = error {
        header_value.push_str(&format!(r#", error="{}""#, e));
    }
    if let Some(d) = description {
        let sanitized = d.replace('"', "'");
        header_value.push_str(&format!(r#", error_description="{}""#, sanitized));
    }

    resp.headers_mut().insert(
        header::WWW_AUTHENTICATE,
        HeaderValue::from_str(&header_value)
            .unwrap_or_else(|_| HeaderValue::from_static(r#"Bearer realm="mcp""#)),
    );

    resp
}

fn bad_request_response(description: &str) -> Response<Body> {
    let mut resp = Response::new(description.to_string().into());
    *resp.status_mut() = StatusCode::BAD_REQUEST;
    resp
}
