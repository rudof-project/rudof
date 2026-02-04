//! Test suite for the Rudof MCP server.
//!
//! This module contains integration and unit tests for:
//! - Server capabilities (tools, prompts, resources)
//! - Transport types (stdio, HTTP)
//! - HTTP middleware (protocol version, origin guards)
//! - Service initialization and handlers

mod capabilities_tests;
mod middleware_tests;
mod service_tests;
mod transport_tests;
