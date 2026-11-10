#[cfg(any(not(target_family = "wasm"), feature = "sparql"))]
use crate::error::ValidationError;
use crate::validator::ShaclConfig;
use crate::validator::ShaclValidationMode;
#[cfg(feature = "sparql")]
use crate::validator::engine::SparqlEngine;
use crate::validator::engine::{Engine, NativeEngine};
use crate::validator::processor::ShaclProcessor;
use crate::validator::store::{Graph, ReadRecorder};
#[cfg(not(target_family = "wasm"))]
use rudof_rdf::rdf_core::RDFFormat;
#[cfg(not(feature = "sparql"))]
use rudof_rdf::rdf_impl::OxigraphInMemory;
use rudof_rdf_sink::TriplesSink;
#[cfg(feature = "sparql")]
use sparql_service::RdfData;
#[cfg(not(target_family = "wasm"))]
use std::path::Path;

// TODO - move to validation::algorithm module
/// The In-Memory Graph Validation algorithm
pub struct GraphValidation {
    #[cfg(feature = "sparql")]
    store: ReadRecorder<RdfData>,
    #[cfg(not(feature = "sparql"))]
    store: ReadRecorder<OxigraphInMemory>,
}

impl GraphValidation {
    pub fn new(store: Graph) -> Self {
        Self {
            store: ReadRecorder::new(store.into_store()),
        }
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
        Ok(Self::new(store))
    }
}

#[cfg(feature = "sparql")]
impl ShaclProcessor<ReadRecorder<RdfData>> for GraphValidation {
    fn store(&self) -> &ReadRecorder<RdfData> {
        &self.store
    }

    fn runner(mode: &ShaclValidationMode, config: &ShaclConfig) -> Box<dyn Engine<ReadRecorder<RdfData>>> {
        match mode {
            ShaclValidationMode::Native => Box::new(NativeEngine::new(config.recursion_semantics())),
            ShaclValidationMode::Sparql => Box::new(SparqlEngine::new(config.recursion_semantics())),
        }
    }

    fn prepare_store(&mut self) -> Result<(), ValidationError> {
        self.store.inner_mut().check_store().map_err(ValidationError::from)
    }

    fn set_read_sink(&mut self, sink: Option<TriplesSink>) -> Result<(), ValidationError> {
        self.store.set_sink(sink);
        Ok(())
    }

    /// Not while recording: the index reads the whole graph up front, and
    /// then answers class lookups without reading the `rdf:type` and
    /// `rdfs:subClassOf` triples behind them.
    fn use_class_index(&self) -> bool {
        !self.store.is_recording()
    }
}

#[cfg(not(feature = "sparql"))]
impl ShaclProcessor<ReadRecorder<OxigraphInMemory>> for GraphValidation {
    fn store(&self) -> &ReadRecorder<OxigraphInMemory> {
        &self.store
    }

    fn runner(mode: &ShaclValidationMode, config: &ShaclConfig) -> Box<dyn Engine<ReadRecorder<OxigraphInMemory>>> {
        match mode {
            ShaclValidationMode::Native => Box::new(NativeEngine::new(config.recursion_semantics())),
        }
    }

    fn set_read_sink(&mut self, sink: Option<TriplesSink>) -> Result<(), ValidationError> {
        self.store.set_sink(sink);
        Ok(())
    }

    /// Not while recording: the index reads the whole graph up front, and
    /// then answers class lookups without reading the `rdf:type` and
    /// `rdfs:subClassOf` triples behind them.
    fn use_class_index(&self) -> bool {
        !self.store.is_recording()
    }
}

impl From<Graph> for GraphValidation {
    fn from(value: Graph) -> Self {
        Self::new(value)
    }
}
