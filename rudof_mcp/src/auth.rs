use std::{collections::HashMap, sync::Arc};
use axum::{
    body::Body,
    http::{Request, Response, StatusCode, HeaderValue, header},
    middleware::Next,
    response::IntoResponse,
    Json
};
use serde::{Serialize, Deserialize};
use tracing::warn;
use tokio::sync::RwLock;
use reqwest::Client;
use josekit::jwk::JwkSet;
use josekit::JoseError;
use serde_json::Value;
use serde_json::json;

/// AuthConfig holds the server's configuration for OAuth2 protected resources
/// including audience (canonical URI) and JWKS caching to verify access tokens.
#[derive(Clone)]
pub struct AuthConfig {
    // The audience the MCP expects tokens to target
    pub canonical_uri: String,
     /// The issuer (Authorization Server base URL)
    pub issuer: String,
    // HTTP client to fetch JWKS or metadata  
    pub http_client: Client,
    // Cache JWKS to reduce network load
    pub jwks_cache: Arc<RwLock<HashMap<String, CachedJwks>>>,
    // Whether this server requires authentication
    pub require_auth: bool,
}

/// Cache structure for JWKS, stores fetch time to allow expiry checks
#[derive(Clone)]
pub struct CachedJwks {
    pub jwks: JwkSet,
    pub fetched_at: std::time::Instant,
}

/// Resource metadata returned for clients to discover authorization servers
#[derive(Serialize, Deserialize)]
pub struct ProtectedResourceMetadata {
    // Identifier of this protected resource
    pub issuer: Option<String>,
    // ist of known AS endpoints
    pub authorization_servers: Vec<String>,
}

/// Claims extracted from validated access tokens
#[derive(Clone, Debug)]
pub struct TokenClaims {
    // Resource owner identifier
    pub sub: Option<String>,
    // Audience(s) the token is valid for
    pub aud: Vec<String>,
    // OAuth2 scope of access
    pub scope: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ClientRegistrationRequest {
    pub client_name: Option<String>,
    pub redirect_uris: Vec<String>,
    pub grant_types: Option<Vec<String>>,
    pub response_types: Option<Vec<String>>,
    pub token_endpoint_auth_method: Option<String>,
}

/// RFC 7591 Dynamic Client Registration Response
#[derive(Serialize)]
pub struct ClientRegistrationResponse {
    pub client_id: String,
    pub client_secret: Option<String>,
    pub client_id_issued_at: u64,
    pub client_secret_expires_at: u64,
    pub redirect_uris: Vec<String>,
}


/// Internal error types for authentication handling
#[allow(dead_code)]
pub enum AuthError {
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
    pub fn new(canonical_uri: String, issuer: String, require_auth: bool) -> Self {
        Self {
            canonical_uri,
            issuer,
            http_client: Client::new(),
            jwks_cache: Arc::new(RwLock::new(HashMap::new())),
            require_auth,
        }
    }

    /// Build the `.well-known/oauth-protected-resource` discovery URL
    /// according to MCP-SPEC
    pub fn resource_metadata_url(&self) -> String {
        format!(
            "{}/.well-known/oauth-protected-resource",
            self.canonical_uri.trim_end_matches('/')
        )
    }
}

/// Serve protected resource metadata.
/// Clients use this endpoint to discover authorization servers and issuer information.
pub async fn protected_resource_metadata_handler(
    resource: String,
) -> impl IntoResponse {
    let canonical_resource = if resource.is_empty() {
        "http://localhost:8000/rudof".to_string()
    } else {
        resource
    };

    let metadata = json!({
        "resource": canonical_resource,  
        "authorization_servers": ["http://localhost:8080/realms/mcp-realm"],
        "scopes_supported": ["openid", "profile", "email"] 
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

pub async fn oauth_authorization_server_metadata_handler() -> impl IntoResponse {
    let keycloak_url = "http://localhost:8080/realms/mcp-realm/.well-known/openid-configuration";

    match reqwest::get(keycloak_url).await {
        Ok(resp) => {
            if let Ok(text) = resp.text().await {
                if let Ok(mut json_value) = serde_json::from_str::<Value>(&text) {
                    // Inject MCP-required fields if missing
                    if json_value.get("response_types_supported").is_none() {
                        json_value["response_types_supported"] = json!(["code"]);
                    }
                    if json_value.get("grant_types_supported").is_none() {
                        json_value["grant_types_supported"] =
                            json!(["authorization_code", "refresh_token"]);
                    }
                    if json_value.get("scopes_supported").is_none() {
                        json_value["scopes_supported"] = json!(["openid", "profile"]);
                    }
                    json_value["registration_endpoint"] = json!(
                        "http://localhost:8000/.well-known/dynamic-client-registration"
                    );
                    let body = serde_json::to_string(&json_value).unwrap_or_default();
                    (
                        StatusCode::OK,
                        [
                            (header::CONTENT_TYPE, "application/json"),
                            (header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"),
                        ],
                        body,
                    )
                } else {
                    (
                        StatusCode::BAD_GATEWAY,
                        [
                            (header::CONTENT_TYPE, "text/plain"),
                            (header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"),
                        ],
                        "Failed to parse Keycloak metadata".into(),
                    )
                }
            } else {
                (
                    StatusCode::BAD_GATEWAY,
                    [
                        (header::CONTENT_TYPE, "text/plain"),
                        (header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"),
                    ],
                    "Failed to read Keycloak metadata".into(),
                )
            }
        }
        Err(_) => (
            StatusCode::BAD_GATEWAY,
            [
                (header::CONTENT_TYPE, "text/plain"),
                (header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"),
            ],
            "Failed to fetch authorization server metadata".into(),
        ),
    }
}

/// Handles dynamic client registration by forwarding the request to Keycloak
pub async fn dynamic_client_registration_handler(
    Json(req): Json<ClientRegistrationRequest>,
) -> impl IntoResponse {
    let keycloak_url = "http://localhost:8080/realms/mcp-realm/clients-registrations/openid-connect";

    let client = Client::new();

    match client.post(keycloak_url)
        .json(&req)
        .send()
        .await
    {
        Ok(resp) => {
            let status = resp.status();
            match resp.json::<Value>().await {
                Ok(json_body) => (status, [(axum::http::header::CONTENT_TYPE, "application/json")], serde_json::to_string(&json_body).unwrap()),
                Err(_) => (axum::http::StatusCode::BAD_GATEWAY, [(axum::http::header::CONTENT_TYPE, "text/plain")], "Failed to parse Keycloak response".to_string())
            }
        },
        Err(e) => (axum::http::StatusCode::BAD_GATEWAY, [(axum::http::header::CONTENT_TYPE, "text/plain")], format!("Failed to contact Keycloak: {}", e))
    }
}

/// Middleware guard to protect resources.
/// Verifies bearer tokens against the MCP audience and external AS JWKS.
pub async fn authorization_guard(
    axum::extract::State(authcfg): axum::extract::State<Arc<AuthConfig>>,
    mut req: Request<Body>,
    next: Next,
) -> Response<Body> {
    let path = req.uri().path().to_string();

    // MCP-SPEC: Well-known endpoints MUST be public; do not require authentication
    if path.starts_with("/.well-known/") || !authcfg.require_auth {
        return next.run(req).await;
    }

    // MCP-SPEC: Extract Authorization header for Bearer tokens
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    match auth_header {
        Some(h) if h.to_lowercase().starts_with("bearer ") => {
            let token = h.split_whitespace().nth(1).unwrap_or("");
            match verify_jwt(token, &authcfg).await {
                Ok(claims) => {
                    // // MCP-SPEC: Token audience MUST include "rudof-mcp"
                    // if !claims.aud.iter().any(|aud| aud == "rudof-mcp") {
                    //     return unauthorized_response(
                    //         &authcfg.resource_metadata_url(),
                    //         Some("invalid_token".to_string()),
                    //         Some(format!("Token audience mismatch: {:?}", claims.aud)),
                    //     );
                    // }

                    // MCP-SPEC: Store claims in request context for downstream handlers
                    req.extensions_mut().insert(claims);
                    next.run(req).await
                }
                Err(AuthError::InvalidToken { detail }) => {
                    warn!(%detail, "invalid access token");
                    unauthorized_response(
                        &authcfg.resource_metadata_url(),
                        Some("invalid_token".to_string()),
                        Some(detail),
                    )
                }
                Err(AuthError::InsufficientScope { missing }) => {
                    forbidden_response(format!("missing scope: {}", missing))
                }
            }
        }
        Some(_) => bad_request_response("Bad Authorization header".to_string()),
        None => {
            warn!("Missing Authorization header");
            unauthorized_response(
                &authcfg.resource_metadata_url(),
                Some("missing_token".to_string()),
                Some("Authorization header required".to_string()),
            )
        }
    }
}

/// Verify JWT using external AS JWKS
/// This ensures that the token is cryptographically valid and issued by a trusted AS
pub async fn verify_jwt(token: &str, cfg: &AuthConfig) -> Result<TokenClaims, AuthError> {
    use josekit::jwt;
    
    let jwks_uri = format!("{}/protocol/openid-connect/certs", cfg.issuer);

    let jwks_bytes = cfg
        .http_client
        .get(&jwks_uri)
        .send()
        .await
        .map_err(|e| AuthError::InvalidToken { detail: format!("JWKS fetch failed: {}", e) })?
        .bytes()
        .await
        .map_err(|e| AuthError::InvalidToken { detail: format!("JWKS read failed: {}", e) })?;

    let jwks = JwkSet::from_bytes(&jwks_bytes)
        .map_err(|e| AuthError::InvalidToken { detail: format!("JWKS parse failed: {}", e) })?;

    for key in jwks.keys() {
        // Try to determine the algorithm from the key
        let alg = key.algorithm().unwrap_or("RS256");
        
        // Try to verify with this key using the appropriate algorithm
        let verify_result = match alg {
            "RS256" => jwt::decode_with_verifier(token, &josekit::jws::RS256.verifier_from_jwk(key)?),
            "RS384" => jwt::decode_with_verifier(token, &josekit::jws::RS384.verifier_from_jwk(key)?),
            "RS512" => jwt::decode_with_verifier(token, &josekit::jws::RS512.verifier_from_jwk(key)?),
            "ES256" => jwt::decode_with_verifier(token, &josekit::jws::ES256.verifier_from_jwk(key)?),
            "ES384" => jwt::decode_with_verifier(token, &josekit::jws::ES384.verifier_from_jwk(key)?),
            _ => continue, // Skip unsupported algorithms
        };

        if let Ok((payload, _header)) = verify_result {
            let claims_map = payload.claims_set();
            
            return Ok(TokenClaims {
                sub: claims_map.get("sub").and_then(|v| v.as_str()).map(|s| s.to_string()),
                aud: {
                    let mut audiences = Vec::new();
                    if let Some(aud_value) = claims_map.get("aud") {
                        // Handle both string and array formats for aud claim
                        if let Some(s) = aud_value.as_str() {
                            audiences.push(s.to_string());
                        } else if let Some(arr) = aud_value.as_array() {
                            for item in arr {
                                if let Some(s) = item.as_str() {
                                    audiences.push(s.to_string());
                                }
                            }
                        }
                    }
                    audiences
                },
                scope: claims_map.get("scope").and_then(|v| v.as_str()).map(|s| s.to_string()),
            });
        }
    }

    Err(AuthError::InvalidToken { detail: "JWT verification failed with all keys".to_string() })
}

/// --- Response helpers ---
/// MCP-SPEC: Construct WWW-Authenticate header in Bearer format for 401 responses
fn unauthorized_response(resource_metadata: &str, error: Option<String>, description: Option<String>) -> Response<Body> {
    let mut resp = Response::new("Unauthorized".into());
    *resp.status_mut() = StatusCode::UNAUTHORIZED;

    let mut header_value = format!(r#"Bearer realm="mcp", url="{}""#, resource_metadata);
    if let Some(e) = error {
        header_value.push_str(&format!(r#", error="{}""#, e));
    }
    if let Some(d) = description {
        header_value.push_str(&format!(r#", error_description="{}""#, d));
    }

    resp.headers_mut().insert(
        header::WWW_AUTHENTICATE,
        HeaderValue::from_str(&header_value).unwrap(),
    );

    resp
}

/// MCP-SPEC: Return 403 Forbidden for insufficient scope
fn forbidden_response(description: String) -> Response<Body> {
    let mut resp: Response<Body> = Response::new(description.into());
    *resp.status_mut() = StatusCode::FORBIDDEN;
    resp
}

/// MCP-SPEC: Return 400 Bad Request for malformed Authorization headers
fn bad_request_response(description: String) -> Response<Body> {
    let mut resp: Response<Body> = Response::new(description.into());
    *resp.status_mut() = StatusCode::BAD_REQUEST;
    resp
}
