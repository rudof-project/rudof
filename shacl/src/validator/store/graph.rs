use std::path::Path;
use rudof_rdf::rdf_core::RDFFormat;
use rudof_rdf::rdf_impl::{InMemoryGraph, InMemoryGraphError, ReaderMode};
use sparql_service::RdfData;
use crate::error::ValidationError;
use crate::validator::store::Store;

pub struct Graph {
    store: RdfData,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            store: RdfData::new(),
        }
    }

    pub fn from_path(path: &Path, rdf_format: &RDFFormat, base: Option<&str>) -> Result<Self, ValidationError> {
        match InMemoryGraph::from_path(
            path,
            rdf_format,
            base,
            &ReaderMode::default() // TODO - This should revisited
        ) {
            Ok(store) => Ok(Self {
                store: RdfData::from_graph(store)?,
            }),
            Err(err) => Err(err.into())
        }
    }

    pub fn from_graph(graph: InMemoryGraph) -> Result<Self, ValidationError> {
        Ok(Self {
            store: RdfData::from_graph(graph)?,
        })
    }

    pub fn from_data(data: RdfData) -> Self {
        Self{ store: data }
    }
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}

impl Store<RdfData> for Graph {
    fn store(&self) -> &RdfData {
        &self.store
    }
}