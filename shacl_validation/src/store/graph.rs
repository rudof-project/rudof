use std::path::Path;

use sparql_service::RdfData;
use srdf::{RDFFormat, ReaderMode, SRDFGraph};

use crate::validate_error::ValidateError;

use super::Store;

pub struct Graph {
    store: RdfData,
}

impl Default for Graph {
    fn default() -> Self {
        Self { store: RdfData::new() }
    }
}

impl Graph {
    pub fn new() -> Graph {
        Graph::default()
    }

    pub fn from_path(path: &Path, rdf_format: RDFFormat, base: Option<&str>) -> Result<Self, Box<ValidateError>> {
        match SRDFGraph::from_path(
            path,
            &rdf_format,
            base,
            &ReaderMode::default(), // TODO: this should be revisited
        ) {
            Ok(store) => Ok(Self {
                store: RdfData::from_graph(store).map_err(|e| Box::new(ValidateError::RdfDataError(e)))?,
            }),
            Err(error) => Err(Box::new(ValidateError::Graph(error))),
        }
    }

    pub fn from_graph(graph: SRDFGraph) -> Result<Graph, Box<ValidateError>> {
        Ok(Graph {
            store: RdfData::from_graph(graph).map_err(|e| Box::new(ValidateError::RdfDataError(e)))?,
        })
    }

    pub fn from_data(data: RdfData) -> Graph {
        Graph { store: data }
    }
}

impl Store<RdfData> for Graph {
    fn store(&self) -> &RdfData {
        &self.store
    }
}
