use anyhow::Result;
use rmcp::{ServiceExt, transport::stdio};

use crate::rudof_mcp_service::RudofMcpService;

/// Run MCP server using stdio transport (stdin/stdout).
/// This transport is ideal for:
/// - CLI tools and command-line interfaces
/// - IDE extensions (VSCode, IntelliJ, etc.)
/// - Local process-to-process communication
/// - Single client connections
///
/// # Security Features
/// - Inherits the security context of the parent process
/// - No network exposure, no authentication needed
/// - Isolated to local process communication
///
/// # Protocol
/// - MCP 2025-06-18 stdio transport
/// - JSON-RPC over stdin/stdout
/// - Input: JSON-RPC messages via stdin
/// - Output: JSON-RPC responses via stdout
pub async fn run_mcp_stdio() -> Result<()> {
    tracing::info!("MCP stdio server started");
    tracing::info!("Transport: JSON-RPC over stdin/stdout");

    let server = RudofMcpService::new();

    let service = server.serve(stdio()).await.inspect_err(|e| {
        tracing::error!("Server error: {:?}", e);
    })?;

    service.waiting().await?;

    tracing::info!("MCP stdio server shutting down");

    Ok(())
}
