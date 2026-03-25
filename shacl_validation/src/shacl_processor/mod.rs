use crate::shacl_engine::engine::Engine;
use crate::shacl_engine::native::NativeEngine;
use crate::shape_validation::Validate;
use crate::store::Graph;
use crate::store::Store;
use crate::validate_error::ValidateError;
use crate::validation_report::report::ValidationReport;
use rudof_rdf::rdf_core::{NeighsRDF, RDFFormat};
#[cfg(not(feature = "sparql"))]
use rudof_rdf::rdf_impl::InMemoryGraph;
use shacl_ir::compiled::schema_ir::SchemaIR;
use std::fmt::Debug;
#[cfg(not(target_family = "wasm"))]
use std::path::Path;
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};
#[cfg(feature = "sparql")]
use {crate::shacl_engine::sparql::SparqlEngine, sparql_service::RdfData};

#[cfg(feature = "sparql")]
mod sparql;
#[cfg(feature = "sparql")]
pub use sparql::{EndpointValidation, RdfDataValidation};

#[derive(Copy, Clone, Debug, PartialEq, Default)]
/// Backend used for the validation.
///
/// According to the SHACL Recommendation, there exists no concrete method for
/// implementing SHACL. Thus, by choosing your preferred SHACL Validation Mode,
/// the user can select which engine is used for the validation.
pub enum ShaclValidationMode {
    /// Rust native engine using functions implemented with Rust native code
    #[default]
    Native,
    /// SPARQL-based engine using SPARQL queries to validate the data
    #[cfg(feature = "sparql")]
    Sparql,
}

impl Display for ShaclValidationMode {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ShaclValidationMode::Native => write!(dest, "native"),
            #[cfg(feature = "sparql")]
            ShaclValidationMode::Sparql => write!(dest, "sparql"),
        }
    }
}

impl FromStr for ShaclValidationMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "native" => Ok(ShaclValidationMode::Native),
            #[cfg(feature = "sparql")]
            "sparql" => Ok(ShaclValidationMode::Sparql),
            other => Err(format!("Unsupported SHACL validation mode: {}", other)),
        }
    }
}

/// The basic operations of the SHACL Processor.
///
/// The ShaclProcessor trait is the one in charge of applying the SHACL
/// Validation algorithm. For this, first, the validation report is initiliazed
/// to empty, and, for each shape in the schema, the target nodes are
/// selected, and then, each validator for each constraint is applied.
pub trait ShaclProcessor<S: NeighsRDF + Debug> {
    /// Executes the Validation of the provided Graph, in any of the supported
    /// formats, against the shapes graph passed as an argument. As a result,
    /// the Validation Report generated from the validation process is returned.
    ///
    /// # Arguments
    ///
    /// * `shapes_graph` - A compiled SHACL shapes graph
    fn validate(&mut self, shapes_graph: &SchemaIR) -> Result<ValidationReport, Box<ValidateError>>;
}

pub(crate) fn do_validate<S: NeighsRDF + Debug>(
    store: &S,
    mut runner: Box<dyn Engine<S>>,
    shapes_graph: &SchemaIR,
) -> Result<ValidationReport, Box<ValidateError>> {
    runner.build_indexes(store)?;
    let mut validation_results = Vec::new();
    for (_, shape) in shapes_graph.iter_with_targets() {
        tracing::debug!("ShaclProcessor.validate with shape {}", shape.id());
        let results = shape.validate(store, &mut (*runner), None, Some(shape), shapes_graph)?;
        validation_results.extend(results);
    }
    Ok(ValidationReport::new()
        .with_results(validation_results)
        .with_prefixmap(shapes_graph.prefix_map()))
}

/// The In-Memory Graph Validation algorithm.
pub struct GraphValidation {
    store: Graph,
    mode: ShaclValidationMode,
}

impl GraphValidation {
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
    ///     ShaclValidationMode::Native, // use the Native mode (performance)
    /// );
    /// ```
    #[cfg(not(target_family = "wasm"))]
    pub fn from_path<P: AsRef<Path>>(
        data: P,
        data_format: RDFFormat,
        base: Option<&str>,
        mode: ShaclValidationMode,
    ) -> Result<Self, Box<ValidateError>> {
        let store = Graph::from_path(data.as_ref(), data_format, base)?;
        Ok(GraphValidation { store, mode })
    }

    pub fn from_graph(graph: Graph, mode: ShaclValidationMode) -> GraphValidation {
        GraphValidation { store: graph, mode }
    }
}

#[cfg(feature = "sparql")]
impl ShaclProcessor<RdfData> for GraphValidation {
    fn validate(&mut self, shapes_graph: &SchemaIR) -> Result<ValidationReport, Box<ValidateError>> {
        let store = self.store.store();
        let runner: Box<dyn Engine<RdfData>> = match self.mode {
            ShaclValidationMode::Native => Box::new(NativeEngine::new()),
            ShaclValidationMode::Sparql => Box::new(SparqlEngine::new()),
        };
        do_validate(store, runner, shapes_graph)
    }
}

#[cfg(not(feature = "sparql"))]
impl ShaclProcessor<InMemoryGraph> for GraphValidation {
    fn validate(&mut self, shapes_graph: &SchemaIR) -> Result<ValidationReport, Box<ValidateError>> {
        let store = self.store.store();
        let runner: Box<dyn Engine<InMemoryGraph>> = Box::new(NativeEngine::new());
        do_validate(store, runner, shapes_graph)
    }
}
