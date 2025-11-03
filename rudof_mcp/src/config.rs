#[derive(Clone)]
pub struct ServerConfig {
    pub route_name: String,
    pub host: String,
    pub port: u16,
}

impl ServerConfig {
    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn canonical_uri(&self) -> String {
        format!("https://{}:{}/{}", self.host, self.port, self.route_name)
    }

    /// MCP-SPEC: When running locally, servers SHOULD bind only to localhost (127.0.0.1)
    /// rather than all network interfaces (0.0.0.0) to mitigate DNS rebinding attacks.
    pub fn safe_bind_address(&self) -> String {
        let host = if self.host == "0.0.0.0" { "127.0.0.1" } else { &self.host };
        format!("{}:{}", host, self.port)
    }
}