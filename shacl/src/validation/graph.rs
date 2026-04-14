use std::path::Path;
use std::str::FromStr;
use rudof_rdf::rdf_core::RDFFormat;
use sparql_service::RdfData;
use crate::ir::IRSchema;
use crate::validation::engine::{Engine, NativeEngine, SparqlEngine};
use crate::validation::error::ValidationError;
use crate::validation::mode::ShaclValidationMode;
use crate::validation::processor::ShaclProcessor;
use crate::validation::report::ValidationReport;
use crate::validation::store::{Graph, Store};
use crate::validation::validator::Validate;

// TODO - move to validation::algorithm module
/// The In-Memory Graph Validation algorithm
pub(crate) struct GraphValidation {
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
    /// use shacl_validation::shacl_processor::GraphValidation;
    /// use shacl_validation::shacl_processor::ShaclValidationMode;
    /// use shacl_validation::shacl_processor::ShaclProcessor;
    /// use rudof_rdf::rdf_core::RDFFormat;
    ///
    /// let graph_validation = GraphValidation::from_path(
    ///     "../examples/book_conformant.ttl", // example graph (refer to the examples folder)
    ///     RDFFormat::Turtle, // serialization format of the graph
    ///     None, // no base is defined
    /// );
    /// ```
    pub fn from_path<P: AsRef<Path>>(path: P, format: RDFFormat, base: Option<&str>) -> Result<Self, ValidationError> {
        let store = Graph::from_path(path.as_ref(), &format, base)?;
        Ok(Self { store })
    }
}

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