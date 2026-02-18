mod in_memory_graph;
mod in_memory_graph_error;
mod oxrdf_impl;
#[cfg(feature = "network")]
mod sparql_endpoint;
#[cfg(feature = "network")]
mod sparql_endpoint_error;

pub use in_memory_graph::{InMemoryGraph, ReaderMode};
pub use in_memory_graph_error::InMemoryGraphError;
#[cfg(feature = "network")]
pub use sparql_endpoint::{SparqlEndpoint, SparqlVars};
#[cfg(feature = "network")]
pub use sparql_endpoint_error::SparqlEndpointError;

#[cfg(test)]
mod tests {
    mod in_memory_graph_tests;
    // mod sparql_endpoint_tests;
}
