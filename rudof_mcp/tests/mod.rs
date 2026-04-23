//! Test suite for the Rudof MCP server.
//!
//! This module contains integration tests for:
//! - Server capabilities (tools, prompts, resources)
//! - Service initialization and handlers

#[cfg(not(target_family = "wasm"))]
mod capabilities_tests;

#[cfg(not(target_family = "wasm"))]
mod service_tests;
