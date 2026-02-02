mod in_memory_graph;
mod in_memory_graph_error;
mod oxrdf_impl;

#[cfg(test)]
mod tests {
    mod in_memory_graph_tests;
}

pub use in_memory_graph::{InMemoryGraph, ReaderMode};

