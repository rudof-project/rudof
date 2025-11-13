/// Authorization Server (Keycloak) base URL.
pub const AS_URL: &str = "http://localhost:8080/realms/mcp-realm";

/// MCP route path
pub const ROUTE_PATH: &str = "/rudof";

/// The list of required scopes for authorization.
pub const SCOPES: &[&str] = &[
    "mcp-audience",
    "mcp:read",
    "mcp:tools",
    "mcp:prompts",
];
