use std::path::Path;

use srdf::{RDFFormat, ReaderMode, SRDFGraph};

use crate::validate_error::ValidateError;

use super::Store;

pub struct Graph {
    store: SRDFGraph,
}

impl Graph {
    // TODO: I would change this to from_path
    pub fn new(
        path: &Path,
        rdf_format: RDFFormat,
        base: Option<&str>,
    ) -> Result<Self, ValidateError> {
        match SRDFGraph::from_path(
            path,
            &rdf_format,
            base,
            &ReaderMode::default(), // TODO: this should be revisited
        ) {
            Ok(store) => Ok(Self { store }),
            Err(error) => Err(ValidateError::Graph(error)),
        }
    }

    pub fn from_graph(graph: SRDFGraph) -> Graph {
        Graph { store: graph }
    }
}

impl Store<SRDFGraph> for Graph {
    fn store(&self) -> &SRDFGraph {
        &self.store
    }
}
