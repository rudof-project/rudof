//! ShExMap operations: bind an RDF graph against an input ShEx schema, materialize an output
//! schema from the bindings, and check a schema pair.  See `shex_ast::shexmap`.
pub mod builders;
pub mod implementations;
mod shexmap_operations_trait;

pub use shexmap_operations_trait::{ShExMapOperations, ShexmapMaterialization};
