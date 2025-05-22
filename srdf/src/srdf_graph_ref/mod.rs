//! Implementation of the SRDF traits using [OxRDF](https://crates.io/crates/oxrdf) but storing references to nodes instead of values.
pub mod srdfgraph_ref;
pub mod srdfgraph_ref_error;

pub use srdfgraph_ref::*;
pub use srdfgraph_ref_error::*;
