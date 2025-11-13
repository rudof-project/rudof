use std::sync::Arc;
use anyhow::Result;
use tokio::sync::RwLock;
use tracing_subscriber::{reload, prelude::*, EnvFilter, fmt};
use std::io;

use super::transport::TransportType;
use super::http::run_mcp_http;
use super::stdio_server::run_mcp_stdio;

/// Entry point for running the MCP server with the specified transport.
/// This function routes to the appropriate transport implementation:
/// - TransportType::Stdio: Uses stdin/stdout for communication (for CLI tools, IDE extensions)
/// - TransportType::HttpSse: Uses HTTP with SSE for communication (for web clients, network access)
/// 
/// If a reload handle is provided, it will be used for dynamic log level changes.
/// Otherwise, a new tracing subscriber will be initialized.
#[tokio::main]
pub async fn run_mcp(
    transport: TransportType, 
    port: u16, 
    route_path: &str,
    reload_handle: Option<reload::Handle<EnvFilter, tracing_subscriber::Registry>>,
) -> Result<()> {
    let log_handle = if let Some(handle) = reload_handle {
        // Use the provided reload handle from the CLI
        Some(Arc::new(RwLock::new(handle)))
    } else {
        // Initialize tracing if not already done
        initialize_tracing()
    };
    
    match transport {
        TransportType::Stdio => run_mcp_stdio(log_handle).await,
        TransportType::HttpSse => run_mcp_http(port, route_path, log_handle).await,
    }
}

/// Initialize tracing with reloadable log levels
/// Returns a handle for dynamic log level changes
/// If tracing was already initialized, it will be replaced with MCP-specific tracing
pub(crate) fn initialize_tracing() -> Option<Arc<RwLock<reload::Handle<EnvFilter, tracing_subscriber::Registry>>>> {
    // Always initialize MCP-specific tracing (replacing any existing one)
    let fmt_layer = fmt::layer()
        .with_file(true)
        .with_target(false)
        .with_line_number(true)
        .with_writer(io::stderr)
        .with_ansi(false)  // Disable ANSI color codes for MCP
        .without_time();
    
    // Create initial filter from environment or default to "info"
    let initial_filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();
    
    // Create a reloadable layer with the filter
    let (filter_layer, reload_handle) = reload::Layer::new(initial_filter);
    
    // Try to initialize the subscriber with the reloadable filter
    let result = tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .try_init();
    
    match result {
        Ok(_) => {
            tracing::info!("Initialized MCP server with dynamic log level control");
            let handle = Arc::new(RwLock::new(reload_handle));
            tracing::debug!("Created log reload handle successfully");
            Some(handle)
        }
        Err(e) => {
            // Tracing was already initialized elsewhere, we can't replace it
            eprintln!("Warning: Tracing already initialized ({}), dynamic log level changes will not be available", e);
            None
        }
    }
}
