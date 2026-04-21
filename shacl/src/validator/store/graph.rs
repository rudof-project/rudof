use std::path::Path;
use rudof_rdf::rdf_core::RDFFormat;
use rudof_rdf::rdf_impl::{InMemoryGraph, InMemoryGraphError, ReaderMode};
use sparql_service::RdfData;
use crate::error::ValidationError;
use crate::validator::store::Store;

pub struct Graph {
    #[cfg(feature = "sparql")]
    store: RdfData,
    #[cfg(not(feature = "sparql"))]
    store: InMemoryGraph,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            #[cfg(feature = "sparql")]
            store: RdfData::new(),
            #[cfg(not(feature = "sparql"))]
            store: InMemoryGraph::new(),
        }
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn from_path(path: &Path, rdf_format: &RDFFormat, base: Option<&str>) -> Result<Self, ValidationError> {
        match InMemoryGraph::from_path(
            path,
            rdf_format,
            base,
            &ReaderMode::default() // TODO - This should revisited
        ) {
            Ok(store) => Ok(Self {
                #[cfg(feature = "sparql")]
                store: RdfData::from_graph(store)?,
                #[cfg(not(feature = "sparql"))]
                store
            }),
            Err(err) => Err(err.into())
        }
    }
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "sparql")]
impl TryFrom<InMemoryGraph> for Graph {
    type Error = ValidationError;

    fn try_from(value: InMemoryGraph) -> Result<Self, Self::Error> {
        Ok(Self { store: RdfData::from_graph(value)? })
    }
}

#[cfg(not(feature = "sparql"))]
impl From<InMemoryGraph> for Graph {
    fn from(value: InMemoryGraph) -> Self {
        Self { store: value }
    }
}

#[cfg(feature = "sparql")]
impl From<RdfData> for Graph {
    fn from(value: RdfData) -> Self {
        Self { store: value }
    }
}

#[cfg(feature = "sparql")]
impl Store<RdfData> for Graph {
    fn store(&self) -> &RdfData {
        &self.store
    }
}