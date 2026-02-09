use reqwest::header::HeaderMap;
use std::io::{Error, ErrorKind, Read};

#[allow(dead_code)]
pub struct Response;
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Client;
#[allow(dead_code)]
pub struct ClientBuilder;

#[allow(dead_code)]
impl ClientBuilder {
    pub fn new() -> Self { Self }
    pub fn build(&self) -> Client { Client }
    pub fn get(_: &str) {}
    pub fn default_headers(&self, _p0: HeaderMap) -> ClientBuilder { Self }
}

#[allow(dead_code)]
impl Client {
    pub fn builder() -> ClientBuilder { ClientBuilder }
    pub fn get(&self, _url: &str) -> Self { Self }
    pub fn send(self) -> Result<Response, Error> {
        Err(Error::new(ErrorKind::Other, "Client is not implemented in WASM target"))
    }
}

impl Read for Response {
    fn read(&mut self, _buf: &mut [u8]) -> std::io::Result<usize> {
        Err(Error::new(ErrorKind::Other, "Response is not implemented in WASM target"))
    }
}
