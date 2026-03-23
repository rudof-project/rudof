use rudof_rdf::rdf_core::RDFFormat;
use rudof_rdf::rdf_impl::{InMemoryGraph, ReaderMode};
use std::path::Path;

use crate::validate_error::ValidateError;

use super::Store;

pub struct Graph {
    store: InMemoryGraph,
}

impl Default for Graph {
    fn default() -> Self {
        Self {
            store: InMemoryGraph::default(),
        }
    }
}

impl Graph {
    pub fn new() -> Graph {
        Graph::default()
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn from_path(path: &Path, rdf_format: RDFFormat, base: Option<&str>) -> Result<Self, Box<ValidateError>> {
        match InMemoryGraph::from_path(path, &rdf_format, base, &ReaderMode::default()) {
            Ok(store) => Ok(Self { store }),
            Err(error) => Err(Box::new(ValidateError::Graph(error))),
        }
    }

    pub fn from_graph(graph: InMemoryGraph) -> Result<Graph, Box<ValidateError>> {
        Ok(Graph { store: graph })
    }
}

impl Store<InMemoryGraph> for Graph {
    fn store(&self) -> &InMemoryGraph {
        &self.store
    }
}
