use serde::{Serialize, Deserialize};
use serde_json::Value;
use anyhow::{Result, Context as AnyhowContext};
use josekit::jwt::JwtPayload;
use tracing::debug;

use super::config::AuthConfig;

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
// Token Verification
// ============================================================================

impl AuthConfig {
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
