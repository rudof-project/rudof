#![allow(dead_code)]
use reqwest::header::HeaderMap;
use std::io::{Error, Read};

/// HTTP response type for WASM environments.
pub struct Response;
/// HTTP client for WASM environments.
#[derive(Debug, Clone)]
pub struct Client;
/// HTTP client builder for WASM environments.
pub struct ClientBuilder;

impl ClientBuilder {
    /// Creates a new client builder stub.
    pub fn new() -> Self {
        Self
    }

    /// Attempts to build a client (always succeeds in stub).
    ///
    /// Returns a stub Client that will error if used for actual requests.
    pub fn build(&self) -> Result<Client, Error> {
        Ok(Client)
    }

    /// Sets default headers for the client (no-op in stub).
    ///
    /// # Parameters
    /// - `_p0`: Header map (ignored in WASM stub)
    ///
    /// # Returns
    /// Self for method chaining.
    pub fn default_headers(&self, _p0: HeaderMap) -> ClientBuilder {
        Self
    }
}

impl Client {
    /// Creates a new client builder.
    pub fn builder() -> ClientBuilder {
        ClientBuilder
    }

    /// Prepares a GET request (no-op in stub).
    ///
    /// # Parameters
    /// - `_url`: Target URL (ignored in WASM stub)
    ///
    /// # Returns
    /// Self for method chaining.
    pub fn get(&self, _url: &str) -> Self {
        Self
    }

    /// Attempts to send the HTTP request.
    ///
    /// # Errors
    /// Always returns an error indicating that HTTP client operations are not implemented in WASM environments.
    pub fn send(self) -> Result<Response, Error> {
        Err(Error::other("Client is not implemented in WASM target"))
    }
}

impl Read for Response {
    /// Attempts to read from the response.
    ///
    /// # Errors
    /// Always returns an error indicating that HTTP response reading is not implemented in WASM environments.
    fn read(&mut self, _buf: &mut [u8]) -> std::io::Result<usize> {
        Err(Error::other("Reading HTTP response is not implemented in WASM target"))
    }
}
