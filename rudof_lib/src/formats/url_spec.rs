use crate::errors::InputSpecError;
use reqwest::blocking::{Client, ClientBuilder};
use std::fmt::Display;
use url::Url;

// ============================================================================
// UrlSpec
// ============================================================================
/// Specification for URL-based inputs with HTTP client configuration.
#[derive(Debug, Clone)]
pub struct UrlSpec {
    /// The URL to fetch data from
    url: Url,
    /// HTTP client for making requests
    client: Client,
}

impl UrlSpec {
    pub fn url(&self) -> &Url {
        &self.url
    }

    pub fn client(&self) -> &Client {
        &self.client
    }
    /// Parses a string into a UrlSpec with an HTTP client.
    pub fn parse(str: &str) -> Result<UrlSpec, InputSpecError> {
        let url = Url::parse(str).map_err(|e| InputSpecError::UrlParseError {
            str: str.to_string(),
            error: format!("{e}"),
        })?;
        let client = ClientBuilder::new()
            .build()
            .map_err(|e| InputSpecError::ClientBuilderError { error: format!("{e}") })?;
        Ok(UrlSpec { url, client })
    }

    /// Returns the URL as a string slice.
    pub fn as_str(&self) -> &str {
        self.url.as_str()
    }
}

impl Display for UrlSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.url)
    }
}
