//! This module defines [ShapeMaps](https://shexspec.github.io/shape-map/)
//!
//! ShapeMaps are used by [ShEx](https://shex.io/) to trigger validation and present validation results.
//!
//! ShapeMaps can associate RDF nodes with shapes indicating whether the RDF nodes conform or not to those shapes.
//!
pub mod association;
pub mod node_selector;
pub mod shape_selector;

pub mod query_shape_map;
pub mod shapemap;
pub mod shapemap_state;

pub use crate::association::*;
pub use crate::node_selector::*;
pub use crate::shape_selector::*;
pub use crate::shapemap::*;
pub use crate::shapemap_state::*;
