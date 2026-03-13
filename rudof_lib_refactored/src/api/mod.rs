// Module organization for Rudof API implementation
// Each module implements a specific trait for the public API

pub mod core;
pub mod data;
pub mod shex;
pub mod shacl;
pub mod query;
pub mod comparison;
pub mod conversion;
pub mod dctap;
pub mod rdf_config;
pub mod pg_schema;

// Re-export trait implementations for easier access
pub use core::*;
pub use data::*;
pub use shex::*;
pub use shacl::*;
pub use query::*;
pub use comparison::*;
pub use conversion::*;
pub use dctap::*;
pub use rdf_config::*;
pub use pg_schema::*;
