//! # Rudof MCP Service
//!
//! This module implements a [Model Context Protocol (MCP)](https://modelcontextprotocol.io/) server
//! for the Rudof library using the [rmcp](https://crates.io/crates/rmcp) Rust SDK.
//!
//! ## Overview
//!
//! The MCP server exposes Rudof's library capabilities to AI assistants and other MCP clients.
//!
//! ## MCP Capabilities
//!
//! This server advertises the following MCP capabilities:
//! - `tools`: Access to Rudof's tools for validation, querying, and RDF data operations.
//! - `prompts`: Guided templates for common validation workflows.
//! - `resources`: Access to up-to-date RDF data and supported formats for operations.
//! - `logging`: Real-time log notifications with level filtering.
//! - `completions`: Argument completions for tools and prompts.
//! - `pagination`: Support for paginated resource, prompt and tool listings with opaque cursors.
//!
//! ## Docker State Persistence
//!
//! When running in Docker MCP Registry (ephemeral containers), the server supports
//! state persistence via Docker volumes. Mount a volume to `/app/state/` and the
//! server will automatically save/load RDF data between container restarts.

mod errors;
mod handlers;
mod logging;
mod mcp_service;
mod pagination;
mod prompts;
mod resource_templates;
mod resources;
mod state;
mod tools;

pub use mcp_service::RudofMcpService;
pub use tools::annotated_tools;
