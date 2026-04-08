//! Node inspection resources.
//!
//! Provides information about node inspection options and modes
//! for exploring RDF graph structure.
//!
//! ## Resources
//!
//! - `rudof://formats/node-modes` - Available node inspection modes

use rmcp::{
    ErrorData as McpError,
    model::{Annotated, RawResource, ReadResourceResult},
};

use crate::service::resources::{json_resource_result, make_resource};
use crate::service::tools::helpers::{NODE_MODE_ENTRIES, option_entries_json};

/// Returns the list of node inspection resources.
pub fn get_node_resources() -> Vec<Annotated<RawResource>> {
    vec![make_resource(
        "rudof://formats/node-modes",
        "Node Inspection Modes",
        "Available modes for node inspection",
        "application/json",
    )]
}

pub fn handle_node_resource(uri: &str) -> Option<Result<ReadResourceResult, McpError>> {
    match uri {
        "rudof://formats/node-modes" => Some(get_node_modes(uri)),
        _ => None,
    }
}

fn get_node_modes(uri: &str) -> Result<ReadResourceResult, McpError> {
    let modes = option_entries_json("modes", NODE_MODE_ENTRIES, "both");
    json_resource_result(uri, &modes)
}
