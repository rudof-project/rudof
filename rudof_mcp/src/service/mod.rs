//! # Rudof MCP Service
//!
//! This module implements a [Model Context Protocol (MCP)](https://modelcontextprotocol.io/) server
//! for the Rudof library using the [rmcp](https://crates.io/crates/rmcp) Rust SDK.
//!
//! ## Overview
//!
//! The MCP server exposes Rudof's library capabilities to AI assistants and other MCP clients.
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────┐
//! │                        MCP Client (AI/LLM)                      │
//! └─────────────────────────────┬───────────────────────────────────┘
//!                               │ JSON-RPC 2.0 (stdio/HTTP)
//! ┌─────────────────────────────┴───────────────────────────────────┐
//! │                      RudofMcpService                            │
//! │  ┌───────────────┐  ┌───────────────┐  ┌───────────────────┐    │
//! │  │  ToolRouter   │  │ PromptRouter  │  │ ResourceHandlers  │    │
//! │  │  (rmcp)       │  │ (rmcp)        │  │                   │    │
//! │  └───────┬───────┘  └───────┬───────┘  └─────────┬─────────┘    │
//! │          │                  │                    │              │
//! │  ┌───────┴──────────────────┴────────────────────┴───────────┐  │
//! │  │                    Rudof Library                          │  │
//! │  │  (ShEx, SHACL, SPARQL, RDF parsing/serialization)         │  │
//! │  └───────────────────────────────────────────────────────────┘  │
//! └─────────────────────────────────────────────────────────────────┘
//! ```
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

mod errors;
mod handlers;
mod logging;
mod prompts;
mod resource_templates;
mod resources;
mod service;
mod tasks;
mod tools;

pub use service::RudofMcpService;
