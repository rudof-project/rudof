use crate::shacl_engine::engine::Engine;
use crate::shacl_engine::native::NativeEngine;
use crate::shacl_engine::sparql::SparqlEngine;
use crate::shape_validation::Validate;
use crate::store::Store;
use crate::store::graph::Graph;
use crate::store::sparql::Endpoint;
use crate::validate_error::ValidateError;
use crate::validation_report::report::ValidationReport;
use clap::ValueEnum;
use prefixmap::PrefixMap;
use shacl_ir::compiled::schema_ir::SchemaIR;
use sparql_service::RdfData;
use srdf::NeighsRDF;
use srdf::RDFFormat;
use srdf::SRDFSparql;
use std::fmt::Debug;
use std::path::Path;
use std::str::FromStr;

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Default)]
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
    Sparql,
}

impl FromStr for ShaclValidationMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "native" => Ok(ShaclValidationMode::Native),
            "sparql" => Ok(ShaclValidationMode::Sparql),
            other => Err(format!("Unsupported SHACL validation mode: {other}")),
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
    // fn store(&self) -> &S;
    // fn runner(&mut self) -> &mut dyn Engine<S>;

    /// Executes the Validation of the provided Graph, in any of the supported
    /// formats, against the shapes graph passed as an argument. As a result,
    /// the Validation Report generated from the validation process is returned.
    ///
    /// # Arguments
    ///
    /// * `shapes_graph` - A compiled SHACL shapes graph
    fn validate(&mut self, shapes_graph: &SchemaIR) -> Result<ValidationReport, Box<ValidateError>>; /*  {
    // we initialize the validation report to empty
    let mut validation_results = Vec::new();
    let store = self.store();
    let runner = self.runner();

    // for each shape in the schema that has at least rust-analyzer-diagnostics-view:/diagnostic%20message%20[17]?17#file:///home/labra/src/rust/rudof/shacl_validation/src/shacl_processor.rsone target
    for (_, shape) in shapes_graph.iter_with_targets() {
    tracing::debug!("ShaclProcessor.validate with shape {}", shape.id());
    let results = shape.validate(store, runner, None, Some(shape), shapes_graph)?;
    validation_results.extend(results);
    }

    // return the possibly empty validation report
    Ok(ValidationReport::new()
    .with_results(validation_results)
    .with_prefixmap(shapes_graph.prefix_map()))
    } */
}

#[derive(Debug)]
pub struct RdfDataValidation {
    data: RdfData,
    mode: ShaclValidationMode,
}

impl RdfDataValidation {
    pub fn from_rdf_data(data: RdfData, mode: ShaclValidationMode) -> Self {
        Self { data, mode }
    }
}

impl ShaclProcessor<RdfData> for RdfDataValidation {
    fn validate(&mut self, shapes_graph: &SchemaIR) -> Result<ValidationReport, Box<ValidateError>> {
        let mut validation_results = Vec::new();
        let mut runner: Box<dyn Engine<RdfData>> = match self.mode {
            ShaclValidationMode::Native => Box::new(NativeEngine::new()),
            ShaclValidationMode::Sparql => Box::new(SparqlEngine::new()),
        };

        for (_, shape) in shapes_graph.iter_with_targets() {
            tracing::debug!("ShaclProcessor.validate with shape {}", shape.id());
            let results = shape.validate(&self.data, &mut (*runner), None, Some(shape), shapes_graph)?;
            validation_results.extend(results);
        }

        // return the possibly empty validation report
        Ok(ValidationReport::new()
            .with_results(validation_results)
            .with_prefixmap(shapes_graph.prefix_map()))
    }
}
/*  fn store(&self) -> &RdfData {
    &self.data
}

fn runner(&mut self) -> &mut dyn Engine<RdfData> {
    match self.mode {
        ShaclValidationMode::Native => &mut self.native_engine,
        ShaclValidationMode::Sparql => &mut SparqlEngine,
    }
} */

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
/// let mut graph_validation = GraphValidation::from_path(
///     "../examples/book_conformant.ttl", // example graph (refer to the examples folder)
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
    /// let graph_validation = GraphValidation::from_path(
    ///     "../examples/book_conformant.ttl", // example graph (refer to the examples folder)
    ///     RDFFormat::Turtle, // serialization format of the graph
    ///     None, // no base is defined
    ///     ShaclValidationMode::Native, // use the Native mode (performance)
    /// );
    /// ```
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

impl ShaclProcessor<RdfData> for GraphValidation {
    fn validate(&mut self, shapes_graph: &SchemaIR) -> Result<ValidationReport, Box<ValidateError>> {
        let mut validation_results = Vec::new();
        let store = self.store.store();
        let mut runner: Box<dyn Engine<RdfData>> = match self.mode {
            ShaclValidationMode::Native => Box::new(NativeEngine::new()),
            ShaclValidationMode::Sparql => Box::new(SparqlEngine::new()),
        };

        for (_, shape) in shapes_graph.iter_with_targets() {
            tracing::debug!("ShaclProcessor.validate with shape {}", shape.id());
            let results = shape.validate(store, &mut (*runner), None, Some(shape), shapes_graph)?;
            validation_results.extend(results);
        }

        // return the possibly empty validation report
        Ok(ValidationReport::new()
            .with_results(validation_results)
            .with_prefixmap(shapes_graph.prefix_map()))
    }
    /*fn store(&self) -> &RdfData {
        self.store.store()
    }

    fn runner(&self) -> &mut dyn Engine<RdfData> {
        match self.mode {
            ShaclValidationMode::Native => &mut NativeEngine,
            ShaclValidationMode::Sparql => &mut SparqlEngine,
        }
    }*/
}

/// The Endpoint Graph Validation algorithm.
pub struct EndpointValidation {
    store: Endpoint,
    mode: ShaclValidationMode,
}

impl EndpointValidation {
    pub fn new(iri: &str, prefixmap: &PrefixMap, mode: ShaclValidationMode) -> Result<Self, Box<ValidateError>> {
        Ok(EndpointValidation {
            store: Endpoint::new(iri, prefixmap)?,
            mode,
        })
    }

    pub fn from_sparql(sparql: SRDFSparql, mode: ShaclValidationMode) -> Result<Self, Box<ValidateError>> {
        let store = Endpoint::from_sparql(sparql);
        Ok(EndpointValidation { store, mode })
    }
}

impl ShaclProcessor<SRDFSparql> for EndpointValidation {
    fn validate(&mut self, shapes_graph: &SchemaIR) -> Result<ValidationReport, Box<ValidateError>> {
        // we initialize the validation report to empty
        let mut validation_results = Vec::new();
        let store = self.store.store();
        let mut runner: Box<dyn Engine<SRDFSparql>> = match self.mode {
            ShaclValidationMode::Native => Box::new(NativeEngine::new()),
            ShaclValidationMode::Sparql => Box::new(SparqlEngine::new()),
        };

        // for each shape in the schema that has at least rust-analyzer-diagnostics-view:/diagnostic%20message%20[17]?17#file:///home/labra/src/rust/rudof/shacl_validation/src/shacl_processor.rsone target
        for (_, shape) in shapes_graph.iter_with_targets() {
            tracing::debug!("ShaclProcessor.validate with shape {}", shape.id());
            let results = shape.validate(store, &mut (*runner), None, Some(shape), shapes_graph)?;
            validation_results.extend(results);
        }

        // return the possibly empty validation report
        Ok(ValidationReport::new()
            .with_results(validation_results)
            .with_prefixmap(shapes_graph.prefix_map()))
    }
    /*fn store(&self) -> &SRDFSparql {
        self.store.store()
    }

    fn runner(&self) -> &dyn Engine<SRDFSparql> {
        match self.mode {
            ShaclValidationMode::Native => &NativeEngine,
            ShaclValidationMode::Sparql => &SparqlEngine,
        }
    }*/
}
