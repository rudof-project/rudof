use std::path::Path;

use sparql_service::RdfData;
use srdf::{RDFFormat, ReaderMode, SRDFGraph};

use crate::validate_error::ValidateError;

use super::Store;

pub struct Graph {
    store: RdfData,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            store: RdfData::new(),
        }
    }

    pub fn from_path(
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
            Ok(store) => Ok(Self {
                store: RdfData::from_graph(store)?,
            }),
            Err(error) => Err(ValidateError::Graph(error)),
        }
    }

    pub fn from_graph(graph: SRDFGraph) -> Result<Graph, ValidateError> {
        Ok(Graph {
            store: RdfData::from_graph(graph)?,
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
