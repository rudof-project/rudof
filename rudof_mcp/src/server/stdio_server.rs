use anyhow::Result;
use rmcp::ServiceExt;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::rudof_mcp_service::{RudofMcpService, service::ReloadHandle};

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
/// - Logging: Sent to stderr to avoid interfering with protocol
pub async fn run_mcp_stdio(log_handle: Option<Arc<RwLock<ReloadHandle>>>) -> Result<()> {
    tracing::info!("Starting MCP server with stdio transport");
    tracing::info!("Protocol: MCP 2025-06-18 stdio");
    
    let service = if let Some(handle) = log_handle {
        RudofMcpService::with_log_handle(handle)
    } else {
        RudofMcpService::new()
    };
    
    let _server = service
        .serve((tokio::io::stdin(), tokio::io::stdout()))
        .await?;
    
    tracing::info!("MCP stdio server started");
    tracing::info!("Transport: JSON-RPC over stdin/stdout");
    tracing::info!("Logging: stderr");
    
    // The server runs until stdin is closed (EOF)
    // or the client sends a shutdown request
    Ok(())
}
