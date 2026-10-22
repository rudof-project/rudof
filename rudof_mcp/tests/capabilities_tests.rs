//! Tests for MCP server capabilities.
//!
//! These tests verify that the server correctly advertises its capabilities

#[cfg(not(target_family = "wasm"))]
use rmcp::{
    ServerHandler, ServiceExt,
    model::{
        CallToolRequestParams, ClientCapabilities, ClientConfig, Implementation, ProtocolVersion,
        ReadResourceRequestParams,
    },
};
#[cfg(not(target_family = "wasm"))]
use rudof_mcp::service::RudofMcpService;

/// Test that the server can be created successfully
#[cfg(not(target_family = "wasm"))]
#[test]
fn test_service_creation() {
    let service = RudofMcpService::try_new();
    assert!(service.is_ok(), "Service should be created successfully");
}

/// Test that the server can be created with default
#[cfg(not(target_family = "wasm"))]
#[test]
fn test_service_default() {
    let _service = RudofMcpService::default();
    // If we get here without panic, the test passes
}

/// Test server info contains required fields per MCP spec
#[cfg(not(target_family = "wasm"))]
#[test]
fn test_server_info_has_required_fields() {
    let service = RudofMcpService::new();
    let info = service.get_info();

    assert_eq!(
        info.protocol_version,
        ProtocolVersion::LATEST_WITH_INITIALIZE,
        "Protocol version should be the latest MCP version with an initialize handshake"
    );

    // The server must identify itself, not the SDK it is built on.
    assert_eq!(info.server_info.name, env!("CARGO_PKG_NAME"));
    assert_eq!(info.server_info.version, env!("CARGO_PKG_VERSION"));

    // Instructions should be provided for LLMs
    assert!(
        info.instructions.is_some(),
        "Instructions should be provided for LLM context"
    );
}

/// Test that tools capability is advertised
#[cfg(not(target_family = "wasm"))]
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
#[cfg(not(target_family = "wasm"))]
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
#[cfg(not(target_family = "wasm"))]
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
#[cfg(not(target_family = "wasm"))]
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
#[cfg(not(target_family = "wasm"))]
#[test]
fn test_completions_capability_advertised() {
    let service = RudofMcpService::new();
    let info = service.get_info();

    assert!(
        info.capabilities.completions.is_some(),
        "Completions capability should be advertised"
    );
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn test_no_experimental_capabilities() {
    let service = RudofMcpService::new();
    let info = service.get_info();

    assert!(
        info.capabilities.experimental.is_none(),
        "Experimental capabilities should not be set unless needed"
    );
}

#[cfg(not(target_family = "wasm"))]
fn client_pinned_to(protocol_version: ProtocolVersion) -> ClientConfig {
    ClientConfig::new(ClientCapabilities::default(), Implementation::from_build_env())
        .with_protocol_version(protocol_version)
}

/// Drive a real handshake over an in-memory duplex and return the protocol
/// version the server answered with.
#[cfg(not(target_family = "wasm"))]
async fn negotiated_version(client: ClientConfig) -> ProtocolVersion {
    let (server_transport, client_transport) = tokio::io::duplex(4096);

    tokio::spawn(async move {
        if let Ok(server) = RudofMcpService::new().serve(server_transport).await {
            let _ = server.waiting().await;
        }
    });

    let service = client
        .serve(client_transport)
        .await
        .expect("the handshake should succeed");
    let version = service
        .peer_info()
        .expect("the server should have answered")
        .protocol_version
        .clone();
    let _ = service.cancel().await;
    version
}

#[cfg(not(target_family = "wasm"))]
#[tokio::test]
async fn test_handshake_answers_the_protocol_version_the_client_asked_for() {
    let negotiated = negotiated_version(client_pinned_to(ProtocolVersion::V_2025_06_18)).await;

    assert_eq!(
        negotiated,
        ProtocolVersion::V_2025_06_18,
        "the handshake must negotiate, not echo the server ceiling"
    );
}

#[cfg(not(target_family = "wasm"))]
#[tokio::test]
async fn test_handshake_falls_back_for_an_unknown_protocol_version() {
    let unknown: ProtocolVersion =
        serde_json::from_str("\"1999-01-01\"").expect("an unknown version should deserialize");

    let negotiated = negotiated_version(client_pinned_to(unknown)).await;

    assert_eq!(negotiated, ProtocolVersion::LATEST_WITH_INITIALIZE);
}

/// Every format the `rudof://current-data/{format}` template documents must be
/// readable, `turtle` included — it used to 404 while the others worked.
#[cfg(not(target_family = "wasm"))]
#[tokio::test]
async fn test_current_data_is_readable_in_every_documented_format() {
    let (server_transport, client_transport) = tokio::io::duplex(4096);

    tokio::spawn(async move {
        if let Ok(server) = RudofMcpService::new().serve(server_transport).await {
            let _ = server.waiting().await;
        }
    });

    let client = ClientConfig::new(ClientCapabilities::default(), Implementation::from_build_env())
        .serve(client_transport)
        .await
        .expect("the handshake should succeed");

    let mut arguments = serde_json::Map::new();
    arguments.insert(
        "data".to_string(),
        serde_json::json!(["<http://a.example/s> <http://a.example/p> <http://a.example/o> ."]),
    );
    arguments.insert("data_format".to_string(), serde_json::json!("ntriples"));
    client
        .call_tool(CallToolRequestParams::new("load_rdf_data_from_sources").with_arguments(arguments))
        .await
        .expect("loading inline data should succeed");

    for format in ["turtle", "ntriples", "rdfxml", "jsonld", "trig", "nquads", "n3"] {
        let uri = format!("rudof://current-data/{format}");
        let result = client
            .read_resource(ReadResourceRequestParams::new(uri.clone()))
            .await
            .unwrap_or_else(|e| panic!("reading {uri} should succeed: {e}"));

        assert!(!result.contents.is_empty(), "{uri} should return contents");
    }

    let _ = client.cancel().await;
}
