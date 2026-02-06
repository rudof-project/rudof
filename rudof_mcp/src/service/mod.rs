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
//!
//! | Capability    | Feature                                              |
//! |---------------|------------------------------------------------------|
//! | `tools`       | 10+ tools for validation, querying, and data ops     |
//! | `prompts`     | Guided templates for common validation workflows     |
//! | `resources`   | Access to current RDF data in multiple formats       |
//! | `logging`     | Real-time log notifications with level filtering     |
//! | `completions` | Argument completions for tools and prompts           |
//! | `tasks`       | Async task support for long-running operations       |
//!
//! ## Docker State Persistence
//!
//! When running in Docker MCP Registry (ephemeral containers), the server supports
//! state persistence via Docker volumes. Mount a volume to `/app/state/` and the
//! server will automatically save/load RDF data between container restarts.

mod errors;
mod handlers;
mod logging;
mod prompts;
mod resource_templates;
mod resources;
mod service;
pub mod state;
mod tasks;
mod tools;

pub use service::RudofMcpService;
pub use state::{PersistedState, load_state, save_state};
pub use tools::annotated_tools;
