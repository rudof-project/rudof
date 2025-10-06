//! This module defines [ShapeMaps](https://shexspec.github.io/shape-map/)
//!
//! ShapeMaps are used by [ShEx](https://shex.io/) to trigger validation and present validation results.
//!
//! ShapeMaps can associate RDF nodes with shapes indicating whether the RDF nodes conform or not to those shapes.
//!
pub mod association;
pub mod node_selector;
pub mod query_shape_map;
pub mod result_shape_map;
pub mod shape_selector;

#[allow(clippy::module_inception)]
pub mod shapemap;

pub mod shapemap_config;
pub mod shapemap_error;
pub mod shapemap_state;
pub mod validation_status;

pub use association::*;
pub use node_selector::*;
pub use query_shape_map::*;
pub use result_shape_map::*;
pub use shape_selector::*;
pub use shapemap::*;
pub use shapemap_config::*;
pub use shapemap_error::*;
pub use shapemap_state::*;
pub use validation_status::*;

/// Format of Shapemap files
#[derive(Debug, Clone, PartialEq, Default)]
pub enum ShapeMapFormat {
    #[default]
    Compact,
    JSON,
}

impl ShapeMapFormat {
    /// Returns the MIME type associated with the format
    pub fn mime_type(&self) -> &str {
        match self {
            ShapeMapFormat::Compact => "text/plain",
            ShapeMapFormat::JSON => "application/json",
        }
    }
}
