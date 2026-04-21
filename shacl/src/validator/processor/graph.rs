use std::path::Path;
use std::str::FromStr;
use rudof_rdf::rdf_core::RDFFormat;
use sparql_service::RdfData;
use crate::error::ValidationError;
use crate::validator::engine::{Engine, NativeEngine, SparqlEngine};
use crate::validator::processor::ShaclProcessor;
use crate::validator::ShaclValidationMode;
use crate::validator::store::{Graph, Store};

// TODO - move to validation::algorithm module
/// The In-Memory Graph Validation algorithm
pub struct GraphValidation {
    store: Graph,
}

impl GraphValidation {
    pub fn new(store: Graph) -> Self {
        Self { store }
    }

    /// Returns an In-Memory Graph validation SHACL processor.
    ///
    /// # Arguments
    ///
    /// * `data` - A path to the graph's serialization file
    /// * `data_format` - Any of the possible RDF serialization formats
    /// * `base` - An optional String, the base URI
    /// * `mode` - Any of the possible SHACL validation modes
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    ///
    /// use shacl::validator::processor::GraphValidation;
    /// use shacl::validator::ShaclValidationMode;
    /// use shacl::validator::processor::ShaclProcessor;
    /// use rudof_rdf::rdf_core::RDFFormat;
    ///
    /// let graph_validation = GraphValidation::from_path(
    ///     "../examples/book_conformant.ttl", // example graph (refer to the examples folder)
    ///     RDFFormat::Turtle, // serialization format of the graph
    ///     None, // no base is defined
    /// );
    /// ```
    #[cfg(not(target_family = "wasm"))]
    pub fn from_path<P: AsRef<Path>>(path: P, format: RDFFormat, base: Option<&str>) -> Result<Self, ValidationError> {
        let store = Graph::from_path(path.as_ref(), &format, base)?;
        Ok(Self { store })
    }
}

#[cfg(feature = "sparql")]
impl ShaclProcessor<RdfData> for GraphValidation {
    fn store(&self) -> &RdfData {
        self.store.store()
    }

    fn runner(mode: &ShaclValidationMode) -> Box<dyn Engine<RdfData>> {
        match mode {
            ShaclValidationMode::Native => Box::new(NativeEngine::new()),
            ShaclValidationMode::Sparql => Box::new(SparqlEngine::new()),
        }
    }
}

impl From<Graph> for GraphValidation {
    fn from(value: Graph) -> Self {
        Self::new(value)
    }
}