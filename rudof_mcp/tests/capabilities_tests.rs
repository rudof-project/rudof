//! Tests for MCP server capabilities.
//!
//! These tests verify that the server correctly advertises its capabilities

use rmcp::{ServerHandler, model::ProtocolVersion};
use rudof_mcp::service::RudofMcpService;

/// Test that the server can be created successfully
#[test]
fn test_service_creation() {
    let service = RudofMcpService::try_new();
    assert!(service.is_ok(), "Service should be created successfully");
}

/// Test that the server can be created with default
#[test]
fn test_service_default() {
    let _service = RudofMcpService::default();
    // If we get here without panic, the test passes
}

/// Test server info contains required fields per MCP spec
#[test]
fn test_server_info_has_required_fields() {
    let service = RudofMcpService::new();
    let info = service.get_info();

    // Protocol version should be the latest
    assert_eq!(
        info.protocol_version,
        ProtocolVersion::LATEST,
        "Protocol version should be the latest MCP version"
    );

    // Server info (implementation) must be present
    assert!(
        !info.server_info.name.is_empty(),
        "Server implementation name should not be empty"
    );

    // Instructions should be provided for LLMs
    assert!(
        info.instructions.is_some(),
        "Instructions should be provided for LLM context"
    );
}

/// Test that tools capability is advertised
#[test]
fn test_tools_capability_advertised() {
    let service = RudofMcpService::new();
    let info = service.get_info();

    assert!(
        info.capabilities.tools.is_some(),
        "Tools capability should be advertised"
    );
}

/// Test that prompts capability is advertised
#[test]
fn test_prompts_capability_advertised() {
    let service = RudofMcpService::new();
    let info = service.get_info();

    assert!(
        info.capabilities.prompts.is_some(),
        "Prompts capability should be advertised"
    );
}

/// Test that resources capability is advertised
#[test]
fn test_resources_capability_advertised() {
    let service = RudofMcpService::new();
    let info = service.get_info();

    let resources_cap = info
        .capabilities
        .resources
        .expect("Resources capability should be advertised");

    assert_eq!(
        resources_cap.subscribe,
        Some(false),
        "Resource subscription is not yet implemented"
    );
    assert_eq!(
        resources_cap.list_changed,
        Some(false),
        "Resource list_changed notifications are not yet implemented"
    );
}

/// Test that logging capability is enabled
#[test]
fn test_logging_capability_advertised() {
    let service = RudofMcpService::new();
    let info = service.get_info();

    let logging = info
        .capabilities
        .logging
        .expect("Logging capability should be advertised");
    assert!(logging.is_empty(), "Logging capability should be declared as an object");
}

/// Test that completions capability is advertised
#[test]
fn test_completions_capability_advertised() {
    let service = RudofMcpService::new();
    let info = service.get_info();

    assert!(
        info.capabilities.completions.is_some(),
        "Completions capability should be advertised"
    );
}

#[test]
fn test_tasks_capability_not_advertised() {
    let service = RudofMcpService::new();
    let info = service.get_info();

    assert!(
        info.capabilities.tasks.is_none(),
        "Tasks capability (SEP-1686) is not implemented yet"
    );
}

#[test]
fn test_no_experimental_capabilities() {
    let service = RudofMcpService::new();
    let info = service.get_info();

    assert!(
        info.capabilities.experimental.is_none(),
        "Experimental capabilities should not be set unless needed"
    );
}
