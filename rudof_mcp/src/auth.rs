use std::{sync::Arc, time::{Duration, Instant}};
use axum::{
    body::Body,
    http::{Request, Response, StatusCode, HeaderValue, header},
    middleware::Next,
    response::IntoResponse,
};
use serde::{Serialize, Deserialize};
use tracing::{warn, debug, error};
use tokio::sync::RwLock;
use reqwest::Client;
use josekit::jwk::JwkSet;
use josekit::jwt::JwtPayload;
use serde_json::Value;
use serde_json::json;
use anyhow::{Result, Context as AnyhowContext};

use crate::config::AS_URL;

// ============================================================================
// Configuration
// ============================================================================

/// AuthConfig holds the server's configuration for OAuth2 protected resources
#[derive(Clone)]
pub struct AuthConfig {
    /// The canonical URI this MCP server expects in token audience claims
    pub canonical_uri: String,
    /// The issuer (Authorization Server) URL
    pub issuer: String,
    /// HTTP client for fetching JWKS and metadata
    pub http_client: Client,
    /// JWKS cache with expiry tracking
    pub jwks_cache: Arc<RwLock<JwksCache>>,
    /// Whether authentication is required
    pub require_auth: bool,
    /// JWKS cache TTL (default: 1 hour)
    pub jwks_cache_ttl: Duration,
}

/// JWKS cache with automatic expiry
pub struct JwksCache {
    jwks: Option<JwkSet>,
    fetched_at: Option<Instant>,
    ttl: Duration,
}

impl JwksCache {
    fn new(ttl: Duration) -> Self {
        Self {
            jwks: None,
            fetched_at: None,
            ttl,
        }
    }

    fn get(&self) -> Option<&JwkSet> {
        if let (Some(jwks), Some(fetched_at)) = (&self.jwks, self.fetched_at) {
            if fetched_at.elapsed() < self.ttl {
                return Some(jwks);
            }
        }
        None
    }

    fn set(&mut self, jwks: JwkSet) {
        self.jwks = Some(jwks);
        self.fetched_at = Some(Instant::now());
    }
}

// ============================================================================
// Token Claims
// ============================================================================

/// Validated JWT claims extracted from access tokens
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    /// Subject (resource owner identifier)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub: Option<String>,
    /// Issuer (authorization server)
    pub iss: String,
    /// Audience(s) - who the token is intended for
    pub aud: Vec<String>,
    /// Expiration time (Unix timestamp)
    pub exp: i64,
    /// Not before time (Unix timestamp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nbf: Option<i64>,
    /// Issued at time (Unix timestamp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iat: Option<i64>,
    /// JWT ID (unique identifier)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jti: Option<String>,
    /// OAuth2 scope
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    /// Client ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}

impl TokenClaims {
    /// Extract claims from a JWT payload with validation
    fn from_payload(payload: &JwtPayload) -> Result<Self> {
        let claims_map = payload.claims_set();
        
        // Required claims
        let iss = claims_map
            .get("iss")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing 'iss' claim"))?
            .to_string();

        let exp = claims_map
            .get("exp")
            .and_then(|v| v.as_i64())
            .ok_or_else(|| anyhow::anyhow!("Missing or invalid 'exp' claim"))?;

        // Audience can be string or array
        let aud = Self::extract_audience(claims_map)?;

        // Optional claims
        let sub = claims_map.get("sub").and_then(|v| v.as_str()).map(String::from);
        let nbf = claims_map.get("nbf").and_then(|v| v.as_i64());
        let iat = claims_map.get("iat").and_then(|v| v.as_i64());
        let jti = claims_map.get("jti").and_then(|v| v.as_str()).map(String::from);
        let scope = claims_map.get("scope").and_then(|v| v.as_str()).map(String::from);
        let client_id = claims_map
            .get("client_id")
            .or_else(|| claims_map.get("azp"))
            .and_then(|v| v.as_str())
            .map(String::from);

        Ok(Self {
            sub,
            iss,
            aud,
            exp,
            nbf,
            iat,
            jti,
            scope,
            client_id,
        })
    }

    fn extract_audience(claims_map: &serde_json::Map<String, Value>) -> Result<Vec<String>> {
        let aud_value = claims_map
            .get("aud")
            .ok_or_else(|| anyhow::anyhow!("Missing 'aud' claim"))?;

        let mut audiences = Vec::new();
        
        if let Some(s) = aud_value.as_str() {
            audiences.push(s.to_string());
        } else if let Some(arr) = aud_value.as_array() {
            for item in arr {
                if let Some(s) = item.as_str() {
                    audiences.push(s.to_string());
                }
            }
        }

        if audiences.is_empty() {
            return Err(anyhow::anyhow!("'aud' claim is empty or invalid"));
        }

        Ok(audiences)
    }

    /// Validate time-based claims
    fn validate_time_claims(&self) -> Result<()> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .context("Failed to get current time")?
            .as_secs() as i64;

        // Check expiration (with 60s clock skew tolerance)
        if now >= self.exp + 60 {
            return Err(anyhow::anyhow!("Token expired"));
        }

        // Check not-before if present (with 60s clock skew tolerance)
        if let Some(nbf) = self.nbf {
            if now < nbf - 60 {
                return Err(anyhow::anyhow!("Token not yet valid"));
            }
        }

        Ok(())
    }

    /// Validate issuer matches expected value
    fn validate_issuer(&self, expected_issuer: &str) -> Result<()> {
        let expected = expected_issuer.trim_end_matches('/');
        let actual = self.iss.trim_end_matches('/');
        
        if actual != expected {
            return Err(anyhow::anyhow!(
                "Invalid issuer: expected '{}', got '{}'",
                expected,
                actual
            ));
        }
        Ok(())
    }

    /// Validate audience contains expected value
    fn validate_audience(&self, expected_audience: &str) -> Result<()> {
        let expected = expected_audience.trim_end_matches('/');
        
        let has_audience = self.aud.iter().any(|aud| {
            aud.trim_end_matches('/') == expected
        });

        if !has_audience {
            return Err(anyhow::anyhow!(
                "Invalid audience: expected '{}', got {:?}",
                expected,
                self.aud
            ));
        }
        Ok(())
    }
}

// ============================================================================
// Auth Config Implementation
// ============================================================================

impl AuthConfig {
    pub fn new(
        canonical_uri: String,
        issuer: String,
        require_auth: bool,
    ) -> Self {
        Self {
            canonical_uri,
            issuer,
            http_client: Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .expect("Failed to create HTTP client"),
            jwks_cache: Arc::new(RwLock::new(JwksCache::new(Duration::from_secs(3600)))),
            require_auth,
            jwks_cache_ttl: Duration::from_secs(3600),
        }
    }

    pub fn with_cache_ttl(mut self, ttl: Duration) -> Self {
        self.jwks_cache_ttl = ttl;
        self
    }

    /// Build the resource metadata discovery URL
    pub fn resource_metadata_url(&self) -> String {
        format!(
            "{}/.well-known/oauth-protected-resource",
            self.canonical_uri.trim_end_matches('/')
        )
    }

    /// Fetch JWKS from the authorization server
    async fn fetch_jwks(&self) -> Result<JwkSet> {
        let jwks_uri = format!("{}/protocol/openid-connect/certs", self.issuer);
        
        debug!("Fetching JWKS from {}", jwks_uri);
        
        let response = self.http_client
            .get(&jwks_uri)
            .send()
            .await
            .context("Failed to fetch JWKS")?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "JWKS endpoint returned status: {}",
                response.status()
            ));
        }

        let jwks_bytes = response
            .bytes()
            .await
            .context("Failed to read JWKS response")?;

        JwkSet::from_bytes(&jwks_bytes)
            .context("Failed to parse JWKS")
    }

    /// Get JWKS with caching
    async fn get_jwks(&self) -> Result<JwkSet> {
        // Try cache first
        {
            let cache = self.jwks_cache.read().await;
            if let Some(jwks) = cache.get() {
                debug!("Using cached JWKS");
                return Ok(jwks.clone());
            }
        }

        // Cache miss or expired - fetch new JWKS
        debug!("JWKS cache miss - fetching new keys");
        let jwks = self.fetch_jwks().await?;
        
        // Update cache
        {
            let mut cache = self.jwks_cache.write().await;
            cache.set(jwks.clone());
        }

        Ok(jwks)
    }

    /// Verify and validate a JWT access token
    pub async fn verify_token(&self, token: &str) -> Result<TokenClaims> {
        use josekit::jwt;

        let jwks = self.get_jwks().await?;

        // Try to verify with each key in the JWKS
        for key in jwks.keys() {
            let alg = key.algorithm().unwrap_or("RS256");
            
            debug!("Attempting verification with algorithm: {}", alg);
            
            let verify_result = match alg {
                "RS256" => jwt::decode_with_verifier(token, &josekit::jws::RS256.verifier_from_jwk(key)?),
                "RS384" => jwt::decode_with_verifier(token, &josekit::jws::RS384.verifier_from_jwk(key)?),
                "RS512" => jwt::decode_with_verifier(token, &josekit::jws::RS512.verifier_from_jwk(key)?),
                "ES256" => jwt::decode_with_verifier(token, &josekit::jws::ES256.verifier_from_jwk(key)?),
                "ES384" => jwt::decode_with_verifier(token, &josekit::jws::ES384.verifier_from_jwk(key)?),
                "ES512" => jwt::decode_with_verifier(token, &josekit::jws::ES512.verifier_from_jwk(key)?),
                "PS256" => jwt::decode_with_verifier(token, &josekit::jws::PS256.verifier_from_jwk(key)?),
                "PS384" => jwt::decode_with_verifier(token, &josekit::jws::PS384.verifier_from_jwk(key)?),
                "PS512" => jwt::decode_with_verifier(token, &josekit::jws::PS512.verifier_from_jwk(key)?),
                _ => {
                    debug!("Unsupported algorithm: {}", alg);
                    continue;
                }
            };

            if let Ok((payload, _header)) = verify_result {
                debug!("JWT signature verified successfully");
                
                // Extract claims
                let claims = TokenClaims::from_payload(&payload)
                    .context("Failed to extract claims from token")?;

                // Validate claims
                claims.validate_time_claims()
                    .context("Time-based claim validation failed")?;

                claims.validate_issuer(&self.issuer)
                    .context("Issuer validation failed")?;

                claims.validate_audience(&self.canonical_uri)
                    .context("Audience validation failed")?;

                debug!("Token validated successfully for subject: {:?}", claims.sub);
                return Ok(claims);
            }
        }

        Err(anyhow::anyhow!("JWT verification failed with all keys"))
    }
}

// ============================================================================
// Discovery Endpoints
// ============================================================================

/// Protected resource metadata endpoint
pub async fn protected_resource_metadata_handler(
    resource: String,
) -> impl IntoResponse {
    let metadata = json!({
        "resource": resource,
        "authorization_servers": [AS_URL],
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
    
    debug!("Authorization guard processing request to: {}", path);

    // Well-known endpoints are always public
    if path.starts_with("/.well-known/") {
        debug!("Allowing public access to well-known endpoint: {}", path);
        return next.run(req).await;
    }

    // Skip auth if not required
    if !auth_cfg.require_auth {
        debug!("Authentication not required, allowing request");
        return next.run(req).await;
    }

    // Extract Authorization header
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok());

    debug!("Authorization header present: {}", auth_header.is_some());

    match auth_header {
        Some(h) if h.to_lowercase().starts_with("bearer ") => {
            let token = match h.split_whitespace().nth(1) {
                Some(t) => {
                    debug!("Bearer token extracted (length: {})", t.len());
                    t
                }
                None => {
                    warn!("Malformed Bearer token");
                    return unauthorized_response(
                        &auth_cfg.resource_metadata_url(),
                        Some("invalid_request"),
                        Some("Malformed Authorization header".to_string()),
                    );
                }
            };

            // Verify and validate token
            debug!("Starting token verification...");
            match auth_cfg.verify_token(token).await {
                Ok(claims) => {
                    debug!("Token validated successfully for subject: {:?}", claims.sub);
                    debug!("Token audience: {:?}", claims.aud);
                    debug!("Token issuer: {}", claims.iss);
                    req.extensions_mut().insert(claims);
                    debug!("Passing request to next handler");
                    next.run(req).await
                }
                Err(e) => {
                    error!("Token validation failed: {:#}", e);
                    error!("Error details: {:?}", e);
                    unauthorized_response(
                        &auth_cfg.resource_metadata_url(),
                        Some("invalid_token"),
                        Some(format!("Token validation failed: {}", e)),
                    )
                }
            }
        }
        Some(_) => {
            warn!("Invalid Authorization header format");
            bad_request_response("Authorization header must use Bearer scheme")
        }
        None => {
            warn!("Missing Authorization header for protected endpoint: {}", path);
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
        HeaderValue::from_str(&header_value).unwrap_or_else(|_| {
            HeaderValue::from_static(r#"Bearer realm="mcp""#)
        }),
    );

    resp
}

fn bad_request_response(description: &str) -> Response<Body> {
    let mut resp = Response::new(description.to_string().into());
    *resp.status_mut() = StatusCode::BAD_REQUEST;
    resp
}