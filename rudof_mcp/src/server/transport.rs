use clap::ValueEnum;

/// Transport type for MCP server
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum TransportType {
    /// Standard input/output transport (for local CLI usage)
    Stdio,
    /// HTTP with Server-Sent Events transport (for network usage)
    HttpSse,
}

impl Default for TransportType {
    fn default() -> Self {
        Self::HttpSse
    }
}

impl std::fmt::Display for TransportType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Stdio => write!(f, "stdio"),
            Self::HttpSse => write!(f, "http-sse"),
        }
    }
}
