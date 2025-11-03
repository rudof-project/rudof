use std::{collections::HashMap, sync::Arc};
use axum::{
    body::Body,
    http::{Request, Response, StatusCode, HeaderValue, header},
    middleware::Next,
    response::IntoResponse,
    Json,
};
use serde::{Serialize, Deserialize};
use tracing::warn;
use tokio::sync::RwLock;
use reqwest::Client;
use josekit::jwk::JwkSet;
use josekit::JoseError;
use josekit::jwt;
use josekit::jws::RS256;

#[derive(Clone)]
pub struct AuthConfig {
    pub canonical_uri: String, // MCP canonical URI (audience)
    pub resource_metadata_url: String, // points to external AS base URL
    pub http_client: Client,
    pub jwks_cache: Arc<RwLock<HashMap<String, CachedJwks>>>,
    pub require_auth: bool,
}

#[derive(Clone)]
pub struct CachedJwks {
    pub jwks: JwkSet,
    pub fetched_at: std::time::Instant,
}

#[derive(Serialize, Deserialize)]
pub struct ProtectedResourceMetadata {
    pub issuer: Option<String>,
    pub authorization_servers: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct TokenClaims {
    pub sub: Option<String>,
    pub aud: Vec<String>,
    pub scope: Option<String>,
}

#[allow(dead_code)]
enum AuthError {
    InvalidToken { detail: String },
    InsufficientScope { missing: String },
}

impl From<JoseError> for AuthError {
    fn from(e: JoseError) -> Self {
        AuthError::InvalidToken {
            detail: format!("JWT error: {}", e),
        }
    }
}

impl AuthConfig {
    /// `base_url` should be the external AS base URL (e.g., https://auth.example.com)
    pub fn new(canonical_uri: String, base_url: String, require_auth: bool) -> Self {
        let resource_metadata_url = format!(
            "{}/.well-known/oauth-protected-resource",
            base_url.trim_end_matches('/')
        );
        Self {
            canonical_uri,
            resource_metadata_url,
            http_client: Client::new(),
            jwks_cache: Arc::new(RwLock::new(HashMap::new())),
            require_auth,
        }
    }
}

/// Protected Resource Metadata handler
/// Advertises external AS to clients
pub async fn protected_resource_metadata_handler(
    authcfg: Arc<AuthConfig>
) -> impl IntoResponse {
    let metadata = ProtectedResourceMetadata {
        issuer: Some(authcfg.resource_metadata_url.clone()),
        authorization_servers: vec![ format!(
            "{}/.well-known/oauth-authorization-server",
            authcfg.resource_metadata_url.trim_end_matches("/.well-known/oauth-protected-resource")
        )],
    };
    (StatusCode::OK, Json(metadata)).into_response()
}

/// Authorization middleware
pub async fn authorization_guard(
    axum::extract::State(authcfg): axum::extract::State<Arc<AuthConfig>>,
    mut req: Request<Body>,
    next: Next,
) -> Response<Body> {
    let path = req.uri().path().to_string();
    if path.starts_with("/.well-known/") || !authcfg.require_auth {
        return next.run(req).await;
    }

    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    match auth_header {
        Some(h) if h.to_lowercase().starts_with("bearer ") => {
            let token = h["Bearer ".len()..].trim();
            match verify_jwt(token, &authcfg).await {
                Ok(claims) => {
                    // Validate audience: must match MCP canonical URI
                    if !claims.aud.iter().any(|aud| aud == &authcfg.canonical_uri) {
                        let mut resp: Response<Body> = Response::new(
                            "Token audience mismatch".into()
                        );
                        *resp.status_mut() = StatusCode::UNAUTHORIZED;
                        return resp;
                    }

                    req.extensions_mut().insert(claims);
                    next.run(req).await
                }
                Err(AuthError::InvalidToken { detail }) => {
                    warn!(%detail, "invalid access token");
                    let mut resp: Response<Body> = Response::new("Unauthorized".into());
                    *resp.status_mut() = StatusCode::UNAUTHORIZED;
                    let header_value = format!(
                        r#"Bearer realm="mcp", resource_metadata="{}""#,
                        authcfg.resource_metadata_url
                    );
                    resp.headers_mut().insert(
                        header::WWW_AUTHENTICATE,
                        HeaderValue::from_str(&header_value).unwrap(),
                    );
                    resp
                }
                Err(AuthError::InsufficientScope { missing }) => {
                    let mut resp: Response<Body> =
                        Response::new(format!("Forbidden: missing scope: {}", missing).into());
                    *resp.status_mut() = StatusCode::FORBIDDEN;
                    resp
                }
            }
        }
        Some(_) => {
            let mut resp: Response<Body> = Response::new("Bad Authorization header".into());
            *resp.status_mut() = StatusCode::BAD_REQUEST;
            resp
        }
        None => {
            warn!("Missing Authorization header");
            let mut resp: Response<Body> = Response::new("Authorization required".into());
            *resp.status_mut() = StatusCode::UNAUTHORIZED;
            let header_value = format!(
                r#"Bearer realm="mcp", resource_metadata="{}""#,
                authcfg.resource_metadata_url
            );
            resp.headers_mut().insert(
                header::WWW_AUTHENTICATE,
                HeaderValue::from_str(&header_value).unwrap(),
            );
            resp
        }
    }
}

/// Verify JWT using external AS JWKS
async fn verify_jwt(token: &str, cfg: &AuthConfig) -> Result<TokenClaims, AuthError> {
    let jwks_uri = format!("{}/.well-known/jwks.json", cfg.resource_metadata_url);
    let jwks_bytes = cfg
        .http_client
        .get(&jwks_uri)
        .send()
        .await
        .map_err(|e| AuthError::InvalidToken {
            detail: format!("JWKS fetch failed: {}", e),
        })?
        .bytes()
        .await
        .map_err(|e| AuthError::InvalidToken {
            detail: format!("JWKS read failed: {}", e),
        })?;

    let jwks = JwkSet::from_bytes(&jwks_bytes).map_err(|e| AuthError::InvalidToken {
        detail: format!("JWKS parse failed: {}", e),
    })?;

    let keys = jwks.keys();
    let key = keys.first().ok_or_else(|| AuthError::InvalidToken {
        detail: "No keys found in JWKS".to_string(),
    })?;
    let verifier = RS256.verifier_from_jwk(key)?;
    let (payload, _header) = jwt::decode_with_verifier(token, &verifier)?;

    Ok(TokenClaims {
        sub: payload.subject().map(|s| s.to_string()),
        aud: payload.audience()
            .map(|aud_set| aud_set.iter().map(|s| s.to_string()).collect())
            .unwrap_or_default(),
        scope: payload.claim("scope").and_then(|v| v.as_str()).map(|s| s.to_string()),
    })
}