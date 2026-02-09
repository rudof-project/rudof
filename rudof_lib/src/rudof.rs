// These are the structs that are publicly re-exported
pub use dctap::{DCTAPFormat, DCTap as DCTAP};
pub use iri_s::iri;
pub use mie::Mie;
pub use prefixmap::PrefixMap;
pub use shacl_ast::ShaclFormat;
pub use shacl_ast::ast::Schema as ShaclSchema;
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
pub use srdf::QueryResultFormat;
pub use srdf::UmlGenerationMode;
pub use srdf::{QuerySolution, QuerySolutions, RDFFormat, ReaderMode, SRDFSparql, VarName};

pub type Result<T> = result::Result<T, RudofError>;

use crate::{InputSpec, RudofConfig, RudofError, ShapesGraphSource, UrlSpec};
use iri_s::IriS;
use rdf_config::RdfConfigModel;
use shacl_rdf::{ShaclParser, ShaclWriter};
use shacl_validation::shacl_processor::{GraphValidation, ShaclProcessor};
use shacl_validation::store::graph::Graph;
use shapes_comparator::CoShaMoConverter;
use shapes_converter::{ShEx2Uml, Tap2ShEx};
use shex_ast::compact::ShExParser;
use shex_ast::ir::schema_ir::SchemaIR;
use shex_ast::shapemap::{NodeSelector, ShapeSelector};
use shex_ast::{ResolveMethod, ShExFormat, ShapeLabelIdx};
// use shex_validation::SchemaWithoutImports;
use srdf::QueryRDF;
use srdf::Rdf;
use srdf::rdf_visualizer::visual_rdf_graph::VisualRDFGraph;
use srdf::{FocusRDF, SRDFGraph, SparqlQuery};
use std::fmt::Debug;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::str::FromStr;
use std::{env, io, result};
use tracing::trace;
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
    pub fn endpoints_to_use(&self) -> Vec<(String, SRDFSparql)> {
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

        mode1: CompareSchemaMode,
        mode2: CompareSchemaMode,

        format1: CompareSchemaFormat,
        format2: CompareSchemaFormat,

        base1: Option<&str>,
        base2: Option<&str>,

        reader_mode: &ReaderMode,

        label1: Option<&str>,
        label2: Option<&str>,

        source_name1: Option<&str>,
        source_name2: Option<&str>,
    ) -> Result<ShaCo> {
        let coshamo1 = self.get_coshamo(reader1, &mode1, &format1, base1, reader_mode, label1, source_name1)?;
        let coshamo2 = self.get_coshamo(reader2, &mode2, &format2, base2, reader_mode, label2, source_name2)?;
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
                ShapeMapFormat::JSON => {
                    serde_json::to_writer_pretty(writer, &shapemap).map_err(|e| RudofError::ErrorWritingShExJson {
                        schema: format!("{:?}", shapemap.clone()),
                        error: format!("{e}"),
                    })?;
                    Ok(())
                },
                ShapeMapFormat::CSV => {
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
            ShaclFormat::RDFXML => Ok(RDFFormat::RDFXML),
            ShaclFormat::TriG => Ok(RDFFormat::TriG),
            ShaclFormat::N3 => Ok(RDFFormat::N3),
            ShaclFormat::NQuads => Ok(RDFFormat::NQuads),
            ShaclFormat::JsonLd => Ok(RDFFormat::JsonLd),
        }?;

        let rdf_graph = SRDFGraph::from_reader(reader, reader_name, &format, base, reader_mode).map_err(|e| {
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
    pub fn get_endpoint(&self, name: &str) -> Result<(String, SRDFSparql)> {
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
                let endpoint = SRDFSparql::new(&iri, &PrefixMap::new()).map_err(|e| RudofError::InvalidEndpoint {
                    endpoint: name.to_string(),
                    error: format!("{e}"),
                })?;
                Ok((name.to_string(), endpoint))
            },
        }
    }

    /// Reads a `DCTAP` and replaces the current one
    /// - `format` indicates the DCTAP format
    pub fn read_dctap<R: std::io::Read>(&mut self, reader: R, format: &DCTAPFormat) -> Result<()> {
        let dctap = match format {
            DCTAPFormat::CSV => {
                let dctap = DCTAP::from_reader(reader, &self.config.tap_config())
                    .map_err(|e| RudofError::DCTAPReaderCSVReader { error: format!("{e}") })?;
                Ok(dctap)
            },
            DCTAPFormat::XLS | DCTAPFormat::XLSB | DCTAPFormat::XLSM | DCTAPFormat::XLSX => {
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
            DCTAPFormat::CSV => {
                let dctap = DCTAP::from_path(path, &self.config.tap_config()).map_err(|e| RudofError::DCTAPReader {
                    path: path_name,
                    format: format.to_string(),
                    error: format!("{e}"),
                })?;
                Ok::<DCTAP, RudofError>(dctap)
            },
            DCTAPFormat::XLS | DCTAPFormat::XLSB | DCTAPFormat::XLSM | DCTAPFormat::XLSX => {
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
            .from_schema_json(&schema_ast, &ResolveMethod::default(), &base_iri)
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
                Ok::<(shacl_ir::schema_ir::SchemaIR, shacl_ast::Schema<RdfData>), RudofError>((
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
        let sparql_endpoint = SRDFSparql::new(iri, prefixmap).map_err(|e| RudofError::AddingEndpointError {
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
            ShapeMapFormat::JSON => todo!(),
            ShapeMapFormat::CSV => todo!(),
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
        mode: &CompareSchemaMode,
        format: &CompareSchemaFormat,
        base: Option<&str>,
        reader_mode: &ReaderMode,
        label: Option<&str>,
        source_name: Option<&str>,
    ) -> Result<CoShaMo> {
        let comparator_config = self.config().comparator_config();
        match mode {
            CompareSchemaMode::Shacl => Err(RudofError::NotImplemented {
                msg: "Not yet implemented comparison between SHACL schemas".to_string(),
            }),
            CompareSchemaMode::ShEx => {
                let shex_format = format
                    .to_shex_format()
                    .map_err(|e| RudofError::InvalidCompareSchemaFormat {
                        format: format!("{format:?}"),
                        error: format!("{e}"),
                    })?;
                let shex = self.read_shex_only(reader, &shex_format, base, reader_mode, source_name)?;
                let mut converter = CoShaMoConverter::new(&comparator_config);
                let coshamo = converter
                    .from_shex(&shex, label)
                    .map_err(|e| RudofError::CoShaMoFromShExError {
                        schema: format!("{shex:?}"),
                        error: format!("{e}"),
                    })?;
                Ok(coshamo)
            },
            CompareSchemaMode::ServiceDescription => Err(RudofError::NotImplemented {
                msg: "Not yet implemented comparison between Service descriptions".to_string(),
            }),
        }
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
        ShaclFormat::RDFXML => Ok(RDFFormat::RDFXML),
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

#[cfg(test)]
mod tests {
    use iri_s::iri;
    use shacl_ast::ShaclFormat;
    use shacl_validation::shacl_processor::ShaclValidationMode;
    use shex_ast::ShExFormat;
    use shex_ast::shapemap::ShapeMapFormat;
    use shex_ast::{Node, ir::shape_label::ShapeLabel};

    use crate::RudofConfig;

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
                &srdf::RDFFormat::Turtle,
                None,
                &srdf::ReaderMode::Strict,
                false,
            )
            .unwrap();

        rudof
            .read_shex(
                shex.as_bytes(),
                &ShExFormat::ShExC,
                None,
                &srdf::ReaderMode::Strict,
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
                &srdf::RDFFormat::Turtle,
                None,
                &srdf::ReaderMode::Strict,
                false,
            )
            .unwrap();

        rudof
            .read_shex(
                shex.as_bytes(),
                &ShExFormat::ShExC,
                None,
                &srdf::ReaderMode::Strict,
                None,
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
    fn test_shex_validation_ko() {
        let data = r#"<http://example/x> <http://example/other> 23 ."#;
        let shex = r#"<http://example/S> { <http://example/p> . }"#;
        let shapemap = r#"<http://example/x>@<http://example/S>"#;
        let mut rudof = Rudof::new(&RudofConfig::default_config().unwrap()).unwrap();
        rudof
            .read_data(
                &mut data.as_bytes(),
                "test",
                &srdf::RDFFormat::Turtle,
                None,
                &srdf::ReaderMode::Strict,
                false,
            )
            .unwrap();

        rudof
            .read_shex(
                shex.as_bytes(),
                &ShExFormat::ShExC,
                None,
                &srdf::ReaderMode::Strict,
                None,
            )
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
                &srdf::RDFFormat::Turtle,
                None,
                &srdf::ReaderMode::Strict,
                false,
            )
            .unwrap();

        rudof
            .read_shacl(
                &mut shacl.as_bytes(),
                "test",
                &ShaclFormat::Turtle,
                None,
                &srdf::ReaderMode::Lax,
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
                &srdf::RDFFormat::Turtle,
                None,
                &srdf::ReaderMode::Strict,
                false,
            )
            .unwrap();

        rudof
            .read_shacl(
                &mut shacl.as_bytes(),
                "test",
                &ShaclFormat::Turtle,
                None,
                &srdf::ReaderMode::Lax,
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
                &srdf::RDFFormat::Turtle,
                None,
                &srdf::ReaderMode::Strict,
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
                &srdf::RDFFormat::Turtle,
                None,
                &srdf::ReaderMode::Strict,
                false,
            )
            .unwrap();
        let result = rudof
            .validate_shacl(&ShaclValidationMode::Native, &crate::ShapesGraphSource::CurrentData)
            .unwrap();
        assert!(result.conforms())
    }
}
