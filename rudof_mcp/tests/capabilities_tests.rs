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

/// Test that resources capability is advertised with subscribe support
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
        Some(true),
        "Resource subscription should be supported"
    );
    assert_eq!(
        resources_cap.list_changed,
        Some(true),
        "Resource list_changed notification should be supported"
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
    assert!(
        logging.get("enabled").and_then(|v| v.as_bool()) == Some(true),
        "Logging should be enabled"
    );
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

/// Test that tasks capability (SEP-1686) is advertised
#[test]
fn test_tasks_capability_advertised() {
    let service = RudofMcpService::new();
    let info = service.get_info();

    assert!(
        info.capabilities.tasks.is_some(),
        "Tasks capability (SEP-1686) should be advertised"
    );
}

/// Test that experimental capability is not set (clean implementation)
#[test]
fn test_no_experimental_capabilities() {
    let service = RudofMcpService::new();
    let info = service.get_info();

    assert!(
        info.capabilities.experimental.is_none(),
        "Experimental capabilities should not be set unless needed"
    );
}

/// Test all required capabilities are present per MCP spec
#[test]
fn test_all_standard_capabilities_present() {
    let service = RudofMcpService::new();
    let info = service.get_info();
    let caps = &info.capabilities;

    // A well-formed MCP server should advertise its capabilities clearly
    // per the spec: "Servers MUST advertise their capabilities"

    let capabilities_count = [
        caps.tools.is_some(),
        caps.prompts.is_some(),
        caps.resources.is_some(),
        caps.logging.is_some(),
        caps.completions.is_some(),
        caps.tasks.is_some(),
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    assert!(
        capabilities_count >= 3,
        "Server should advertise at least tools, prompts, and resources capabilities"
    );
}
