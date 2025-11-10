/// MCP-SPEC: The canonical URI of this MCP server.
pub const CANONICAL_URI: &str = "http://localhost:8000/rudof";

/// Authorization Server (Keycloak) base URL.
pub const AS_URL: &str = "http://localhost:8080/realms/mcp-realm";

pub const ROUTE_PATH: &str = "/rudof";

/// MCP-SPEC: When running locally, servers SHOULD bind only to localhost (127.0.0.1) rather than 
/// all network interfaces (0.0.0.0) to mitigate DNS rebinding attacks.
pub const BIND_ADDR: &str = "localhost:8000";