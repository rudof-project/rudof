use anyhow::Result;
use rmcp::{ServiceExt, transport::stdio};

use crate::service::RudofMcpService;

/// Run MCP server using stdio transport (stdin/stdout).
/// This transport is ideal for:
/// - CLI tools and command-line interfaces
/// - IDE extensions (VSCode, IntelliJ, etc.)
/// - Local process-to-process communication
/// - Single client connections
///
/// # Protocol (MCP 2025-11-25)
/// - The client launches the MCP server as a subprocess
/// - The server reads JSON-RPC messages from stdin and sends messages to stdout
/// - Messages are individual JSON-RPC requests, notifications, or responses
/// - Messages are delimited by newlines and MUST NOT contain embedded newlines
/// - The server MAY write UTF-8 strings to stderr for logging (informational, debug, error)
/// - The client MAY capture, forward, or ignore stderr and SHOULD NOT assume it indicates errors
///
/// # Important
/// - The server MUST NOT write anything to stdout that is not a valid MCP message
/// - The client MUST NOT write anything to stdin that is not a valid MCP message
pub async fn run_mcp_stdio() -> Result<()> {
    tracing::debug!("Initializing MCP stdio server");

    let server = RudofMcpService::new();
    tracing::debug!("RudofMcpService created");

    let service = server.serve(stdio()).await.map_err(|e| {
        tracing::error!("Failed to initialize stdio transport: {}", e);
        e
    })?;

    tracing::info!("MCP stdio server ready, waiting for messages on stdin");

    service.waiting().await.map_err(|e| {
        tracing::error!("MCP stdio server error: {}", e);
        e
    })?;

    tracing::debug!("MCP stdio server shutdown complete");
    Ok(())
}
