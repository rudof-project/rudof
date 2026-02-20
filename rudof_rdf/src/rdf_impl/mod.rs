mod in_memory_graph;
mod in_memory_graph_error;
mod oxrdf_impl;
mod sparql_endpoint;
mod sparql_endpoint_error;

pub use in_memory_graph::{InMemoryGraph, ReaderMode};
pub use in_memory_graph_error::InMemoryGraphError;
pub use sparql_endpoint::{SparqlEndpoint, SparqlVars};
pub use sparql_endpoint_error::SparqlEndpointError;

#[cfg(test)]
mod tests {
    mod in_memory_graph_tests;
    // mod sparql_endpoint_tests;
}
