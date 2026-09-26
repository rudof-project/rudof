#[cfg(not(target_family = "wasm"))]
use crate::error::ValidationError;
use crate::validator::store::Store;
use rudof_rdf::rdf_impl::OxigraphInMemory;
#[cfg(not(target_family = "wasm"))]
use rudof_rdf::{rdf_core::RDFFormat, rdf_impl::ReaderMode};
#[cfg(sparql_validation)]
use sparql_service::RdfData;
#[cfg(not(target_family = "wasm"))]
use std::path::Path;

pub struct Graph {
    #[cfg(sparql_validation)]
    store: RdfData,
    #[cfg(not(sparql_validation))]
    store: OxigraphInMemory,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            #[cfg(sparql_validation)]
            store: RdfData::new(),
            #[cfg(not(sparql_validation))]
            store: OxigraphInMemory::new(),
        }
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn from_path(path: &Path, rdf_format: &RDFFormat, base: Option<&str>) -> Result<Self, ValidationError> {
        match OxigraphInMemory::from_path(
            path,
            rdf_format,
            base,
            &ReaderMode::default(), // TODO - This should revisited
        ) {
            Ok(store) => Ok(Self {
                #[cfg(sparql_validation)]
                store: RdfData::from_graph(store)?,
                #[cfg(not(sparql_validation))]
                store,
            }),
            Err(err) => Err(err.into()),
        }
    }
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(sparql_validation)]
impl TryFrom<OxigraphInMemory> for Graph {
    type Error = ValidationError;

    fn try_from(value: OxigraphInMemory) -> Result<Self, Self::Error> {
        Ok(Self {
            store: RdfData::from_graph(value)?,
        })
    }
}

#[cfg(not(sparql_validation))]
impl From<OxigraphInMemory> for Graph {
    fn from(value: OxigraphInMemory) -> Self {
        Self { store: value }
    }
}

#[cfg(sparql_validation)]
impl From<RdfData> for Graph {
    fn from(value: RdfData) -> Self {
        Self { store: value }
    }
}

#[cfg(sparql_validation)]
impl Store<RdfData> for Graph {
    fn store(&self) -> &RdfData {
        &self.store
    }
}

#[cfg(not(sparql_validation))]
impl Store<OxigraphInMemory> for Graph {
    fn store(&self) -> &OxigraphInMemory {
        &self.store
    }
}

#[cfg(sparql_validation)]
impl Graph {
    pub(crate) fn store_mut(&mut self) -> &mut RdfData {
        &mut self.store
    }
}
