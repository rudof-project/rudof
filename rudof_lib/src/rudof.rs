// These are the structs that are publicly re-exported
pub use dctap::{DCTAPFormat, DCTap as DCTAP};
pub use iri_s::iri;
pub use mie::Mie;
pub use prefixmap::PrefixMap;
pub use rudof_rdf::rdf_core::{
    RDFFormat,
    query::{QueryResultFormat, QuerySolution, QuerySolutions, SparqlQuery, VarName},
    term::Object,
    visualizer::uml_converter::UmlGenerationMode,
};
pub use rudof_rdf::rdf_impl::{InMemoryGraph, ReaderMode};
pub use shacl_ast::ShaclFormat;
pub use shacl_ast::ast::ShaclSchema;
pub use shacl_ir::compiled::schema_ir::SchemaIR as ShaclSchemaIR;
pub use shacl_validation::shacl_processor::ShaclValidationMode;
pub use shacl_validation::validation_report::report::ValidationReport;
pub use shapes_comparator::{CoShaMo, ComparatorError, CompareSchemaFormat, CompareSchemaMode, ShaCo};
pub use shex_ast::Node;
pub use shex_ast::Schema as ShExSchema;
pub use shex_ast::compact::{ShExFormatter, ShapeMapParser, ShapemapFormatter as ShapeMapFormatter};
pub use shex_ast::ir::shape_label::ShapeLabel;
pub use shex_ast::shapemap::{QueryShapeMap, ResultShapeMap, ShapeMapFormat, SortMode, ValidationStatus};
pub use shex_validation::Validator as ShExValidator;
pub use shex_validation::ValidatorConfig;
pub use sparql_service::RdfData;
pub use sparql_service::ServiceDescription;
pub use sparql_service::ServiceDescriptionFormat;

pub type Result<T> = result::Result<T, RudofError>;

use crate::{
    InputSpec, RudofConfig, RudofError, ShapesGraphSource, UrlSpec,
    compare::{InputCompareFormat, InputCompareMode},
    data_format::DataFormat,
    dctap_result_format::DCTapResultFormat,
    generate_schema_format::GenerateSchemaFormat,
    node_info::{NodeInfo, NodeInfoOptions, format_node_info_list, get_node_info},
    query::execute_query,
    query_result_format::ResultQueryFormat,
    query_type::QueryType,
    rdf_config::RdfConfigResultFormat,
    rdf_reader_mode::RDFReaderMode,
    selector::parse_node_selector,
};
use iri_s::{IriS, MimeType};
use rdf_config::RdfConfigModel;
// use shex_validation::SchemaWithoutImports;
use pgschema::{
    parser::{map_builder::MapBuilder, pg_builder::PgBuilder, pgs_builder::PgsBuilder},
    pg::PropertyGraph,
    pgs::PropertyGraphSchema,
    type_map::TypeMap,
    validation_result::ValidationResult,
};
use rudof_generate::{DataGenerator, GeneratorConfig, config::OutputFormat};
use rudof_rdf::rdf_core::{FocusRDF, Rdf, query::QueryRDF, visualizer::VisualRDFGraph};
use rudof_rdf::rdf_impl::SparqlEndpoint;
use shacl_rdf::{ShaclParser, ShaclWriter};
use shacl_validation::shacl_processor::{GraphValidation, ShaclProcessor};
use shacl_validation::store::graph::Graph;
use shapes_comparator::CoShaMoConverter;
use shapes_converter::{ShEx2Uml, Tap2ShEx};
use shex_ast::compact::ShExParser;
use shex_ast::ir::schema_ir::SchemaIR;
use shex_ast::shapemap::{NodeSelector, ShapeSelector};
use shex_ast::{ResolveMethod, ShExFormat, ShapeLabelIdx};
use std::fmt::Debug;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::str::FromStr;
use std::{env, io, path::PathBuf, result};
use tracing::trace;
#[cfg(not(target_family = "wasm"))]
use url::Url;

/// This represents the public API to interact with `rudof`
#[derive(Debug)]
pub struct Rudof {
    /// Version of Rudof
    version: String,

    /// Current configuration
    config: RudofConfig,

    /// Current RDF Data
    rdf_data: RdfData,

    /// Current SHACL Schema
    shacl_schema: Option<ShaclSchema<RdfData>>,

    /// Current SHACL Schema Internal Representation
    shacl_schema_ir: Option<ShaclSchemaIR>,

    /// Current ShEx Schema
    shex_schema: Option<ShExSchema>,

    /// ShEx Schema Internal Representation
    shex_schema_ir: Option<SchemaIR>,

    /// Current ShEx validator. It holds the compiled schema and the validator which can be reused several times if needed
    shex_validator: Option<ShExValidator>,

    /// Current ShEx Schema
    shapemap: Option<QueryShapeMap>,

    /// Current DCTAP
    dctap: Option<DCTAP>,

    /// Current ShEx validation results
    shex_results: Option<ResultShapeMap>,

    /// Current SPARQL Query
    sparql_query: Option<SparqlQuery>,

    /// Current Service Description
    service_description: Option<ServiceDescription>,

    /// Current rdf_config model
    rdf_config: Option<RdfConfigModel>,
}

// TODO: We added this declaration so PyRudof can contain Rudof and be Send as required by PyO3
// TODO: Review what are the consequences of this declaration
unsafe impl Send for Rudof {}

impl Rudof {
    /// Create a new instance of Rudof with the given configuration
    pub fn new(config: &RudofConfig) -> Result<Rudof> {
        let rdf_data = RdfData::new()
            .with_rdf_data_config(&config.rdf_data_config())
            .map_err(|e| RudofError::RdfDataConfigError { error: format!("{e}") })?;
        Ok(Rudof {
            version: env!("CARGO_PKG_VERSION").to_string(),
            config: config.clone(),
            shex_schema: None,
            shex_schema_ir: None,
            shacl_schema: None,
            shacl_schema_ir: None,
            shex_validator: None,
            rdf_data,
            shapemap: None,
            dctap: None,
            shex_results: None,
            sparql_query: None,
            service_description: None,
            rdf_config: None,
        })
    }

    /// Get the current configuration
    pub fn config(&self) -> &RudofConfig {
        &self.config
    }

    /// Get the current version of Rudof
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Use one endpoint by its name or URL in the next queries
    pub fn use_endpoint(&mut self, name: &str) -> Result<()> {
        let (name, sparql_endpoint) = self.get_endpoint(name)?;
        self.rdf_data.use_endpoint(&name, sparql_endpoint);
        Ok(())
    }

    /// Stop using an endpoint by its name in the next queries
    pub fn dont_use_endpoint(&mut self, name: &str) {
        self.rdf_data.dont_use_endpoint(name);
    }

    /// Get the list of endpoints to use in the queries
    pub fn endpoints_to_use(&self) -> Vec<(String, SparqlEndpoint)> {
        self.rdf_data
            .endpoints_to_use()
            .map(|(name, endpoint)| (name.to_string(), endpoint.clone()))
            .collect::<Vec<_>>()
    }

    /// Update the current configuration
    pub fn update_config(&mut self, config: &RudofConfig) {
        self.config = config.clone();
    }

    /// Resets the current RDF Data
    pub fn reset_data(&mut self) {
        self.rdf_data.reset()
    }

    /// Resets the current DCTAP
    pub fn reset_dctap(&mut self) {
        self.dctap = None
    }

    /// Resets the current query from a String
    pub fn read_query_str(&mut self, str: &str) -> Result<()> {
        let query = SparqlQuery::new(str).map_err(|e| RudofError::SparqlSyntaxError {
            error: format!("{e}"),
            source_name: "string".to_string(),
        })?;
        self.sparql_query = Some(query);
        Ok(())
    }

    /// Resets the current SHACL shapes graph
    pub fn reset_shacl(&mut self) {
        self.shacl_schema = None
    }

    /// Resets the current SPARQL query
    pub fn reset_query(&mut self) {
        self.sparql_query = None
    }

    /// Resets the current service description
    pub fn reset_service_description(&mut self) {
        self.service_description = None
    }

    /// Resets all current values
    pub fn reset_all(&mut self) {
        self.reset_data();
        self.reset_dctap();
        self.reset_shacl();
        self.reset_shapemap();
        self.reset_validation_results();
        self.reset_shex();
        self.reset_query();
        self.reset_service_description();
    }

    /// Get the shapes graph schema from the current RDF data
    pub fn get_shacl_from_data(&mut self) -> Result<()> {
        let schema = shacl_schema_from_data(self.rdf_data.clone())?;
        self.shacl_schema = Some(schema.clone());
        let shacl_ir = ShaclSchemaIR::compile(&schema).map_err(|e| RudofError::ShaclCompilation { error: e })?;
        self.shacl_schema_ir = Some(shacl_ir);
        Ok(())
    }

    /// Get the current SHACL
    pub fn get_shacl(&self) -> Option<&ShaclSchema<RdfData>> {
        self.shacl_schema.as_ref()
    }

    /// Get the current SPARQL Query
    pub fn get_query(&self) -> Option<&SparqlQuery> {
        self.sparql_query.as_ref()
    }

    /// Get the current SHACL Schema Internal Representation
    pub fn get_shacl_ir(&self) -> Option<&ShaclSchemaIR> {
        self.shacl_schema_ir.as_ref()
    }

    /// Get the current ShEx Schema
    pub fn get_shex(&self) -> Option<&ShExSchema> {
        self.shex_schema.as_ref()
    }

    /// Get the current Service Description
    pub fn get_service_description(&self) -> Option<&ServiceDescription> {
        self.service_description.as_ref()
    }

    /// Get the current ShEx Schema Internal Representation
    pub fn get_shex_ir(&self) -> Option<&SchemaIR> {
        self.shex_schema_ir.as_ref()
    }

    pub fn get_rdf_config(&self) -> Option<&RdfConfigModel> {
        self.rdf_config.as_ref()
    }

    /// Get the current DCTAP
    pub fn get_dctap(&self) -> Option<&DCTAP> {
        self.dctap.as_ref()
    }

    /// Get the current shapemap
    pub fn get_shapemap(&self) -> Option<&QueryShapeMap> {
        self.shapemap.as_ref()
    }

    /// List the registered SPARQL endpoints
    pub fn list_use_endpoints(&self) -> Vec<(String, IriS)> {
        self.rdf_data
            .use_endpoints()
            .iter()
            .map(|(name, endpoint)| (name.clone(), endpoint.iri().clone()))
            .collect()
    }

    pub fn list_endpoints(&self) -> Vec<(String, IriS)> {
        self.rdf_data
            .endpoints()
            .iter()
            .map(|(name, endpoint)| (name.clone(), endpoint.iri().clone()))
            .collect()
    }

    #[allow(clippy::too_many_arguments)]
    pub fn compare_schemas<R: io::Read>(
        &mut self,
        reader1: &mut R,
        reader2: &mut R,

        mode1: InputCompareMode,
        mode2: InputCompareMode,

        format1: InputCompareFormat,
        format2: InputCompareFormat,

        base1: Option<&str>,
        base2: Option<&str>,

        reader_mode: &RDFReaderMode,

        label1: Option<&str>,
        label2: Option<&str>,

        source_name1: Option<&str>,
        source_name2: Option<&str>,
    ) -> Result<ShaCo> {
        let reader_mode: ReaderMode = reader_mode.into();
        let coshamo1 = self.get_coshamo(reader1, &mode1, &format1, base1, &reader_mode, label1, source_name1)?;
        let coshamo2 = self.get_coshamo(reader2, &mode2, &format2, base2, &reader_mode, label2, source_name2)?;
        Ok(coshamo1.compare(&coshamo2))
    }

    /// Converts the current DCTAP to a ShExSchema
    /// Stores the value of the ShExSchema in the current shex
    pub fn dctap2shex(&mut self) -> Result<()> {
        if let Some(dctap) = self.get_dctap() {
            let converter = Tap2ShEx::new(&self.config.tap2shex_config());
            let shex = converter
                .convert(dctap)
                .map_err(|e| RudofError::DCTap2ShEx { error: format!("{e}") })?;
            self.shex_schema = Some(shex);
            Ok(())
        } else {
            Err(RudofError::NoDCTAP)
        }
    }

    /// Generate a PlantUML representation of RDF Data
    ///
    /// The visualization configuration is taken from the current Rudof configuration
    /// The configuration can be customized to change colors, styles, and other aspects of the visualization
    pub fn data2plant_uml<W: io::Write>(&self, writer: &mut W) -> Result<()> {
        let config = self.config.rdf_data_config().rdf_visualization_config();
        let converter = VisualRDFGraph::from_rdf(&self.rdf_data, config)
            .map_err(|e| RudofError::RDF2PlantUmlError { error: format!("{e}") })?;
        converter
            .as_plantuml(writer, &UmlGenerationMode::AllNodes)
            .map_err(|e| RudofError::RDF2PlantUmlErrorAsPlantUML { error: format!("{e}") })?;
        Ok(())
    }

    /// Generate a UML Class-like representation of a ShEx schema according to PlantUML syntax
    ///
    pub fn shex2plant_uml<W: io::Write>(&self, mode: &UmlGenerationMode, writer: &mut W) -> Result<()> {
        if let Some(shex) = &self.shex_schema {
            let mut converter = ShEx2Uml::new(&self.config.shex2uml_config());
            converter
                .convert(shex)
                .map_err(|e| RudofError::ShEx2PlantUmlError { error: format!("{e}") })?;
            converter
                .as_plantuml(writer, mode)
                .map_err(|e| RudofError::ShEx2PlantUmlErrorAsPlantUML { error: format!("{e}") })?;
            Ok(())
        } else {
            Err(RudofError::ShEx2UmlWithoutShEx)
        }
    }

    pub fn serialize_data<W: io::Write>(&self, format: &RDFFormat, writer: &mut W) -> Result<()> {
        self.rdf_data
            .serialize(format, writer)
            .map_err(|e| RudofError::SerializingData { error: format!("{e}") })
    }

    /// Serialize the current ShapeMap
    pub fn serialize_shapemap<W: io::Write>(
        &self,
        format: &ShapeMapFormat,
        formatter: &ShapeMapFormatter,
        writer: &mut W,
    ) -> Result<()> {
        if let Some(shapemap) = &self.shapemap {
            match format {
                ShapeMapFormat::Compact => {
                    formatter
                        .write_shapemap(shapemap, writer)
                        .map_err(|e| RudofError::ErrorFormattingShapeMap {
                            shapemap: format!("{:?}", shapemap.clone()),
                            error: format!("{e}"),
                        })
                },
                ShapeMapFormat::Json => {
                    serde_json::to_writer_pretty(writer, &shapemap).map_err(|e| RudofError::ErrorWritingShExJson {
                        schema: format!("{:?}", shapemap.clone()),
                        error: format!("{e}"),
                    })?;
                    Ok(())
                },
                ShapeMapFormat::Csv => {
                    todo!()
                },
            }
        } else {
            Err(RudofError::NoShapeMapToSerialize)
        }
    }

    /// Serialize the current ShEx Schema
    pub fn serialize_shex<W: io::Write>(
        &self,
        shex: &ShExSchema,
        format: &ShExFormat,
        formatter: &ShExFormatter,
        writer: &mut W,
    ) -> Result<()> {
        match format {
            ShExFormat::ShExC => {
                formatter
                    .write_schema(shex, writer)
                    .map_err(|e| RudofError::ErrorFormattingSchema {
                        schema: format!("{:?}", shex.clone()),
                        error: format!("{e}"),
                    })?;
                Ok(())
            },
            ShExFormat::ShExJ => {
                serde_json::to_writer_pretty(writer, &shex).map_err(|e| RudofError::ErrorWritingShExJson {
                    schema: format!("{:?}", shex.clone()),
                    error: format!("{e}"),
                })?;
                Ok(())
            },
            ShExFormat::RDFFormat(_) => Err(RudofError::NotImplemented {
                msg: format!("ShEx from RDF format ShExR for {shex:?}"),
            }),
        }
    }

    /// Serialize a specific shape in the current ShEx Schema
    pub fn serialize_shape_current_shex<W: io::Write>(
        &self,
        shape_selector: &ShapeSelector,
        _format: &ShExFormat,
        _formatter: &ShExFormatter,
        writer: &mut W,
    ) -> Result<()> {
        if let Some(shex) = &self.shex_schema_ir {
            for shape_expr_label in shape_selector.iter_shape() {
                let shape_label =
                    ShapeLabel::from_shape_expr_label(shape_expr_label, &shex.prefixmap()).map_err(|e| {
                        RudofError::InvalidShapeLabel {
                            label: shape_expr_label.to_string(),
                            error: format!("{e}"),
                        }
                    })?;
                if let Some((idx, shape_expr)) = shex.find_label(&shape_label) {
                    writeln!(writer, "# Shape {shape_label}")?;
                    write!(writer, "  {shape_expr}")?;
                    trace!("Show triple expressions with extends");
                    writeln!(writer, "  # Triple expressions with extends:")?;
                    show_triple_exprs(idx, shex, writer)?;
                    writeln!(writer, "Predicates:")?;
                    let preds = shex.get_preds_extends(idx);
                    writeln!(
                        writer,
                        "  # Predicates: [{}]",
                        preds.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(" ")
                    )?;
                } else {
                    write!(writer, "Shape {shape_label} not found in schema")?;
                }
            }
            Ok(())
        } else {
            Err(RudofError::NoShExSchemaToSerialize)
        }
    }

    /// Serialize the current ShEx Schema
    pub fn serialize_current_shex<W: io::Write>(
        &self,
        format: &ShExFormat,
        formatter: &ShExFormatter,
        writer: &mut W,
    ) -> Result<()> {
        if let Some(shex) = &self.shex_schema {
            self.serialize_shex(shex, format, formatter, writer)
        } else {
            Err(RudofError::NoShExSchemaToSerialize)
        }
    }

    pub fn run_query_construct_str(&mut self, str: &str, result_format: &QueryResultFormat) -> Result<String> {
        self.rdf_data
            .check_store()
            .map_err(|e| RudofError::StorageError { error: format!("{e}") })?;
        let result = self
            .rdf_data
            .query_construct(str, result_format)
            .map_err(|e| RudofError::QueryError {
                str: str.to_string(),
                error: format!("{e}"),
            })?;
        Ok(result)
    }

    pub fn run_query_construct<R: io::Read>(
        &mut self,
        reader: &mut R,
        query_format: &QueryResultFormat,
    ) -> Result<String> {
        let mut str = String::new();
        reader
            .read_to_string(&mut str)
            .map_err(|e| RudofError::ReadError { error: format!("{e}") })?;
        self.run_query_construct_str(str.as_str(), query_format)
    }

    pub fn run_query_select_str(&mut self, str: &str) -> Result<QuerySolutions<RdfData>> {
        trace!("Running SELECT query: {str}");
        self.rdf_data
            .check_store()
            .map_err(|e| RudofError::StorageError { error: format!("{e}") })?;
        trace!("After checking RDF store");
        let results = self.rdf_data.query_select(str).map_err(|e| RudofError::QueryError {
            str: str.to_string(),
            error: format!("{e}"),
        })?;
        Ok(results)
    }

    pub fn run_query_select<R: io::Read>(&mut self, reader: &mut R) -> Result<QuerySolutions<RdfData>> {
        let mut str = String::new();
        reader
            .read_to_string(&mut str)
            .map_err(|e| RudofError::ReadError { error: format!("{e}") })?;
        self.run_query_select_str(str.as_str())
    }

    pub fn serialize_shacl<W: io::Write>(&self, format: &ShaclFormat, writer: &mut W) -> Result<()> {
        if let Some(shacl) = &self.shacl_schema {
            match format {
                ShaclFormat::Internal => write!(writer, "{shacl}")
                    .map_err(|e| RudofError::SerializingSHACLInternal { error: format!("{e}") }),
                _ => {
                    let data_format = shacl_format2rdf_format(format)?;
                    let mut shacl_writer: ShaclWriter<RdfData> = ShaclWriter::new();
                    shacl_writer.write(shacl).map_err(|e| RudofError::WritingSHACL {
                        shacl: format!("{:?}", shacl.clone()),
                        error: format!("{e}"),
                    })?;
                    shacl_writer
                        .serialize(&data_format, writer)
                        .map_err(|e| RudofError::SerializingSHACL {
                            error: format!("{e}"),
                            shacl: format!("{:?}", shacl.clone()),
                        })?;
                    Ok(())
                },
            }
        } else {
            Err(RudofError::NoShaclToSerialize)
        }
    }

    /// Resets the current ShEx validation results
    /// The action is necessary to start a fresh validation
    pub fn reset_validation_results(&mut self) {
        // TODO: We could add another operation to reset only the current validation results keeping the compiled schema
        /*if let Some(ref mut validator) = &mut self.shex_validator {
            validator.reset_result_map()
        }*/
        self.shex_results = None
    }

    /// Resets the current validator
    /// This operation removes the current shex_schema
    pub fn reset_shex(&mut self) {
        self.shex_schema = None;
        self.shex_validator = None
    }

    /// Reads a SHACL schema from a reader
    /// - `base` is used to resolve relative IRIs
    /// - `format` indicates the Shacl format
    pub fn read_shacl<R: io::Read>(
        &mut self,
        reader: &mut R,
        reader_name: &str,
        format: &ShaclFormat,
        base: Option<&str>,
        reader_mode: &ReaderMode,
    ) -> Result<()> {
        let format = match format {
            ShaclFormat::Internal => Err(RudofError::InternalSHACLFormatNonReadable),
            ShaclFormat::Turtle => Ok(RDFFormat::Turtle),
            ShaclFormat::NTriples => Ok(RDFFormat::NTriples),
            ShaclFormat::RdfXml => Ok(RDFFormat::Rdfxml),
            ShaclFormat::TriG => Ok(RDFFormat::TriG),
            ShaclFormat::N3 => Ok(RDFFormat::N3),
            ShaclFormat::NQuads => Ok(RDFFormat::NQuads),
            ShaclFormat::JsonLd => Ok(RDFFormat::JsonLd),
        }?;

        let rdf_graph = InMemoryGraph::from_reader(reader, reader_name, &format, base, reader_mode).map_err(|e| {
            RudofError::ReadingSHACLError {
                reader: reader_name.to_string(),
                error: e.to_string(),
                format: format.to_string(),
            }
        })?;

        let rdf_data = RdfData::from_graph(rdf_graph).map_err(|e| RudofError::ReadingSHACLFromGraphError {
            error: e.to_string(),
            format: format.to_string(),
            reader_name: reader_name.to_string(),
        })?;

        let schema = shacl_schema_from_data(rdf_data)?;

        self.shacl_schema = Some(schema);
        Ok(())
    }

    /// Run a SPARQL query against a remote endpoint
    /// - `query` is the SPARQL query to be executed
    /// - `endpoint` is the name or the URL of the SPARQL endpoint
    ///   Rudof keeps a cache of endpoints registered with some common names like `wikidata`, `dbpedia`, etc. to make it easier to use them. `
    ///   Returns the results as QuerySolutions
    pub fn run_query_endpoint(&mut self, query: &str, endpoint: &str) -> Result<QuerySolutions<RdfData>> {
        /*let iri_endpoint =
        IriS::from_str(endpoint).map_err(|e| RudofError::InvalidEndpointIri {
            endpoint: endpoint.to_string(),
            error: format!("{e}"),
        })?;*/
        let (name, sparql_endpoint) = self.get_endpoint(endpoint).map_err(|e| RudofError::InvalidEndpoint {
            endpoint: endpoint.to_string(),
            error: format!("{e}"),
        })?;

        let rdf_data = RdfData::from_endpoint(name.as_str(), sparql_endpoint);
        let solutions = rdf_data
            .query_select(query)
            .map_err(|e| RudofError::QueryEndpointError {
                endpoint: endpoint.to_string(),
                query: query.to_string(),
                error: format!("{e}"),
            })?;
        Ok(solutions)
    }

    /// Get an endpoint by its name or URL
    /// - `name` is the name or URL of the SPARQL endpoint
    ///   Returns the corresponding `SRDFSparql` instance if found or creates a new one if `name` is a valid URL
    ///   If a new endpoint is created, its name will be the same as the URL
    ///   It sets the endpoint for its usage in the next queries
    pub fn get_endpoint(&self, name: &str) -> Result<(String, SparqlEndpoint)> {
        // First, try to find the endpoint by its name in the registered endpoints
        match self.rdf_data.find_endpoint(name) {
            Some(endpoint) => Ok((name.to_string(), endpoint)),
            None => {
                // If not found, try to parse the name as an IRI
                let iri = IriS::from_str(name).map_err(|e| RudofError::InvalidEndpointIri {
                    endpoint: name.to_string(),
                    error: format!("{e}"),
                })?;
                // Create a new SRDFSparql instance with the given IRI
                let endpoint =
                    SparqlEndpoint::new(&iri, &PrefixMap::new()).map_err(|e| RudofError::InvalidEndpoint {
                        endpoint: name.to_string(),
                        error: format!("{e}"),
                    })?;
                Ok((name.to_string(), endpoint))
            },
        }
    }

    /// Reads a `DCTAP` and replaces the current one
    /// - `format` indicates the DCTAP format
    pub fn read_dctap<R: io::Read>(&mut self, reader: R, format: &DCTAPFormat) -> Result<()> {
        let dctap = match format {
            DCTAPFormat::Csv => {
                let dctap = DCTAP::from_reader(reader, &self.config.tap_config())
                    .map_err(|e| RudofError::DCTAPReaderCSVReader { error: format!("{e}") })?;
                Ok(dctap)
            },
            DCTAPFormat::Xls | DCTAPFormat::Xlsb | DCTAPFormat::Xlsm | DCTAPFormat::Xlsx => {
                Err(RudofError::DCTAPReadXLSNoPath)
            },
        }?;
        self.dctap = Some(dctap);
        Ok(())
    }

    /// Reads a `DCTAP` and replaces the current one
    /// - `format` indicates the DCTAP format
    pub fn read_dctap_path<P: AsRef<Path>>(&mut self, path: P, format: &DCTAPFormat) -> Result<()> {
        let path_name = path.as_ref().display().to_string();
        let dctap = match format {
            DCTAPFormat::Csv => {
                let dctap = DCTAP::from_path(path, &self.config.tap_config()).map_err(|e| RudofError::DCTAPReader {
                    path: path_name,
                    format: format.to_string(),
                    error: format!("{e}"),
                })?;
                Ok::<DCTAP, RudofError>(dctap)
            },
            DCTAPFormat::Xls | DCTAPFormat::Xlsb | DCTAPFormat::Xlsm | DCTAPFormat::Xlsx => {
                let path_buf = path.as_ref().to_path_buf();
                let dctap = DCTAP::from_excel(path_buf, None, &self.config.tap_config()).map_err(|e| {
                    RudofError::DCTAPReaderPathXLS {
                        path: path_name,
                        error: format!("{e}"),
                        format: format!("{format:?}"),
                    }
                })?;
                Ok(dctap)
            },
        }?;
        self.dctap = Some(dctap);
        Ok(())
    }

    pub fn read_rdf_config<R: io::Read>(&mut self, reader: R, source_name: String) -> Result<()> {
        let rdf_config = rdf_config::RdfConfigModel::from_reader(reader, source_name)
            .map_err(|e| RudofError::RdfConfigReadError { error: format!("{e}") })?;
        self.rdf_config = Some(rdf_config);
        Ok(())
    }

    /// Reads a `SparqlQuery` and replaces the current one
    pub fn read_query<R: io::Read>(&mut self, reader: R, source_name: Option<&str>) -> Result<()> {
        use std::io::Read;
        let mut str = String::new();
        let mut buf_reader = BufReader::new(reader);
        buf_reader
            .read_to_string(&mut str)
            .map_err(|e| RudofError::ReadError { error: format!("{e}") })?;
        let query = SparqlQuery::new(&str).map_err(|e| RudofError::SparqlSyntaxError {
            error: format!("{e}"),
            source_name: source_name.unwrap_or("source without name").to_string(),
        })?;
        self.sparql_query = Some(query);
        Ok(())
    }

    // Runs the current SPARQL query if it is a SELECT query
    // Returns the result as QuerySolutions
    // If the current query is not a SELECT query, returns an error
    pub fn run_current_query_select(&mut self) -> Result<QuerySolutions<RdfData>> {
        if let Some(sparql_query) = &self.sparql_query {
            if sparql_query.is_select() {
                self.run_query_select_str(&sparql_query.to_string())
            } else {
                Err(RudofError::NotSelectQuery {
                    query: sparql_query.to_string(),
                })
            }
        } else {
            Err(RudofError::NoCurrentSPARQLQuery)
        }
    }

    /// Runs the current SPARQL query if it is a CONSTRUCT query
    /// Returns the result serialized according to `format`
    /// If the current query is not a CONSTRUCT query, returns an error
    pub fn run_current_query_construct(&mut self, format: &QueryResultFormat) -> Result<String> {
        if let Some(sparql_query) = &self.sparql_query {
            if sparql_query.is_construct() {
                self.run_query_construct_str(&sparql_query.to_string(), format)
            } else {
                Err(RudofError::NotConstructQuery {
                    query: sparql_query.to_string(),
                })
            }
        } else {
            Err(RudofError::NoCurrentSPARQLQuery)
        }
    }

    /// Reads a `ShExSchema` and replaces the current one
    /// It also updates the current ShEx validator with the new ShExSchema
    /// - `base` is used to resolve relative IRIs
    /// - `format` indicates the ShEx format according to [`ShExFormat`](https://docs.rs/shex_validation/latest/shex_validation/shex_format/enum.ShExFormat.html)
    pub fn read_shex<R: io::Read>(
        &mut self,
        reader: R,
        format: &ShExFormat,
        base: Option<&str>,
        reader_mode: &ReaderMode,
        source_name: Option<&str>,
    ) -> Result<()> {
        let schema_ast = self.read_shex_only(reader, format, base, reader_mode, source_name)?;
        self.shex_schema = Some(schema_ast.clone());
        trace!("Schema AST read: {schema_ast}");
        let mut schema = SchemaIR::new();
        trace!("Compiling schema");
        let base_iri = if let Some(base) = base {
            Some(IriS::from_str(base).map_err(|e| RudofError::BaseIriError {
                str: base.to_string(),
                error: format!("{e}"),
            })?)
        } else {
            None
        };
        schema
            .populate_from_schema_json(&schema_ast, &ResolveMethod::default(), &base_iri)
            .map_err(|e| RudofError::CompilingSchemaError { error: format!("{e}") })?;
        trace!("Schema compiled and storing it in schema_ir: {schema:?}");
        trace!("Displaying schema_ir: {}", schema);
        self.shex_schema_ir = Some(schema.clone());
        trace!("Schema_ir cloned, preparing validator...");
        let validator = ShExValidator::new(schema, &self.config.validator_config()).map_err(|e| {
            RudofError::ShExValidatorCreationError {
                error: format!("{e}"),
                schema: format!("{schema_ast}"),
            }
        })?;
        trace!("Validator created");
        self.shex_validator = Some(validator);
        Ok(())
    }

    /// Reads a ShEx schema without storing it in the current shex_schema
    pub fn read_shex_only<R: io::Read>(
        &mut self,
        reader: R,
        format: &ShExFormat,
        base: Option<&str>,
        _reader_mode: &ReaderMode,
        source_name: Option<&str>,
    ) -> Result<ShExSchema> {
        match format {
            ShExFormat::ShExC => {
                let base = match base {
                    Some(str) => {
                        let iri = IriS::from_str(str).map_err(|e| RudofError::BaseIriError {
                            str: str.to_string(),
                            error: format!("{e}"),
                        })?;
                        Ok::<Option<IriS>, RudofError>(Some(iri))
                    },
                    None => Ok(None),
                }?;

                let source_iri = match source_name {
                    #[cfg(target_family = "wasm")]
                    Some(_) => Err(RudofError::WASMError(
                        "Reading ShExC from a source with a name is not supported in WASM".to_string(),
                    )),
                    #[cfg(not(target_family = "wasm"))]
                    Some(name) => {
                        let cwd =
                            env::current_dir().map_err(|e| RudofError::CurrentDirError { error: format!("{e}") })?;
                        trace!("Current directory: {}", cwd.display());
                        // Note: we use from_directory_path to convert a directory to a file URL that ends with a trailing slash
                        // from_url_path would not add the trailing slash and would fail when resolving relative IRIs
                        let url =
                            Url::from_directory_path(&cwd).map_err(|_| RudofError::ConvertingCurrentFolderUrl {
                                current_dir: cwd.to_string_lossy().to_string(),
                            })?;
                        trace!("Current directory as URL: {}", url);
                        let iri = IriS::from_str_base(name, Some(url.as_str())).map_err(|e| {
                            RudofError::SourceNameIriError {
                                source_name: name.to_string(),
                                error: e.to_string(),
                            }
                        })?;
                        Ok::<IriS, RudofError>(iri)
                    },
                    None => Ok(iri!("http://default/")),
                }?;
                let schema_json =
                    ShExParser::from_reader(reader, base, &source_iri).map_err(|e| RudofError::ShExCParserError {
                        error: format!("{e}"),
                        source_name: source_name.unwrap_or("source without name").to_string(),
                    })?;
                Ok(schema_json)
            },
            ShExFormat::ShExJ => {
                let schema_json = ShExSchema::from_reader(reader).map_err(|e| RudofError::ShExJParserError {
                    error: format!("{e}"),
                    source_name: source_name.unwrap_or("source without name").to_string(),
                })?;
                Ok(schema_json)
            },
            ShExFormat::RDFFormat(_) => {
                todo!()
                /*let rdf = parse_data(
                    &vec![input.clone()],
                    &DataFormat::Turtle,
                    reader_mode,
                    &config.rdf_config(),
                )?;
                let schema = ShExRParser::new(rdf).parse()?;
                Ok(schema) */
            },
        }
    }

    pub fn read_service_description<R: io::Read>(
        &mut self,
        reader: &mut R,
        source_name: &str,
        format: &RDFFormat,
        base: Option<&str>,
        reader_mode: &ReaderMode,
    ) -> Result<()> {
        let service_description = ServiceDescription::from_reader(reader, source_name, format, base, reader_mode)
            .map_err(|e| RudofError::ReadingServiceDescription { error: format!("{e}") })?;
        self.service_description = Some(service_description);
        Ok(())
    }

    pub fn read_service_description_file<P: AsRef<Path>>(
        &mut self,
        path: P,
        format: &RDFFormat,
        base: Option<&str>,
        reader_mode: &ReaderMode,
    ) -> Result<()> {
        let file = File::open(path.as_ref()).map_err(|e| RudofError::ReadingServiceDescriptionPath {
            path: path.as_ref().to_string_lossy().to_string(),
            error: format!("{e}"),
        })?;
        let mut reader = BufReader::new(file);
        self.read_service_description(
            &mut reader,
            path.as_ref().display().to_string().as_str(),
            format,
            base,
            reader_mode,
        )
    }

    pub fn read_service_description_url(
        &mut self,
        url_str: &str,
        format: &RDFFormat,
        base: Option<&str>,
        reader_mode: &ReaderMode,
    ) -> Result<()> {
        let url_spec = UrlSpec::parse(url_str).map_err(|e| RudofError::ParsingUrlReadingServiceDescriptionUrl {
            url: url_str.to_string(),
            error: format!("{e}"),
        })?;
        let url_spec = InputSpec::Url(url_spec);
        let mut reader = url_spec
            .open_read(Some("text/turtle"), "Reading service description")
            .map_err(|e| RudofError::ReadingServiceDescriptionUrl {
                url: url_str.to_string(),
                error: format!("{e}"),
            })?;
        self.read_service_description(&mut reader, url_str, format, base, reader_mode)
    }

    pub fn serialize_service_description<W: io::Write>(
        &self,
        format: &ServiceDescriptionFormat,
        writer: &mut W,
    ) -> Result<()> {
        if let Some(service_description) = &self.service_description {
            service_description
                .serialize(format, writer)
                .map_err(|e| RudofError::SerializingServiceDescription { error: format!("{e}") })
        } else {
            Err(RudofError::NoServiceDescriptionToSerialize)
        }
    }

    /// Validate RDF data using SHACL
    ///
    /// mode: Indicates whether to use SPARQL or native Rust implementation
    /// shapes_graph_source: Indicates the source of the shapes graph,
    /// which can be extracted from the current RDF data,
    /// or from the current SHACL schema.
    /// If there is no current SHACL schema, it tries to get it from the current RDF data
    pub fn validate_shacl(
        &mut self,
        mode: &ShaclValidationMode,
        shapes_graph_source: &ShapesGraphSource,
    ) -> Result<ValidationReport> {
        self.compile_shacl(shapes_graph_source)?;
        let compiled_schema = self.shacl_schema_ir.as_ref().ok_or(RudofError::NoShaclSchema {})?;
        let shacl_schema = self.shacl_schema.as_ref().ok_or(RudofError::NoShaclSchema {})?;
        let mut validator = GraphValidation::from_graph(Graph::from_data(self.rdf_data.clone()), *mode);
        let result = ShaclProcessor::validate(&mut validator, compiled_schema).map_err(|e| {
            RudofError::SHACLValidationError {
                error: format!("{e}"),
                schema: Box::new(shacl_schema.to_owned()),
            }
        })?;
        Ok(result)
    }

    /// Compiles the current SHACL schema to an internal representation
    pub fn compile_shacl(&mut self, shapes_graph_source: &ShapesGraphSource) -> Result<()> {
        let (compiled_schema, ast_schema) = match shapes_graph_source {
            ShapesGraphSource::CurrentSchema if self.shacl_schema.is_some() => {
                let ast_schema = self.shacl_schema.as_ref().unwrap();
                let compiled_schema =
                    ShaclSchemaIR::compile(ast_schema).map_err(|e| RudofError::SHACLCompilationError {
                        error: e.to_string(),
                        schema: Box::new(ast_schema.clone()),
                    })?;
                Ok::<(shacl_ir::schema_ir::SchemaIR, shacl_ast::ShaclSchema<RdfData>), RudofError>((
                    compiled_schema,
                    ast_schema.clone(),
                ))
            },
            // If self.shacl_schema is None or shapes_graph_source is CurrentData
            // We extract the SHACL schema from the current RDF data
            _ => {
                let ast_schema = shacl_schema_from_data(self.rdf_data.clone())?;
                let compiled_schema =
                    ShaclSchemaIR::compile(&ast_schema).map_err(|e| RudofError::SHACLCompilationError {
                        error: e.to_string(),
                        schema: Box::new(ast_schema.clone()),
                    })?;
                Ok((compiled_schema, ast_schema))
            },
        }?;
        self.shacl_schema = Some(ast_schema);
        self.shacl_schema_ir = Some(compiled_schema);
        Ok(())
    }

    /// Validate RDF data using ShEx
    /// It uses a ShEx validator which has a corrent ShEx schema and the current ShapeMap
    pub fn validate_shex(&mut self) -> Result<ResultShapeMap> {
        // We initialize the store in case the SPARQL based node selectors need to do SPARQL queries
        self.rdf_data
            .check_store()
            .map_err(|e| RudofError::StorageError { error: format!("{e}") })?;
        let schema_str = format!("{:?}", self.shex_validator);
        match self.shex_validator {
            None => Err(RudofError::ShExValidatorUndefined {}),
            Some(ref mut validator) => match &self.shapemap {
                None => Err(RudofError::NoShapeMap { schema: schema_str }),
                Some(shapemap) => {
                    let schema = validator.schema().clone();
                    let result = validator
                        .validate_shapemap2(
                            shapemap,
                            &self.rdf_data,
                            &schema,
                            &Some(self.rdf_data.prefixmap_in_memory()),
                        )
                        .map_err(|e| RudofError::ShExValidatorError {
                            schema: schema_str.clone(),
                            rdf_data: format!("{:?}", self.rdf_data),
                            query_map: format!("{shapemap:?}"),
                            error: format!("{e}"),
                        })?;
                    Ok(result.clone())
                },
            },
        }
    }

    /// Adds an endpoint to the current RDF data
    /// - `name` is the name of the endpoint
    /// - `iri` is the IRI of the endpoint
    /// - `prefixmap` is the prefix map to be used with the endpoint
    ///   If an endpoint with the same name already exists, it is replaced
    pub fn add_endpoint(&mut self, name: &str, iri: &IriS, prefixmap: &PrefixMap) -> Result<()> {
        let sparql_endpoint = SparqlEndpoint::new(iri, prefixmap).map_err(|e| RudofError::AddingEndpointError {
            iri: iri.clone(),
            error: format!("{e}"),
        })?;
        self.rdf_data.add_endpoint(name, sparql_endpoint);
        Ok(())
    }

    /// Parses an RDF graph from a reader and merges it with the current graph
    pub fn read_data<R: io::Read>(
        &mut self,
        reader: &mut R,
        source_name: &str,
        format: &RDFFormat,
        base: Option<&str>,
        reader_mode: &ReaderMode,
        merge: bool,
    ) -> Result<()> {
        if !merge {
            self.rdf_data = RdfData::new();
        }
        self.rdf_data
            .merge_from_reader(reader, source_name, format, base, reader_mode)
            .map_err(|e| RudofError::MergeRDFDataFromReader {
                source_name: source_name.to_string(),
                format: format!("{format:?}"),
                base: format!("{base:?}"),
                reader_mode: format!("{reader_mode:?}"),
                error: format!("{e}"),
            })?;
        Ok(())
    }

    /// Cleans the in-memory graph
    pub fn clean_rdf_graph(&mut self) {
        self.rdf_data.clean_graph();
    }

    /// Add a pair of node selector and shape selector to the current shapemap
    pub fn shapemap_add_node_shape_selectors(&mut self, node: NodeSelector, shape: ShapeSelector) {
        match &mut self.shapemap {
            None => {
                let mut shapemap = QueryShapeMap::new();
                shapemap.add_association(node, shape);
                self.shapemap = Some(shapemap)
            },
            Some(sm) => {
                sm.add_association(node, shape);
            },
        };
    }

    /// Read a shapemap
    pub fn read_shapemap<R: io::Read>(
        &mut self,
        mut reader: R,
        reader_name: &str,
        shapemap_format: &ShapeMapFormat,
    ) -> Result<()> {
        let mut v = Vec::new();
        reader
            .read_to_end(&mut v)
            .map_err(|e| RudofError::ReadError { error: format!("{e}") })?;
        let s = String::from_utf8(v).map_err(|e| RudofError::Utf8Error { error: format!("{e}") })?;
        let shapemap = match shapemap_format {
            ShapeMapFormat::Compact => {
                let shapemap =
                    ShapeMapParser::parse(s.as_str(), &Some(self.nodes_prefixmap()), &self.shex_shapes_prefixmap())
                        .map_err(|e| RudofError::ShapeMapParseError {
                            source_name: reader_name.to_string(),
                            str: s.to_string(),
                            error: format!("{e}"),
                        })?;
                Ok::<QueryShapeMap, RudofError>(shapemap)
            },
            _ => todo!(),
        }?;
        self.shapemap = Some(shapemap);
        Ok(())
    }

    pub fn reset_shapemap(&mut self) {
        self.shapemap = None
    }

    /// Returns the RDF data prefixmap
    pub fn nodes_prefixmap(&self) -> PrefixMap {
        self.rdf_data.prefixmap().unwrap_or_default()
    }

    /// Returns the shapes prefixmap
    ///
    /// If no ShEx schema has been set, returns None
    pub fn shex_shapes_prefixmap(&self) -> Option<PrefixMap> {
        self.shex_validator
            .as_ref()
            .map(|validator| validator.shapes_prefixmap())
    }

    /// Get current RDF Data
    pub fn get_rdf_data(&self) -> &RdfData {
        &self.rdf_data
    }

    #[allow(clippy::too_many_arguments)]
    pub fn get_coshamo(
        &mut self,
        reader: &mut dyn std::io::Read,
        mode: &InputCompareMode,
        format: &InputCompareFormat,
        base: Option<&str>,
        reader_mode: &ReaderMode,
        label: Option<&str>,
        source_name: Option<&str>,
    ) -> Result<CoShaMo> {
        let comparator_config = self.config().comparator_config();
        match mode {
            InputCompareMode::Shacl => Err(RudofError::NotImplemented {
                msg: "Not yet implemented comparison between SHACL schemas".to_string(),
            }),
            InputCompareMode::ShEx => {
                let shex_format = format
                    .to_shex_format()
                    .map_err(|e| RudofError::InvalidCompareSchemaFormat {
                        format: format!("{format:?}"),
                        error: e.to_string(),
                    })?;
                let shex = self.read_shex_only(reader, &shex_format, base, reader_mode, source_name)?;
                let mut converter = CoShaMoConverter::new(&comparator_config);
                let coshamo =
                    converter
                        .populate_from_shex(&shex, label)
                        .map_err(|e| RudofError::CoShaMoFromShExError {
                            schema: format!("{shex:?}"),
                            error: e.to_string(),
                        })?;
                Ok(coshamo)
            },
            InputCompareMode::Service => Err(RudofError::NotImplemented {
                msg: "Not yet implemented comparison between Service descriptions".to_string(),
            }),
            InputCompareMode::Dctap => Err(RudofError::NotImplemented {
                msg: "Not yet implemented comparison between DCTAP files".to_string(),
            }),
        }
    }

    /// Serializes the current RDF Config to a writer
    pub fn serialize_rdf_config<W: io::Write>(&self, format: &RdfConfigResultFormat, writer: &mut W) -> Result<()> {
        if let Some(rdf_config) = &self.rdf_config {
            rdf_config
                .serialize(cnv_rdf_config_format(format), writer)
                .map_err(|e| RudofError::SerializingRdfConfig { error: format!("{e}") })
        } else {
            writeln!(writer, "{{\"error\": \"No RDF Config read\"}}")?;
            Ok(())
        }
    }

    /// Returns the base IRI for the current context.
    ///
    /// If a base IRI is explicitly provided, it is returned.
    /// Otherwise, if a base IRI is set in the ShEx config, it is returned.
    /// If neither is available, an error is returned (depending on WASM environment).
    pub fn get_base_iri(&self, base_iri: &Option<IriS>) -> Result<IriS> {
        if let Some(base_iri) = base_iri {
            Ok(base_iri.clone())
        } else if let Some(base_iri) = self.config.shex_config().base.as_ref() {
            Ok(base_iri.clone())
        } else {
            #[cfg(target_family = "wasm")]
            return Err(RudofError::WASMError(
                "Base IRI must be provided in WASM environment".to_string(),
            ));
            #[cfg(not(target_family = "wasm"))]
            {
                let cwd = env::current_dir().map_err(|e| RudofError::CurrentDirError { error: format!("{e}") })?;
                // Note: we use from_directory_path to convert a directory to a file URL that ends with a trailing slash
                // from_url_path would not add the trailing slash and would fail when resolving relative IRIs
                let url = Url::from_directory_path(&cwd).map_err(|_| RudofError::ConvertingCurrentFolderUrl {
                    current_dir: cwd.to_string_lossy().to_string(),
                })?;
                Ok(url.into())
            }
        }
    }

    pub fn parse_shape_selector(&self, label_str: &str) -> Result<ShapeSelector> {
        let selector =
            ShapeMapParser::parse_shape_selector(label_str).map_err(|e| RudofError::ShapeSelectorParseError {
                shape_selector: label_str.to_string(),
                error: e.to_string(),
            })?;
        Ok(selector)
    }

    pub fn load_data(
        &mut self,
        data: &[InputSpec],
        data_format: &DataFormat,
        base: &Option<IriS>,
        endpoint: &Option<String>,
        reader_mode: &ReaderMode,
        allow_no_data: bool,
    ) -> Result<()> {
        match (data.is_empty(), endpoint) {
            (true, None) => {
                if allow_no_data {
                    self.reset_data();
                    Ok(())
                } else {
                    Err(RudofError::MissingDataAndEndpoint)
                }
            },
            (false, None) => {
                let rdf_format: RDFFormat = data_format
                    .try_into()
                    .map_err(|e| RudofError::DataFormatError { error: e })?;

                for data_input in data {
                    let mut data_reader = data_input
                        .open_read(Some(data_format.mime_type()), "RDF data")
                        .map_err(|e| RudofError::RDFDataReadError {
                            source_name: data_input.source_name(),
                            mime_type: data_format.mime_type().to_string(),
                            error: e.to_string(),
                        })?;

                    let base = self.get_base_iri(base)?;
                    self.read_data(
                        &mut data_reader,
                        data_input.source_name().as_str(),
                        &rdf_format,
                        Some(base.as_str()),
                        reader_mode,
                        true,
                    )?;
                }
                Ok(())
            },
            (true, Some(endpoint)) => {
                let (name, _) = self.get_endpoint(endpoint)?;
                self.use_endpoint(name.as_str())
            },
            (false, Some(_)) => Err(RudofError::BothDataAndEndpointSpecified),
        }
    }

    /// Loads a ShEx schema from an InputSpec.
    ///
    /// This is a high-level method that handles:
    /// - Opening the input source
    /// - Determining the schema format
    /// - Resolving the base IRI
    /// - Parsing the schema
    ///
    /// # Arguments
    /// * `input` - The input specification (file, URL, or stdin)
    /// * `schema_format` - Optional format (defaults to ShExC)
    /// * `base` - Optional base IRI (uses config or current dir if not provided)
    /// * `reader_mode` - Reader mode for parsing
    pub fn load_shex_schema(
        &mut self,
        input: &InputSpec,
        schema_format: &Option<ShExFormat>,
        base: &Option<IriS>,
        reader_mode: &ReaderMode,
    ) -> Result<()> {
        use iri_s::MimeType;

        let schema_format = schema_format.clone().unwrap_or(ShExFormat::ShExC);

        // Open the schema reader
        let schema_reader = input
            .open_read(Some(schema_format.mime_type()), "ShEx Schema")
            .map_err(|e| RudofError::ReadingPathContext {
                path: input.source_name().to_string(),
                error: e.to_string(),
                context: "ShEx Schema".to_string(),
            })?;

        // Resolve base IRI
        let base_iri = self.get_base_iri(base)?;

        // Read the schema
        self.read_shex(
            schema_reader,
            &schema_format,
            Some(base_iri.as_str()),
            reader_mode,
            Some(&input.source_name()),
        )?;

        Ok(())
    }

    /// Loads a shapemap from an InputSpec.
    ///
    /// # Arguments
    /// * `input` - The input specification for the shapemap
    /// * `shapemap_format` - The format of the shapemap
    pub fn load_shapemap(&mut self, input: &InputSpec, shapemap_format: &ShapeMapFormat) -> Result<()> {
        let shapemap_reader = input
            .open_read(None, "ShapeMap")
            .map_err(|e| RudofError::ShapeMapParseError {
                source_name: input.source_name(),
                str: input.source_name().to_string(),
                error: e.to_string(),
            })?;

        self.read_shapemap(shapemap_reader, input.source_name().as_str(), shapemap_format)?;

        Ok(())
    }

    /// Adds a node and shape selector to the current shapemap.
    ///
    /// This method handles the logic of combining node and shape selectors:
    /// - If only node is provided, uses START shape
    /// - If both are provided, uses the specified shape
    /// - If neither is provided, does nothing
    /// - If only shape is provided, logs a warning (shape without node is ignored)
    ///
    /// # Arguments
    /// * `node` - Optional node selector string
    /// * `shape` - Optional shape selector string
    pub fn add_node_shape_to_shapemap(&mut self, node: &Option<String>, shape: &Option<String>) -> Result<()> {
        match (node, shape) {
            (None, None) => {
                // Nothing to do
                Ok(())
            },
            (Some(node_str), None) => {
                let node_selector = crate::parse_node_selector(node_str)?;
                let shape_selector = crate::selector::start();
                self.shapemap_add_node_shape_selectors(node_selector, shape_selector);
                Ok(())
            },
            (Some(node_str), Some(shape_str)) => {
                let node_selector = crate::parse_node_selector(node_str)?;
                let shape_selector = crate::parse_shape_selector(shape_str)?;
                self.shapemap_add_node_shape_selectors(node_selector, shape_selector);
                Ok(())
            },
            (None, Some(shape_str)) => {
                tracing::debug!("Shape label {shape_str} ignored because no node selector was provided");
                Ok(())
            },
        }
    }

    /// Performs complete ShEx validation workflow.
    ///
    /// This is a high-level method that:
    /// 1. Loads the ShEx schema (if not already loaded)
    /// 2. Loads the shapemap (if provided)
    /// 3. Adds individual node/shape pairs (if provided)
    /// 4. Performs the validation
    ///
    /// # Arguments
    /// * `schema` - Optional schema input (if None, uses already loaded schema)
    /// * `schema_format` - Schema format
    /// * `base_schema` - Base IRI for schema resolution
    /// * `reader_mode` - Reader mode for parsing
    /// * `shapemap` - Optional shapemap input
    /// * `shapemap_format` - Shapemap format
    /// * `node` - Optional node selector
    /// * `shape` - Optional shape selector
    ///
    /// # Returns
    /// The validation result as a ResultShapeMap
    #[allow(clippy::too_many_arguments)]
    pub fn validate_shex_complete(
        &mut self,
        schema: &Option<InputSpec>,
        schema_format: &Option<ShExFormat>,
        base_schema: &Option<IriS>,
        reader_mode: &ReaderMode,
        shapemap: &Option<InputSpec>,
        shapemap_format: &ShapeMapFormat,
        node: &Option<String>,
        shape: &Option<String>,
    ) -> Result<ResultShapeMap> {
        // Load schema if provided
        if let Some(schema_input) = schema {
            self.load_shex_schema(schema_input, schema_format, base_schema, reader_mode)?;
        }

        // Load shapemap if provided
        if let Some(shapemap_input) = shapemap {
            self.load_shapemap(shapemap_input, shapemap_format)?;
        }

        // Add individual node/shape pair if provided
        self.add_node_shape_to_shapemap(node, shape)?;

        // Perform validation
        self.validate_shex()
    }

    /// Validates that the ShEx schema is well-formed.
    ///
    /// Specifically checks for negative cycles in shape dependencies,
    /// which would make the schema invalid.
    ///
    /// # Returns
    /// Ok if the schema is well-formed, Err if negative cycles are detected
    pub fn validate_shex_schema_well_formed(&self) -> Result<()> {
        if self.config.shex_config().check_well_formed()
            && let Some(shex_ir) = self.get_shex_ir()
            && shex_ir.has_neg_cycle()
        {
            return Err(RudofError::Generic {
                error: format!("Schema contains negative cycles: {:?}", shex_ir.neg_cycles()),
            });
        }
        Ok(())
    }

    /// Serializes a specific shape from the current ShEx schema.
    ///
    /// # Arguments
    /// * `shape_label` - String representation of the shape selector
    /// * `format` - Output format for serialization
    /// * `formatter` - Formatter for pretty-printing
    /// * `writer` - Output writer
    pub fn serialize_shape_by_label<W: io::Write>(
        &self,
        shape_label: &str,
        format: &ShExFormat,
        formatter: &shex_ast::compact::ShExFormatter,
        writer: &mut W,
    ) -> Result<()> {
        let shape_selector = self.parse_shape_selector(shape_label)?;
        self.serialize_shape_current_shex(&shape_selector, format, formatter, writer)?;
        Ok(())
    }

    /// Gets statistics about the ShEx schema's internal representation.
    ///
    /// Returns information such as:
    /// - Number of shapes with N extends
    /// - Local vs total shape counts
    /// - Shape labels and their sources
    /// - Dependencies between shapes
    ///
    /// # Returns
    /// A ShExStatistics struct containing all relevant metrics
    pub fn get_shex_statistics(&self) -> Result<ShExStatistics> {
        let shex_ir = self.get_shex_ir().ok_or_else(|| RudofError::Generic {
            error: "Schema was not compiled to IR".to_string(),
        })?;

        Ok(ShExStatistics {
            extends_count: shex_ir.count_extends(),
            local_shapes_count: shex_ir.local_shapes_count(),
            total_shapes_count: shex_ir.total_shapes_count(),
            shapes: shex_ir
                .shapes()
                .map(|(l, s, e)| (l.clone(), s.clone(), e.clone()))
                .collect(),
            dependencies: shex_ir.dependencies(),
            has_imports: !shex_ir.imported_schemas().is_empty(),
            neg_cycles: shex_ir.neg_cycles(),
        })
    }

    /// Loads a SHACL schema from an InputSpec.
    ///
    /// This is a high-level method that handles:
    /// - Opening the input source
    /// - Determining the schema format
    /// - Resolving the base IRI
    /// - Parsing the schema
    ///
    /// # Arguments
    /// * `input` - The input specification (file, URL, or stdin)
    /// * `schema_format` - The SHACL format
    /// * `base` - Optional base IRI (uses config or current dir if not provided)
    /// * `reader_mode` - Reader mode for parsing
    pub fn load_shacl_schema(
        &mut self,
        input: &InputSpec,
        schema_format: &ShaclFormat,
        base: &Option<IriS>,
        reader_mode: &ReaderMode,
    ) -> Result<()> {
        // Open the schema reader
        let mime_type = schema_format.mime_type();
        let mut schema_reader =
            input
                .open_read(Some(mime_type), "SHACL shapes")
                .map_err(|e| RudofError::ReadingPathContext {
                    path: input.source_name().to_string(),
                    error: e.to_string(),
                    context: "SHACL Schema".to_string(),
                })?;

        // Resolve base IRI
        let base_iri = self.get_base_iri(base)?;

        // Read the SHACL schema
        self.read_shacl(
            &mut schema_reader,
            &input.source_name(),
            schema_format,
            Some(base_iri.as_str()),
            reader_mode,
        )?;

        Ok(())
    }

    /// Reads Property Graph data from a reader.
    ///
    /// # Arguments
    /// * `reader` - The reader to read PG data from
    /// * `_data_format` - The format of the PG data (currently unused, assumes PG format)
    ///
    /// # Returns
    /// A PropertyGraph parsed from the reader
    pub fn get_pg_data<R: io::Read>(&self, reader: &mut R, _data_format: &DataFormat) -> Result<PropertyGraph> {
        let mut data_content = String::new();
        reader.read_to_string(&mut data_content)?;
        let graph = match PgBuilder::new().parse_pg(data_content.as_str()) {
            Ok(graph) => graph,
            Err(e) => {
                return Err(RudofError::PGDataParseError {
                    source_name: "reader".to_string(),
                    error: format!("Failed to parse graph: {}", e),
                });
            },
        };
        Ok(graph)
    }

    /// Loads Property Graph data from multiple input sources.
    ///
    /// This is a high-level method that handles opening and merging multiple PG data sources.
    ///
    /// # Arguments
    /// * `data` - Slice of input specifications for PG data
    /// * `data_format` - Format of the PG data
    ///
    /// # Returns
    /// A merged PropertyGraph containing all data
    pub fn load_pg_data(&self, data: &[InputSpec], data_format: &DataFormat) -> Result<PropertyGraph> {
        let mut graph = PropertyGraph::new();

        for data_input in data {
            let mut data_reader =
                data_input
                    .open_read(None, "PG data")
                    .map_err(|e| RudofError::ReadingPathContext {
                        path: data_input.source_name().to_string(),
                        error: e.to_string(),
                        context: "PG Data".to_string(),
                    })?;

            let new_graph = self.get_pg_data(&mut data_reader, data_format)?;
            graph.merge(&new_graph);
        }

        Ok(graph)
    }

    /// Loads a PGSchema from an input source.
    ///
    /// # Arguments
    /// * `schema_input` - The input specification for the schema
    ///
    /// # Returns
    /// The parsed PgSchema
    pub fn load_pg_schema(&self, schema_input: &InputSpec) -> Result<PropertyGraphSchema> {
        let mut schema_reader =
            schema_input
                .open_read(None, "PGSchema")
                .map_err(|e| RudofError::ReadingPathContext {
                    path: schema_input.source_name().to_string(),
                    error: e.to_string(),
                    context: "PGSchema".to_string(),
                })?;

        self.read_pg_schema(&mut schema_reader)
            .map_err(|e| RudofError::PGSchemaParseError {
                source_name: schema_input.source_name(),
                error: format!("{e}"),
            })
    }

    /// Reads a Property Graph schema from a reader
    ///
    /// # Arguments
    /// * `reader` - The reader to read PG schema from
    ///
    /// # Returns
    /// The parsed PropertyGraphSchema
    pub fn get_schema<R: io::Read>(&self, reader: &mut R) -> Result<PropertyGraphSchema> {
        let mut schema_content = String::new();
        reader
            .read_to_string(&mut schema_content)
            .map_err(|e| RudofError::ReadError { error: format!("{e}") })?;
        let schema = match PgsBuilder::new().parse_pgs(schema_content.as_str()) {
            Ok(schema) => schema,
            Err(e) => {
                return Err(RudofError::PGSchemaParseError {
                    source_name: "reader".to_string(),
                    error: format!("Failed to parse schema: {}", e),
                });
            },
        };
        Ok(schema)
    }

    /// Reads a Property Graph type map from a reader
    ///
    /// # Arguments
    /// * `reader` - The reader to read type map from
    ///
    /// # Returns
    /// The parsed TypeMap
    pub fn get_map<R: io::Read>(&self, reader: &mut R) -> Result<TypeMap> {
        let mut map_content = String::new();
        reader
            .read_to_string(&mut map_content)
            .map_err(|e| RudofError::ReadError { error: format!("{e}") })?;
        let map = match MapBuilder::new().parse_map(map_content.as_str()) {
            Ok(map) => map,
            Err(e) => {
                return Err(RudofError::PGTypeMapParseError {
                    source_name: "reader".to_string(),
                    error: format!("Failed to parse type map: {}", e),
                });
            },
        };
        Ok(map)
    }

    /// Reads a PGSchema from a reader.
    fn read_pg_schema<R: io::Read>(&self, reader: &mut R) -> Result<PropertyGraphSchema> {
        // Delegate to existing get_schema logic from pgschema crate
        // Assuming get_schema is available from pgschema crate
        self.get_schema(reader).map_err(|e| RudofError::PGSchemaParseError {
            source_name: "reader".to_string(),
            error: format!("{e}"),
        })
    }

    /// Loads a TypeMap from an input source.
    ///
    /// # Arguments
    /// * `map_input` - The input specification for the type map
    ///
    /// # Returns
    /// The parsed TypeMap
    pub fn load_pg_typemap(&self, map_input: &InputSpec) -> Result<TypeMap> {
        let mut map_reader = map_input
            .open_read(None, "type map")
            .map_err(|e| RudofError::ReadingPathContext {
                path: map_input.source_name().to_string(),
                error: e.to_string(),
                context: "PG TypeMap".to_string(),
            })?;

        self.read_pg_typemap(&mut map_reader)
            .map_err(|e| RudofError::PGTypeMapParseError {
                source_name: map_input.source_name(),
                error: format!("{e}"),
            })
    }

    /// Reads a TypeMap from a reader.
    fn read_pg_typemap<R: io::Read>(&self, reader: &mut R) -> Result<TypeMap> {
        // Delegate to existing get_map logic from pgschema crate
        // Assuming get_map is available from pgschema crate
        self.get_map(reader).map_err(|e| RudofError::PGTypeMapParseError {
            source_name: "reader".to_string(),
            error: format!("{e}"),
        })
    }

    /// Validates Property Graph data against a PGSchema using a TypeMap.
    ///
    /// This is a high-level method that performs the complete validation workflow.
    ///
    /// # Arguments
    /// * `schema` - The PGSchema to validate against
    /// * `graph` - The PropertyGraph data to validate
    /// * `type_map` - The TypeMap defining the mapping
    ///
    /// # Returns
    /// The validation result
    pub fn validate_pgschema(
        &self,
        schema: &PropertyGraphSchema,
        graph: &PropertyGraph,
        type_map: &TypeMap,
    ) -> Result<ValidationResult> {
        type_map
            .validate(schema, graph)
            .map_err(|e| RudofError::PGSchemaValidationError { error: format!("{e}") })
    }

    /// Parses a node selector string into a NodeSelector.
    pub fn parse_node_selector(&self, node_str: &str) -> Result<NodeSelector> {
        parse_node_selector(node_str)
    }

    /// Gets node information for a given node selector.
    ///
    /// This is a high-level method that retrieves information about nodes
    /// in the current RDF data based on the selector and options.
    pub fn get_node_info(
        &self,
        node_selector: NodeSelector,
        predicates: &[String],
        options: &NodeInfoOptions,
    ) -> Result<Vec<NodeInfo<RdfData>>> {
        get_node_info(&self.rdf_data, node_selector, predicates, options)
            .map_err(|e| RudofError::NodeInfoError { error: format!("{e}") })
    }

    /// Formats node information to a writer.
    ///
    /// This handles the display formatting of node information including
    /// tree structures for outgoing/incoming arcs.
    pub fn format_node_info<W: io::Write>(
        &self,
        node_infos: &[NodeInfo<RdfData>],
        writer: &mut W,
        options: &NodeInfoOptions,
    ) -> Result<()> {
        format_node_info_list(node_infos, &self.rdf_data, writer, options)
            .map_err(|e| RudofError::NodeInfoFormatError { error: format!("{e}") })
    }

    /// High-level method to show node information.
    ///
    /// This encapsulates the complete workflow: parsing the node selector,
    /// retrieving node info, and writing formatted output.
    ///
    /// # Arguments
    /// * `node_str` - Node selector string (e.g., "http://example/node" or "prefix:local")
    /// * `predicates` - Optional predicates to filter outgoing arcs
    /// * `show_mode` - Mode for showing arcs (outgoing, incoming, or both)
    /// * `depth` - Depth for recursive traversal
    /// * `writer` - Output writer
    pub fn show_node_info<W: io::Write>(
        &self,
        node_str: &str,
        predicates: &[String],
        show_mode: &str, // "outgoing", "incoming", "both"
        depth: usize,
        writer: &mut W,
    ) -> Result<()> {
        // Parse node selector
        let node_selector = parse_node_selector(node_str).map_err(|e| RudofError::NodeSelectorParseError {
            node_selector: node_str.to_string(),
            error: format!("{e}"),
        })?;

        tracing::debug!("Node info with node selector: {node_selector:?}");

        // Build options from mode
        let options = NodeInfoOptions::from_mode_str(show_mode)
            .map_err(|e| RudofError::InvalidNodeInfoMode {
                mode: show_mode.to_string(),
                error: format!("{e}"),
            })?
            .with_depth(depth);

        // Get node info
        let node_infos = get_node_info(&self.rdf_data, node_selector, predicates, &options)
            .map_err(|e| RudofError::NodeInfoError { error: format!("{e}") })?;

        // Format output
        format_node_info_list(&node_infos, &self.rdf_data, writer, &options)
            .map_err(|e| RudofError::NodeInfoFormatError { error: format!("{e}") })?;

        Ok(())
    }

    /// High-level SHACL schema extraction from data.
    ///
    /// Extracts SHACL schema from current RDF data, optionally merging with
    /// an external schema file, and serializes to writer.
    ///
    /// # Arguments
    /// * `shapes` - Optional external schema to merge
    /// * `shapes_format` - Format of external schema
    /// * `base_shapes` - Base IRI for external schema
    /// * `reader_mode` - Reader mode for parsing external schema
    /// * `result_format` - Format for serializing result
    /// * `writer` - Output writer
    pub fn shacl_extract<W: io::Write>(
        &mut self,
        shapes: &Option<InputSpec>,
        shapes_format: &Option<ShaclFormat>,
        base_shapes: &Option<IriS>,
        reader_mode: &ReaderMode,
        result_format: &ShaclFormat,
        writer: &mut W,
    ) -> Result<()> {
        // Load external schema if provided
        if let Some(schema_input) = shapes {
            let fmt = shapes_format.clone().unwrap_or_default();
            self.load_shacl_schema(schema_input, &fmt, base_shapes, reader_mode)?;
            tracing::trace!("Compiling SHACL schema from shapes graph");
            self.compile_shacl(&ShapesGraphSource::CurrentSchema)?;
        } else {
            self.compile_shacl(&ShapesGraphSource::CurrentData)?;
        }

        self.serialize_shacl(result_format, writer)?;

        if tracing::enabled!(tracing::Level::DEBUG) {
            match self.get_shacl_ir() {
                Some(ir) => tracing::debug!("SHACL IR: {}", ir),
                None => tracing::debug!("No SHACL IR available"),
            }
        }

        Ok(())
    }

    /// High-level DCTAP read and serialize workflow.
    ///
    /// Reads a DCTAP file (CSV or Excel formats) and serializes it to the requested output format.
    ///
    /// # Arguments
    /// * `input` - Input source (file, URL, stdin, or string)
    /// * `format` - Input format (CSV, XLSX, XLSB, XLSM, XLS)
    /// * `result_format` - Output format (Internal or JSON)
    /// * `writer` - Output writer
    pub fn process_dctap<W: io::Write>(
        &mut self,
        input: &InputSpec,
        format: &DCTAPFormat,
        result_format: &DCTapResultFormat,
        writer: &mut W,
    ) -> Result<()> {
        // Read DCTAP based on format and input type
        self.read_dctap_input(input, format)?;

        // Serialize to requested output format
        if let Some(dctap) = self.get_dctap() {
            match result_format {
                DCTapResultFormat::Internal => {
                    writeln!(writer, "{dctap}")?;
                },
                DCTapResultFormat::Json => {
                    let json_str =
                        serde_json::to_string_pretty(&dctap).map_err(|e| RudofError::DCTapSerializationError {
                            format: "JSON".to_string(),
                            error: format!("{e}"),
                        })?;
                    writeln!(writer, "{json_str}")?;
                },
            }
            Ok(())
        } else {
            Err(RudofError::NoDCTAPData)
        }
    }

    /// Internal helper to read DCTAP from various input sources.
    /// Handles the format-specific logic (CSV vs Excel).
    pub fn read_dctap_input(&mut self, input: &InputSpec, format: &DCTAPFormat) -> Result<()> {
        match format {
            // CSV can be read from any InputSpec (including stdin, URL, string)
            DCTAPFormat::Csv => {
                let reader = input.open_read(None, "DCTAP").map_err(|e| RudofError::InputSpecError {
                    context: "DCTAP".to_string(),
                    error: format!("{e}"),
                })?;
                self.read_dctap(reader, format)?;
                Ok(())
            },
            // Excel formats require a file path (cannot read from stdin/URL/string)
            _ => match input {
                InputSpec::Path(path_buf) => {
                    self.read_dctap_path(path_buf, format)?;
                    Ok(())
                },
                InputSpec::Stdin => Err(RudofError::DCTapExcelFromStdin),
                InputSpec::Url(_) => Err(RudofError::DCTapExcelFromUrl),
                InputSpec::Str(_) => Err(RudofError::DCTapExcelFromString),
            },
        }
    }

    /// High-level data generation from schema.
    ///
    /// Generates RDF data based on a ShEx or SHACL schema.
    ///
    /// # Arguments
    /// * `schema` - Input schema source (must be a file path)
    /// * `schema_format` - Schema format (Auto, ShEx, or SHACL)
    /// * `entity_count` - Number of entities to generate
    /// * `output` - Optional output path override
    /// * `result_format` - Output RDF format
    /// * `seed` - Optional random seed for reproducibility
    /// * `parallel` - Optional number of parallel worker threads
    /// * `config_file` - Optional configuration file path
    pub async fn generate_data(
        &self,
        schema: &InputSpec,
        schema_format: &GenerateSchemaFormat,
        entity_count: usize,
        output: &Option<PathBuf>,
        result_format: &DataFormat,
        seed: Option<u64>,
        parallel: Option<usize>,
        config_file: &Option<PathBuf>,
    ) -> Result<()> {
        // Load or create configuration
        let mut config = if let Some(config_path) = config_file {
            if config_path.extension().and_then(|s| s.to_str()) == Some("toml") {
                GeneratorConfig::from_toml_file(config_path)
                    .map_err(|e| RudofError::GeneratorConfigError { error: format!("{e}") })?
            } else {
                GeneratorConfig::from_json_file(config_path)
                    .map_err(|e| RudofError::GeneratorConfigError { error: format!("{e}") })?
            }
        } else {
            GeneratorConfig::default()
        };

        // Apply CLI overrides
        config.generation.entity_count = entity_count;

        if let Some(output_path) = output {
            config.output.path = output_path.clone();
        }

        if let Some(seed_value) = seed {
            config.generation.seed = Some(seed_value);
        }

        if let Some(threads) = parallel {
            config.parallel.worker_threads = Some(threads);
        }

        // Determine output format (only Turtle and NTriples are supported)
        config.output.format = match result_format {
            DataFormat::Turtle | DataFormat::TriG | DataFormat::N3 => OutputFormat::Turtle,
            _ => OutputFormat::NTriples,
        };

        // Get schema path - must be a file path
        let schema_path = match schema {
            InputSpec::Path(path) => path.clone(),
            InputSpec::Stdin => {
                return Err(RudofError::GenerateSchemaFromStdin);
            },
            InputSpec::Url(url) => {
                return Err(RudofError::GenerateSchemaFromUrl { url: url.to_string() });
            },
            InputSpec::Str(s) => {
                return Err(RudofError::GenerateSchemaFromString { content: s.clone() });
            },
        };

        // Create generator
        let mut generator =
            DataGenerator::new(config).map_err(|e| RudofError::DataGeneratorCreationError { error: format!("{e}") })?;

        // Load schema based on format
        match schema_format {
            GenerateSchemaFormat::Auto => {
                generator
                    .load_schema_auto(&schema_path)
                    .await
                    .map_err(|e| RudofError::SchemaLoadError { error: format!("{e}") })?;
            },
            GenerateSchemaFormat::ShEx => {
                generator
                    .load_shex_schema(&schema_path)
                    .await
                    .map_err(|e| RudofError::SchemaLoadError { error: format!("{e}") })?;
            },
            GenerateSchemaFormat::Shacl => {
                generator
                    .load_shacl_schema(&schema_path)
                    .await
                    .map_err(|e| RudofError::SchemaLoadError { error: format!("{e}") })?;
            },
        }

        // Generate data
        generator
            .generate()
            .await
            .map_err(|e| RudofError::DataGenerationError { error: format!("{e}") })?;

        Ok(())
    }

    /// High-level query execution.
    ///
    /// Executes a SPARQL query against the current RDF data.
    pub fn execute_query<W: io::Write>(
        &mut self,
        query: &InputSpec,
        query_type: &QueryType,
        result_format: &ResultQueryFormat,
        writer: &mut W,
    ) -> Result<()> {
        execute_query(self, query, query_type, result_format, writer)
            .map_err(|e| RudofError::QueryExecutionError { error: format!("{e}") })
    }
}

fn cnv_rdf_config_format(format: &RdfConfigResultFormat) -> &rdf_config::RdfConfigFormat {
    match format {
        RdfConfigResultFormat::Yaml => &rdf_config::RdfConfigFormat::Yaml,
        RdfConfigResultFormat::Internal => &rdf_config::RdfConfigFormat::Yaml,
    }
}

fn shacl_schema_from_data<RDF: FocusRDF + Debug>(rdf_data: RDF) -> Result<ShaclSchema<RDF>> {
    let schema = ShaclParser::new(rdf_data)
        .parse()
        .map_err(|e| RudofError::SHACLParseError { error: format!("{e}") })?;
    Ok(schema)
}

fn shacl_format2rdf_format(shacl_format: &ShaclFormat) -> Result<RDFFormat> {
    match shacl_format {
        ShaclFormat::N3 => Ok(RDFFormat::N3),
        ShaclFormat::NQuads => Ok(RDFFormat::NQuads),
        ShaclFormat::NTriples => Ok(RDFFormat::NTriples),
        ShaclFormat::RdfXml => Ok(RDFFormat::Rdfxml),
        ShaclFormat::TriG => Ok(RDFFormat::TriG),
        ShaclFormat::Turtle => Ok(RDFFormat::Turtle),
        ShaclFormat::Internal => Err(RudofError::NoInternalFormatForRDF),
        ShaclFormat::JsonLd => Ok(RDFFormat::JsonLd),
    }
}

pub fn show_triple_exprs(idx: &ShapeLabelIdx, schema: &SchemaIR, writer: &mut impl io::Write) -> Result<()> {
    if let Some(triple_exprs) = schema.get_triple_exprs(idx) {
        if let Some(current) = triple_exprs.get(&None) {
            writeln!(
                writer,
                "    Current -> {}",
                current
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            )?;
        } else {
            writeln!(writer, "    Current -> None?")?;
        }
        if triple_exprs.len() > 1 {
            for (label, exprs) in triple_exprs.iter().filter(|(k, _)| k.is_some()) {
                writeln!(
                    writer,
                    "    {} -> {}",
                    label.as_ref().unwrap(),
                    exprs.iter().map(|t| t.to_string()).collect::<Vec<String>>().join(", ")
                )?;
            }
        }
        Ok(())
    } else {
        trace!("No triple expressions for shape {idx}");
        Ok(())
    }
}

// ============================================================================
// Supporting Types
// ============================================================================

/// Statistics and metadata about a compiled ShEx schema.
#[derive(Debug, Clone)]
pub struct ShExStatistics {
    /// Count of shapes grouped by number of extends clauses
    pub extends_count: std::collections::HashMap<usize, usize>,

    /// Number of locally defined shapes
    pub local_shapes_count: usize,

    /// Total number of shapes (including imported)
    pub total_shapes_count: usize,

    /// List of (label, source, expression) tuples for all shapes
    pub shapes: Vec<(ShapeLabel, IriS, shex_ast::ir::shape_expr::ShapeExpr)>,

    /// List of (source, positive/negative, target) dependency tuples
    pub dependencies: Vec<(ShapeLabel, shex_ast::ir::dependency_graph::PosNeg, ShapeLabel)>,

    /// Whether the schema has imported schemas
    pub has_imports: bool,

    /// List of negative cycles detected in the schema
    pub neg_cycles: Vec<Vec<(ShapeLabelIdx, ShapeLabelIdx, Vec<ShapeLabelIdx>)>>,
}

#[cfg(test)]
mod tests {
    use iri_s::iri;
    use shacl_ast::ShaclFormat;
    use shacl_validation::shacl_processor::ShaclValidationMode;
    use shex_ast::ShExFormat;
    use shex_ast::shapemap::ShapeMapFormat;
    use shex_ast::{Node, ir::shape_label::ShapeLabel};

    use crate::RudofConfig;
    use rudof_rdf::rdf_core::RDFFormat;
    use rudof_rdf::rdf_impl::ReaderMode;

    use super::Rudof;

    #[test]
    fn test_single_shex() {
        let data = r#"
        prefix : <http://example/>
        :x :p 2 .
        "#;
        let shex = r#"
        prefix : <http://example/>
        :S @:T
        :T { :p @:U }
        :U [ 2 ]
        "#;
        let shapemap = r#":x@:S"#;
        let mut rudof = Rudof::new(&RudofConfig::default_config().unwrap()).unwrap();
        rudof
            .read_data(
                &mut data.as_bytes(),
                "test",
                &RDFFormat::Turtle,
                None,
                &ReaderMode::Strict,
                false,
            )
            .unwrap();

        rudof
            .read_shex(
                shex.as_bytes(),
                &ShExFormat::ShExC,
                None,
                &ReaderMode::Strict,
                Some("test"),
            )
            .unwrap();
        rudof
            .read_shapemap(shapemap.as_bytes(), "Test", &ShapeMapFormat::default())
            .unwrap();
        let result = rudof.validate_shex().unwrap();
        let node = Node::iri(iri!("http://example/x"));
        let shape = ShapeLabel::iri(iri!("http://example/S"));
        assert!(result.get_info(&node, &shape).unwrap().is_conformant())
    }

    #[test]
    fn test_shex_validation_ok() {
        let data = r#"<http://example/x> <http://example/p> 23 ."#;
        let shex = r#"<http://example/S> { <http://example/p> . }"#;
        let shapemap = r#"<http://example/x>@<http://example/S>"#;
        let mut rudof = Rudof::new(&RudofConfig::default_config().unwrap()).unwrap();
        rudof
            .read_data(
                &mut data.as_bytes(),
                "test",
                &RDFFormat::Turtle,
                None,
                &ReaderMode::Strict,
                false,
            )
            .unwrap();

        rudof
            .read_shex(shex.as_bytes(), &ShExFormat::ShExC, None, &ReaderMode::Strict, None)
            .unwrap();
        rudof
            .read_shapemap(shapemap.as_bytes(), "Test", &ShapeMapFormat::default())
            .unwrap();
        let result = rudof.validate_shex().unwrap();
        let node = Node::iri(iri!("http://example/x"));
        let shape = ShapeLabel::iri(iri!("http://example/S"));
        assert!(result.get_info(&node, &shape).unwrap().is_conformant())
    }

    #[test]
    fn test_shex_validation_ko() {
        let data = r#"<http://example/x> <http://example/other> 23 ."#;
        let shex = r#"<http://example/S> { <http://example/p> . }"#;
        let shapemap = r#"<http://example/x>@<http://example/S>"#;
        let mut rudof = Rudof::new(&RudofConfig::default_config().unwrap()).unwrap();
        rudof
            .read_data(
                &mut data.as_bytes(),
                "test",
                &RDFFormat::Turtle,
                None,
                &ReaderMode::Strict,
                false,
            )
            .unwrap();

        rudof
            .read_shex(shex.as_bytes(), &ShExFormat::ShExC, None, &ReaderMode::Strict, None)
            .unwrap();
        rudof
            .read_shapemap(shapemap.as_bytes(), "Test", &ShapeMapFormat::default())
            .unwrap();
        let result = rudof.validate_shex().unwrap();
        let node = Node::iri(iri!("http://example/x"));
        let shape = ShapeLabel::iri(iri!("http://example/S"));
        assert!(result.get_info(&node, &shape).unwrap().is_non_conformant(),)
    }

    #[test]
    fn test_shacl_validation_ok() {
        let data = r#"prefix : <http://example.org/>
        :x :p 23 .
        "#;
        let shacl = r#"prefix :       <http://example.org/>
            prefix sh:     <http://www.w3.org/ns/shacl#>
            prefix xsd:    <http://www.w3.org/2001/XMLSchema#>

            :S a sh:NodeShape; sh:closed true ;
              sh:targetNode :x ;
            sh:property [
                sh:path     :p ;
                sh:minCount 1;
                sh:maxCount 1;
                sh:datatype xsd:integer ;
            ] .
             "#;
        let mut rudof = Rudof::new(&RudofConfig::default_config().unwrap()).unwrap();
        rudof
            .read_data(
                &mut data.as_bytes(),
                "test",
                &RDFFormat::Turtle,
                None,
                &ReaderMode::Strict,
                false,
            )
            .unwrap();

        rudof
            .read_shacl(
                &mut shacl.as_bytes(),
                "test",
                &ShaclFormat::Turtle,
                None,
                &ReaderMode::Lax,
            )
            .unwrap();
        let result = rudof
            .validate_shacl(&ShaclValidationMode::Native, &crate::ShapesGraphSource::CurrentSchema)
            .unwrap();
        assert!(result.results().is_empty())
    }

    #[test]
    fn test_shacl_validation_ko() {
        let data = r#"prefix : <http://example.org/>
        :x :other 23 .
        "#;
        let shacl = r#"prefix :       <http://example.org/>
            prefix sh:     <http://www.w3.org/ns/shacl#>
            prefix xsd:    <http://www.w3.org/2001/XMLSchema#>

            :S a sh:NodeShape;
             sh:targetNode :x ;
            sh:property [
                sh:path     :p ;
                sh:minCount 1;
                sh:maxCount 1;
                sh:datatype xsd:integer ;
            ] .
             "#;
        let mut rudof = Rudof::new(&RudofConfig::new().unwrap()).unwrap();
        rudof
            .read_data(
                &mut data.as_bytes(),
                "test",
                &RDFFormat::Turtle,
                None,
                &ReaderMode::Strict,
                false,
            )
            .unwrap();

        rudof
            .read_shacl(
                &mut shacl.as_bytes(),
                "test",
                &ShaclFormat::Turtle,
                None,
                &ReaderMode::Lax,
            )
            .unwrap();
        let result = rudof
            .validate_shacl(&ShaclValidationMode::Native, &crate::ShapesGraphSource::CurrentSchema)
            .unwrap();
        assert!(!result.conforms())
    }

    #[test]
    fn test_shacl_validation_data_ko() {
        let data = r#"prefix :       <http://example.org/>
            prefix sh:     <http://www.w3.org/ns/shacl#>
            prefix xsd:    <http://www.w3.org/2001/XMLSchema#>

            :x :other 23 .

            :S a sh:NodeShape;
               sh:targetNode :x ;
               sh:property [
                sh:path     :p ;
                sh:minCount 1;
                sh:maxCount 1;
                sh:datatype xsd:integer ;
            ] .
             "#;
        let mut rudof = Rudof::new(&RudofConfig::new().unwrap()).unwrap();
        rudof
            .read_data(
                &mut data.as_bytes(),
                "test",
                &RDFFormat::Turtle,
                None,
                &ReaderMode::Strict,
                false,
            )
            .unwrap();
        let result = rudof
            .validate_shacl(&ShaclValidationMode::Native, &crate::ShapesGraphSource::CurrentData)
            .unwrap();
        assert!(!result.conforms())
    }

    #[test]
    fn test_shacl_validation_data_ok() {
        let data = r#"prefix :       <http://example.org/>
            prefix sh:     <http://www.w3.org/ns/shacl#>
            prefix xsd:    <http://www.w3.org/2001/XMLSchema#>

            :x :p 23 .

            :S a sh:NodeShape;
               sh:targetNode :x ;
               sh:property [
                sh:path     :p ;
                sh:minCount 1;
                sh:maxCount 1;
                sh:datatype xsd:integer ;
            ] .
             "#;
        let mut rudof = Rudof::new(&RudofConfig::new().unwrap()).unwrap();
        rudof
            .read_data(
                &mut data.as_bytes(),
                "test",
                &RDFFormat::Turtle,
                None,
                &ReaderMode::Strict,
                false,
            )
            .unwrap();
        let result = rudof
            .validate_shacl(&ShaclValidationMode::Native, &crate::ShapesGraphSource::CurrentData)
            .unwrap();
        assert!(result.conforms())
    }
}
