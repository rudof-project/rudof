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

    /// MCP-SPEC: The `canonical_uri` uniquely identifies this MCP instance as a protected resource.
    /// It serves as the audience claim value for validating OAuth 2.1 Bearer tokens.
    /// Clients MUST use this canonical URI as the `aud` when requesting access tokens from the Authorization Server.
    pub fn canonical_uri(&self) -> String {
        format!("https://{}/{}", self.host, self.route_name)
    }

    /// MCP-SPEC: When running locally, servers SHOULD bind only to localhost (127.0.0.1)
    /// rather than all network interfaces (0.0.0.0) to mitigate DNS rebinding attacks.
    pub fn safe_bind_address(&self) -> String {
        let host = if self.host == "0.0.0.0" { "127.0.0.1" } else { &self.host };
        format!("{}:{}", host, self.port)
    }
}