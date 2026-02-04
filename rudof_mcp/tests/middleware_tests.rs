//! Tests for HTTP middleware (MCP protocol guards).
//!
//! These tests verify the protocol version and origin validation

use rudof_mcp::server::{is_valid_origin, is_valid_protocol_version};

// =============================================================================
// Protocol Version Guard Tests
// =============================================================================

mod protocol_version_guard_tests {
    use super::*;

    /// Test that supported protocol versions should be accepted
    #[test]
    fn test_accepts_supported_protocol_version() {
        let supported_versions = vec!["2025-11-25", "2025-06-18", "2025-03-26"];

        for version in supported_versions {
            assert!(
                is_valid_protocol_version(version),
                "Version {} should be valid",
                version
            );
        }
    }

    /// Test that unsupported protocol versions should be rejected
    #[test]
    fn test_rejects_unsupported_protocol_version() {
        let unsupported_versions = vec![
            "2020-01-01",
            "1.0",
            "invalid",
            "",
            "2024-01-01",
            "2025-01-01",
        ];

        for version in unsupported_versions {
            assert!(
                !is_valid_protocol_version(version),
                "Version {} should be invalid",
                version
            );
        }
    }
}

// =============================================================================
// Origin Guard Tests
// =============================================================================

mod origin_guard_tests {
    use super::*;

    /// Test that localhost origins are accepted
    #[test]
    fn test_accepts_localhost_origins() {
        let valid_origins = vec![
            "http://localhost",
            "http://localhost:3000",
            "https://localhost",
            "https://localhost:8080",
            "http://127.0.0.1",
            "http://127.0.0.1:3000",
            "https://127.0.0.1",
            "https://127.0.0.1:8443",
            "http://[::1]",
            "http://[::1]:3000",
            "https://[::1]",
            "https://[::1]:8080",
        ];

        for origin in valid_origins {
            assert!(is_valid_origin(origin), "Origin {} should be valid", origin);
        }
    }

    /// Test that remote origins are rejected per MCP spec
    #[test]
    fn test_rejects_remote_origins() {
        let invalid_origins = vec![
            "http://example.com",
            "https://example.com",
            "http://192.168.1.1",
            "http://10.0.0.1",
            "https://api.example.com",
            "http://attacker.com",
            "http://localhost.attacker.com",
            "http://127.0.0.2",
            "https://localhost.evil.com",
            "http://localhostevil.com",
        ];

        for origin in invalid_origins {
            assert!(
                !is_valid_origin(origin),
                "Origin {} should be invalid (remote)",
                origin
            );
        }
    }
}
