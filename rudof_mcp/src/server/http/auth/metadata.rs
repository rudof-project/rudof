use axum::{
    http::{StatusCode, header},
    response::IntoResponse,
};
use serde_json::json;

use super::super::config::{AS_URL, SCOPES};

// ============================================================================
// Discovery Endpoints
// ============================================================================

/// Protected resource metadata endpoint
pub async fn protected_resource_metadata_handler(resource: String) -> impl IntoResponse {
    let metadata = json!({
        "resource": resource,
        "authorization_servers": [AS_URL],
        "scopes_supported": SCOPES
    });

    (
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, "application/json"),
            (header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"),
        ],
        serde_json::to_string(&metadata).unwrap(),
    )
}
