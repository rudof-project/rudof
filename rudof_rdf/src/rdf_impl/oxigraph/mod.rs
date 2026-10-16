//! Oxigraph-based backends for `rudof_rdf`.
//!
//! Bundles the in-memory graph backed by `oxrdf::Graph` and the remote
//! SPARQL endpoint client, both of which rely on the Oxigraph stack.

#[cfg(any(all(not(target_family = "wasm"), feature = "sparql"), feature = "jspi"))]
mod endpoint;
#[cfg(any(all(not(target_family = "wasm"), feature = "sparql"), feature = "jspi"))]
mod endpoint_error;
#[cfg(any(all(not(target_family = "wasm"), feature = "sparql"), feature = "jspi"))]
pub(crate) mod http;
mod in_memory;
mod in_memory_error;
mod oxrdf_impl;

#[cfg(any(all(not(target_family = "wasm"), feature = "sparql"), feature = "jspi"))]
pub use endpoint::{OxigraphEndpoint, SparqlVars};
#[cfg(any(all(not(target_family = "wasm"), feature = "sparql"), feature = "jspi"))]
pub use endpoint_error::OxigraphEndpointError;
pub use in_memory::{OxigraphInMemory, ReaderMode};
pub use in_memory_error::OxigraphInMemoryError;
