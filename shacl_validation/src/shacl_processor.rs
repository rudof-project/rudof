use std::path::Path;

use clap::ValueEnum;
use shacl_ast::compiled::schema::CompiledSchema;
use srdf::RDFFormat;
use srdf::SRDFBasic;
use srdf::SRDFGraph;
use srdf::SRDFSparql;

use crate::engine::native::NativeEngine;
use crate::engine::sparql::SparqlEngine;
use crate::engine::Engine;
use crate::shape::Validate;
use crate::store::graph::Graph;
use crate::store::sparql::Endpoint;
use crate::store::Store;
use crate::validate_error::ValidateError;
use crate::validation_report::report::ValidationReport;

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq)]
/// Backend used for the validation.
///
/// According to the SHACL Recommendation, there exists no concrete method for
/// implementing SHACL. Thus, by choosing your preferred SHACL Validation Mode,
/// the user can select which engine is used for the validation.
pub enum ShaclValidationMode {
    /// We use a Rust native engine in an imperative manner (performance)
    Native,
    /// We use a  SPARQL-based engine, which is declarative
    Sparql,
}

/// The basic operations of the SHACL Processor.
///
/// The ShaclProcessor trait is the one in charge of applying the SHACL
/// Validation algorithm. For this, first, the validation report is initiliazed
/// to empty, and, for each shape in the schema, the target nodes are
/// selected, and then, each validator for each constraint is applied.
pub trait ShaclProcessor<S: SRDFBasic> {
    fn store(&self) -> &S;
    fn runner(&self) -> &dyn Engine<S>;

    /// Executes the Validation of the provided Graph, in any of the supported
    /// formats, against the shapes graph passed as an argument. As a result,
    /// the Validation Report generated from the validation process is returned.
    ///
    /// # Arguments
    ///
    /// * `shapes_graph` - A compiled SHACL shapes graph
    fn validate(
        &self,
        shapes_graph: &CompiledSchema<S>,
    ) -> Result<ValidationReport<S>, ValidateError> {
        // we initialize the validation report to empty
        let mut validation_results = Vec::new();

        // for each shape in the schema
        for (_, shape) in shapes_graph.iter() {
            let results = shape.validate(self.store(), self.runner(), None)?;
            validation_results.extend(results);
        }

        Ok(ValidationReport::new(validation_results)) // return the possibly empty validation report
    }
}

/// The In-Memory Graph Validation algorithm.
///
/// ```
/// use std::path::Path;
///
/// use shacl_validation::shacl_processor::GraphValidation;
/// use shacl_validation::shacl_processor::ShaclValidationMode;
/// use shacl_validation::shacl_processor::ShaclProcessor;
/// use shacl_validation::store::ShaclDataManager;
/// use srdf::RDFFormat;
///
/// let graph_validation = GraphValidation::new(
///     Path::new("../examples/book_conformant.ttl"), // example graph (refer to the examples folder)
///     RDFFormat::Turtle, // serialization format of the graph
///     None, // no base is defined
///     ShaclValidationMode::Native, // use the Native mode (performance)
/// )
/// .unwrap();
///
/// // the following schema should generate no errors when the conforming graph
/// // loaded in the previous declaration is used for validation
/// let schema = std::fs::read_to_string(Path::new("../examples/book.ttl")).unwrap();
/// let cursor = std::io::Cursor::new(schema);
/// let compiled_schema = ShaclDataManager::load(cursor, RDFFormat::Turtle, None).unwrap();
///
/// let report = graph_validation.validate(&compiled_schema).unwrap();
///
/// assert_eq!(report.results().len(), 0);
/// ```
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
    /// use srdf::RDFFormat;
    ///
    /// let graph_validation = GraphValidation::new(
    ///     Path::new("../examples/book_conformant.ttl"), // example graph (refer to the examples folder)
    ///     RDFFormat::Turtle, // serialization format of the graph
    ///     None, // no base is defined
    ///     ShaclValidationMode::Native, // use the Native mode (performance)
    /// );
    /// ```
    pub fn new(
        data: &Path,
        data_format: RDFFormat,
        base: Option<&str>,
        mode: ShaclValidationMode,
    ) -> Result<Self, ValidateError> {
        if mode == ShaclValidationMode::Sparql {
            return Err(ValidateError::UnsupportedMode("Graph".to_string()));
        }

        Ok(GraphValidation {
            store: Graph::new(data, data_format, base)?,
            mode,
        })
    }
}

impl ShaclProcessor<SRDFGraph> for GraphValidation {
    fn store(&self) -> &SRDFGraph {
        self.store.store()
    }

    fn runner(&self) -> &dyn Engine<SRDFGraph> {
        match self.mode {
            ShaclValidationMode::Native => &NativeEngine,
            ShaclValidationMode::Sparql => todo!(),
        }
    }
}

/// The Endpoint Graph Validation algorithm.
pub struct EndpointValidation {
    store: Endpoint,
    mode: ShaclValidationMode,
}

impl EndpointValidation {
    pub fn new(data: &str, mode: ShaclValidationMode) -> Result<Self, ValidateError> {
        Ok(EndpointValidation {
            store: Endpoint::new(data)?,
            mode,
        })
    }
}

impl ShaclProcessor<SRDFSparql> for EndpointValidation {
    fn store(&self) -> &SRDFSparql {
        self.store.store()
    }

    fn runner(&self) -> &dyn Engine<SRDFSparql> {
        match self.mode {
            ShaclValidationMode::Native => &NativeEngine,
            ShaclValidationMode::Sparql => &SparqlEngine,
        }
    }
}
