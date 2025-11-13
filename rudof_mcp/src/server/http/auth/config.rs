use std::{sync::Arc, time::{Duration, Instant}};
use tokio::sync::RwLock;
use reqwest::Client;
use josekit::jwk::JwkSet;
use anyhow::{Result, Context as AnyhowContext};
use tracing::debug;

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
    pub(super) async fn get_jwks(&self) -> Result<JwkSet> {
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
}
