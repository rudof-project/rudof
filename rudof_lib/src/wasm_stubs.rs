use reqwest::header::HeaderMap;
use std::io::{Error, Read};

pub struct Response;
#[derive(Debug, Clone)]
pub(super) struct Client;
pub(super) struct ClientBuilder;

impl ClientBuilder {
    pub fn new() -> Self {
        Self
    }
    pub fn build(&self) -> Result<Client, Error> {
        Ok(Client)
    }
    pub fn default_headers(&self, _p0: HeaderMap) -> ClientBuilder {
        Self
    }
}

impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder
    }
    pub fn get(&self, _url: &str) -> Self {
        Self
    }
    pub fn send(self) -> Result<Response, Error> {
        Err(Error::other("Client is not implemented in WASM target"))
    }
}

impl Read for Response {
    fn read(&mut self, _buf: &mut [u8]) -> std::io::Result<usize> {
        Err(Error::other("Response is not implemented in WASM target"))
    }
}

pub fn terminal_width() -> usize {
    0
}
