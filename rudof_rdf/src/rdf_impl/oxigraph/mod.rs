//! Oxigraph-based backends for `rudof_rdf`.
//!
//! Bundles the in-memory graph backed by `oxrdf::Graph` and the remote
//! SPARQL endpoint client, both of which rely on the Oxigraph stack.

#[cfg(all(not(target_family = "wasm"), feature = "sparql"))]
mod endpoint;
#[cfg(all(not(target_family = "wasm"), feature = "sparql"))]
mod endpoint_error;
mod in_memory;
mod in_memory_error;
mod oxrdf_impl;

#[cfg(all(not(target_family = "wasm"), feature = "sparql"))]
pub use endpoint::{OxigraphEndpoint, SparqlVars};
#[cfg(all(not(target_family = "wasm"), feature = "sparql"))]
pub use endpoint_error::OxigraphEndpointError;
pub use in_memory::{OxigraphInMemory, ReaderMode};
pub use in_memory_error::OxigraphInMemoryError;
