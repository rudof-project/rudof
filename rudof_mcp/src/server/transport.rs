use clap::ValueEnum;

/// Transport type for MCP server
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum, Default)]
pub enum TransportType {
    /// Standard input/output transport (for local CLI usage)
    #[default]
    Stdio,
    /// HTTP with Server-Sent Events transport (for network usage)
    StreamableHTTP,
}

impl std::fmt::Display for TransportType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Stdio => write!(f, "stdio"),
            Self::StreamableHTTP => write!(f, "streamable-http"),
        }
    }
}

mod tests {

    #[allow(unused_imports)]
    use super::TransportType;

    /// Test TransportType default is Stdio (as specified for local CLI usage)
    #[test]
    fn test_transport_type_default_is_stdio() {
        let transport = TransportType::default();
        assert_eq!(
            transport,
            TransportType::Stdio,
            "Default transport should be Stdio for CLI tools"
        );
    }

    /// Test TransportType Display implementation for Stdio
    #[test]
    fn test_transport_type_display_stdio() {
        let transport = TransportType::Stdio;
        assert_eq!(
            format!("{}", transport),
            "stdio",
            "Stdio transport should display as 'stdio'"
        );
    }

    /// Test TransportType Display implementation for StreamableHTTP
    #[test]
    fn test_transport_type_display_streamable_http() {
        let transport = TransportType::StreamableHTTP;
        assert_eq!(
            format!("{}", transport),
            "streamable-http",
            "StreamableHTTP transport should display as 'streamable-http'"
        );
    }

    /// Test TransportType Debug implementation
    #[test]
    fn test_transport_type_debug() {
        let stdio = TransportType::Stdio;
        let http = TransportType::StreamableHTTP;

        assert!(
            format!("{:?}", stdio).contains("Stdio"),
            "Debug should contain 'Stdio'"
        );
        assert!(
            format!("{:?}", http).contains("StreamableHTTP"),
            "Debug should contain 'StreamableHTTP'"
        );
    }

    /// Test TransportType equality
    #[test]
    fn test_transport_type_equality() {
        assert_eq!(TransportType::Stdio, TransportType::Stdio);
        assert_eq!(TransportType::StreamableHTTP, TransportType::StreamableHTTP);
        assert_ne!(TransportType::Stdio, TransportType::StreamableHTTP);
    }

    /// Test TransportType Clone
    #[test]
    fn test_transport_type_clone() {
        let original = TransportType::Stdio;
        let cloned = original.clone();
        assert_eq!(original, cloned);

        let original = TransportType::StreamableHTTP;
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }

    /// Test TransportType Copy
    #[test]
    fn test_transport_type_copy() {
        let original = TransportType::Stdio;
        let copied = original; // Copy, not move
        assert_eq!(original, copied); // original is still valid because Copy

        let original = TransportType::StreamableHTTP;
        let copied = original;
        assert_eq!(original, copied);
    }
}
