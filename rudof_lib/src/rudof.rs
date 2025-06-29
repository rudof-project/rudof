use crate::{RudofConfig, RudofError, ShapesGraphSource};
use iri_s::IriS;
use shacl_rdf::{ShaclParser, ShaclWriter};
use shacl_validation::shacl_processor::{GraphValidation, ShaclProcessor};
use shacl_validation::store::graph::Graph;

use shapemap::{NodeSelector, ShapeSelector};
use shapes_converter::{ShEx2Uml, Tap2ShEx};
use shex_ast::ir::schema_ir::SchemaIR;
use shex_compact::ShExParser;
use shex_validation::{ResolveMethod, SchemaWithoutImports};
use srdf::{FocusRDF, SRDFGraph};
use std::fmt::Debug;
use std::path::Path;
use std::str::FromStr;
use std::{io, result};

// These are the structs that are publicly re-exported
pub use dctap::{DCTAPFormat, DCTap as DCTAP};
pub use iri_s::iri;
pub use prefixmap::PrefixMap;
pub use shacl_ast::ShaclFormat;
pub use shacl_validation::shacl_processor::ShaclValidationMode;
pub use shacl_validation::validation_report::report::ValidationReport;
pub use shapemap::{QueryShapeMap, ResultShapeMap, ShapeMapFormat, ValidationStatus};
pub use shex_compact::{ShExFormatter, ShapeMapParser, ShapemapFormatter as ShapeMapFormatter};
pub use shex_validation::Validator as ShExValidator;
pub use shex_validation::{ShExFormat, ValidatorConfig};
use srdf::QueryRDF;
pub use srdf::{QuerySolution, QuerySolutions, RDFFormat, ReaderMode, SRDFSparql, VarName};

pub type Result<T> = result::Result<T, RudofError>;
pub use shacl_ast::ast::Schema as ShaclSchema;
pub use shacl_ir::compiled::schema::SchemaIR as ShaclSchemaIR;
pub use shapes_converter::UmlGenerationMode;
pub use shex_ast::Schema as ShExSchema;
pub use sparql_service::RdfData;

/// This represents the public API to interact with `rudof`
#[derive(Debug)]
pub struct Rudof {
    config: RudofConfig,
    rdf_data: RdfData,
    shacl_schema: Option<ShaclSchema<RdfData>>,
    shacl_schema_ir: Option<ShaclSchemaIR>,
    shex_schema: Option<ShExSchema>,
    shex_schema_ir: Option<SchemaIR>,
    resolved_shex_schema: Option<SchemaWithoutImports>,
    shex_validator: Option<ShExValidator>,
    shapemap: Option<QueryShapeMap>,
    dctap: Option<DCTAP>,
    shex_results: Option<ResultShapeMap>,
}

// TODO: We added this declaration so PyRudof can contain Rudof and be Send as required by PyO3
// TODO: Review what are the consequences of this declaration
unsafe impl Send for Rudof {}

impl Rudof {
    pub fn new(config: &RudofConfig) -> Rudof {
        Rudof {
            config: config.clone(),
            shex_schema: None,
            shex_schema_ir: None,
            shacl_schema: None,
            shacl_schema_ir: None,
            resolved_shex_schema: None,
            shex_validator: None,
            rdf_data: RdfData::new(),
            shapemap: None,
            dctap: None,
            shex_results: None,
        }
    }

    pub fn config(&self) -> &RudofConfig {
        &self.config
    }

    pub fn update_config(&mut self, config: &RudofConfig) {
        self.config = config.clone();
    }

    /// Resets the current RDF Data
    pub fn reset_data(&mut self) {
        self.rdf_data = RdfData::new()
    }

    /// Resets the current DCTAP
    pub fn reset_dctap(&mut self) {
        self.dctap = None
    }

    /// Resets the current SHACL shapes graph
    pub fn reset_shacl(&mut self) {
        self.shacl_schema = None
    }

    /// Resets all current values
    pub fn reset_all(&mut self) {
        self.reset_data();
        self.reset_dctap();
        self.reset_shacl();
        self.reset_shapemap();
        self.reset_validation_results();
        self.reset_shex();
    }

    /// Get the shapes graph schema from the current RDF data
    pub fn get_shacl_from_data(&mut self) -> Result<()> {
        let schema = shacl_schema_from_data(self.rdf_data.clone())?;
        self.shacl_schema = Some(schema.clone());
        let shacl_ir = ShaclSchemaIR::compile(&schema)
            .map_err(|e| RudofError::ShaclCompilation { error: Box::new(e) })?;
        self.shacl_schema_ir = Some(shacl_ir);
        Ok(())
    }

    /// Get the current SHACL
    pub fn get_shacl(&self) -> Option<&ShaclSchema<RdfData>> {
        self.shacl_schema.as_ref()
    }

    /// Get the current SHACL Schema Internal Representation
    pub fn get_shacl_ir(&self) -> Option<&ShaclSchemaIR> {
        self.shacl_schema_ir.as_ref()
    }

    /// Get the current ShEx Schema
    pub fn get_shex(&self) -> Option<&ShExSchema> {
        self.shex_schema.as_ref()
    }

    /// Get the current ShEx Schema Internal Representation
    pub fn get_shex_ir(&self) -> Option<&SchemaIR> {
        self.shex_schema_ir.as_ref()
    }

    /// Get the current DCTAP
    pub fn get_dctap(&self) -> Option<&DCTAP> {
        self.dctap.as_ref()
    }

    /// Get the current shapemap
    pub fn get_shapemap(&self) -> Option<&QueryShapeMap> {
        self.shapemap.as_ref()
    }

    /// Converts the current DCTAP to a ShExSchema
    /// Stores the value of the ShExSchema in the current shex
    pub fn dctap2shex(&mut self) -> Result<()> {
        if let Some(dctap) = self.get_dctap() {
            let converter = Tap2ShEx::new(&self.config.tap2shex_config());
            let shex = converter
                .convert(dctap)
                .map_err(|e| RudofError::DCTap2ShEx {
                    error: format!("{e}"),
                })?;
            self.shex_schema = Some(shex);
            Ok(())
        } else {
            Err(RudofError::NoDCTAP)
        }
    }

    /// Generate a UML Class-like representation of a ShEx schema according to PlantUML syntax
    ///
    pub fn shex2plant_uml<W: io::Write>(
        &self,
        mode: &UmlGenerationMode,
        writer: &mut W,
    ) -> Result<()> {
        if let Some(shex) = &self.shex_schema {
            let mut converter = ShEx2Uml::new(&self.config.shex2uml_config());
            converter
                .convert(shex)
                .map_err(|e| RudofError::ShEx2PlantUmlError {
                    error: format!("{e}"),
                })?;
            converter.as_plantuml(writer, mode).map_err(|e| {
                RudofError::ShEx2PlantUmlErrorAsPlantUML {
                    error: format!("{e}"),
                }
            })?;
            Ok(())
        } else {
            Err(RudofError::ShEx2UmlWithoutShEx)
        }
    }

    pub fn serialize_data<W: io::Write>(&self, format: &RDFFormat, writer: &mut W) -> Result<()> {
        self.rdf_data
            .serialize(format, writer)
            .map_err(|e| RudofError::SerializingData {
                error: format!("{e}"),
            })
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
                    formatter.write_shapemap(shapemap, writer).map_err(|e| {
                        RudofError::ErrorFormattingShapeMap {
                            shapemap: format!("{:?}", shapemap.clone()),
                            error: format!("{e}"),
                        }
                    })
                }
                ShapeMapFormat::JSON => {
                    serde_json::to_writer_pretty(writer, &shapemap).map_err(|e| {
                        RudofError::ErrorWritingShExJson {
                            schema: format!("{:?}", shapemap.clone()),
                            error: format!("{e}"),
                        }
                    })?;
                    Ok(())
                }
            }
        } else {
            Err(RudofError::NoShapeMapToSerialize)
        }
    }

    /// Serialize the current ShEx Schema
    pub fn serialize_shex<W: io::Write>(
        &self,
        format: &ShExFormat,
        formatter: &ShExFormatter,
        writer: &mut W,
    ) -> Result<()> {
        if let Some(shex) = &self.shex_schema {
            match format {
                ShExFormat::ShExC => {
                    formatter.write_schema(shex, writer).map_err(|e| {
                        RudofError::ErrorFormattingSchema {
                            schema: format!("{:?}", shex.clone()),
                            error: format!("{e}"),
                        }
                    })?;
                    Ok(())
                }
                ShExFormat::ShExJ => {
                    serde_json::to_writer_pretty(writer, &shex).map_err(|e| {
                        RudofError::ErrorWritingShExJson {
                            schema: format!("{:?}", shex.clone()),
                            error: format!("{e}"),
                        }
                    })?;
                    Ok(())
                }
                ShExFormat::Turtle => Err(RudofError::NotImplemented {
                    msg: format!("ShEx to ShExR for {shex:?}"),
                }),
            }
        } else {
            Err(RudofError::NoShExSchemaToSerialize)
        }
    }

    pub fn run_query_str(&mut self, str: &str) -> Result<QuerySolutions<RdfData>> {
        self.rdf_data
            .check_store()
            .map_err(|e| RudofError::StorageError {
                error: format!("{e}"),
            })?;
        let results = self
            .rdf_data
            .query_select(str)
            .map_err(|e| RudofError::QueryError {
                str: str.to_string(),
                error: format!("{e}"),
            })?;
        Ok(results)
    }

    pub fn run_query<R: io::Read>(&mut self, reader: &mut R) -> Result<QuerySolutions<RdfData>> {
        let mut str = String::new();
        reader
            .read_to_string(&mut str)
            .map_err(|e| RudofError::ReadError {
                error: format!("{e}"),
            })?;
        self.run_query_str(str.as_str())
    }

    pub fn serialize_shacl<W: io::Write>(
        &self,
        format: &ShaclFormat,
        writer: &mut W,
    ) -> Result<()> {
        if let Some(shacl) = &self.shacl_schema {
            match format {
                ShaclFormat::Internal => {
                    write!(writer, "{shacl}").map_err(|e| RudofError::SerializingSHACLInternal {
                        error: format!("{e}"),
                    })
                }
                _ => {
                    let data_format = shacl_format2rdf_format(format)?;
                    let mut shacl_writer: ShaclWriter<RdfData> = ShaclWriter::new();
                    shacl_writer
                        .write(shacl)
                        .map_err(|e| RudofError::WritingSHACL {
                            shacl: format!("{:?}", shacl.clone()),
                            error: format!("{e}"),
                        })?;
                    shacl_writer.serialize(&data_format, writer).map_err(|e| {
                        RudofError::SerializingSHACL {
                            error: format!("{e}"),
                            shacl: format!("{:?}", shacl.clone()),
                        }
                    })?;
                    Ok(())
                }
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
        reader: R,
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
        }?;

        let rdf_graph =
            SRDFGraph::from_reader(reader, &format, base, reader_mode).map_err(|e| {
                RudofError::ReadError {
                    error: format!("{e}"),
                }
            })?;
        let rdf_data = RdfData::from_graph(rdf_graph).map_err(|e| RudofError::ReadError {
            error: format!("Obtaining SHACL from rdf_data: {e}"),
        })?;
        let schema = shacl_schema_from_data(rdf_data)?;
        self.shacl_schema = Some(schema);
        Ok(())
    }

    /// Reads a `DCTAP` and replaces the current one
    /// - `format` indicates the DCTAP format
    pub fn read_dctap<R: std::io::Read>(&mut self, reader: R, format: &DCTAPFormat) -> Result<()> {
        let dctap = match format {
            DCTAPFormat::CSV => {
                let dctap = DCTAP::from_reader(reader, &self.config.tap_config()).map_err(|e| {
                    RudofError::DCTAPReaderCSVReader {
                        error: format!("{e}"),
                    }
                })?;
                Ok(dctap)
            }
            DCTAPFormat::XLS | DCTAPFormat::XLSB | DCTAPFormat::XLSM | DCTAPFormat::XLSX => {
                Err(RudofError::DCTAPReadXLSNoPath)
            }
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
                let dctap = DCTAP::from_path(path, &self.config.tap_config()).map_err(|e| {
                    RudofError::DCTAPReaderCSV {
                        path: path_name,
                        error: format!("{e}"),
                    }
                })?;
                Ok(dctap)
            } /*DCTAPFormat::XLS | DCTAPFormat::XLSB | DCTAPFormat::XLSM | DCTAPFormat::XLSX => {
            let path_buf = path.as_ref().to_path_buf();
            let dctap = DCTAP::from_excel(path_buf, None, &self.config.tap_config())
            .map_err(|e| RudofError::DCTAPReaderPathXLS {
            path: path_name,
            error: format!("{e}"),
            format: format!("{format:?}"),
            })?;
            Ok(dctap)
            }*/
            _ => todo!(),
        }?;
        self.dctap = Some(dctap);
        Ok(())
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
    ) -> Result<()> {
        let schema_json = match format {
            ShExFormat::ShExC => {
                let base = match base {
                    Some(str) => {
                        let iri = IriS::from_str(str).map_err(|e| RudofError::BaseIriError {
                            str: str.to_string(),
                            error: format!("{e}"),
                        })?;
                        Ok(Some(iri))
                    }
                    None => Ok(None),
                }?;
                let schema_json = ShExParser::from_reader(reader, base).map_err(|e| {
                    RudofError::ShExCParserError {
                        error: format!("{e}"),
                    }
                })?;
                Ok(schema_json)
            }
            ShExFormat::ShExJ => {
                let schema_json =
                    ShExSchema::from_reader(reader).map_err(|e| RudofError::ShExJParserError {
                        error: format!("{e}"),
                    })?;
                Ok(schema_json)
            }
            ShExFormat::Turtle => {
                todo!()
                /*let rdf = parse_data(
                    &vec![input.clone()],
                    &DataFormat::Turtle,
                    reader_mode,
                    &config.rdf_config(),
                )?;
                let schema = ShExRParser::new(rdf).parse()?;
                Ok(schema) */
            }
        }?;
        self.shex_schema = Some(schema_json.clone());
        let mut schema = SchemaIR::new();
        schema
            .from_schema_json(&schema_json)
            .map_err(|e| RudofError::CompilingSchemaError {
                error: format!("{e}"),
            })?;
        self.shex_schema_ir = Some(schema.clone());

        let validator =
            ShExValidator::new(schema, &self.config.validator_config()).map_err(|e| {
                RudofError::ShExValidatorCreationError {
                    error: format!("{e}"),
                    schema: format!("{schema_json}"),
                }
            })?;
        self.shex_validator = Some(validator);
        Ok(())
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
        let (compiled_schema, shacl_schema) = match shapes_graph_source {
            ShapesGraphSource::CurrentSchema if self.shacl_schema.is_some() => {
                let ast_schema = self.shacl_schema.as_ref().unwrap();
                let compiled_schema = ast_schema.clone().to_owned().try_into().map_err(|e| {
                    RudofError::SHACLCompilationError {
                        error: format!("{e}"),
                        schema: Box::new(ast_schema.clone()),
                    }
                })?;
                Ok((compiled_schema, ast_schema.clone()))
            }
            _ => {
                let ast_schema = shacl_schema_from_data(self.rdf_data.clone())?;
                let compiled_schema = ast_schema.to_owned().try_into().map_err(|e| {
                    RudofError::SHACLCompilationError {
                        error: format!("{e}"),
                        schema: Box::new(ast_schema.clone()),
                    }
                })?;
                Ok((compiled_schema, ast_schema))
            }
        }?;
        let validator = GraphValidation::from_graph(Graph::from_data(self.rdf_data.clone()), *mode);
        let result = ShaclProcessor::validate(&validator, &compiled_schema).map_err(|e| {
            RudofError::SHACLValidationError {
                error: format!("{e}"),
                schema: Box::new(shacl_schema),
            }
        })?;
        Ok(result)
    }

    /// Validate RDF data using ShEx
    /// It uses a ShEx validator which has a corrent ShEx schema and the current ShapeMap
    pub fn validate_shex(&mut self) -> Result<ResultShapeMap> {
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
                            &None, // TODO!! Get schema prefix map
                        )
                        .map_err(|e| RudofError::ShExValidatorError {
                            schema: schema_str.clone(),
                            rdf_data: format!("{:?}", self.rdf_data),
                            query_map: format!("{shapemap:?}"),
                            error: format!("{e}"),
                        })?;
                    Ok(result.clone())
                }
            },
        }
    }

    /// Adds an endpoint to the current RDF data
    pub fn add_endpoint(&mut self, iri: &IriS, prefixmap: &PrefixMap) -> Result<()> {
        let sparql_endpoint =
            SRDFSparql::new(iri, prefixmap).map_err(|e| RudofError::AddingEndpointError {
                iri: iri.clone(),
                error: format!("{e}"),
            })?;

        self.rdf_data.add_endpoint(sparql_endpoint);
        Ok(())
    }

    /// Parses an RDF graph from a reader and merges it with the current graph
    pub fn read_data<R: io::Read>(
        &mut self,
        reader: R,
        format: &RDFFormat,
        base: Option<&str>,
        reader_mode: &ReaderMode,
    ) -> Result<()> {
        self.rdf_data
            .merge_from_reader(reader, format, base, reader_mode)
            .map_err(|e| RudofError::MergeRDFDataFromReader {
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
            }
            Some(ref mut sm) => {
                sm.add_association(node, shape);
            }
        };
    }

    /// Read a shapemap
    pub fn read_shapemap<R: io::Read>(
        &mut self,
        mut reader: R,
        shapemap_format: &ShapeMapFormat,
    ) -> Result<()> {
        let mut v = Vec::new();
        reader
            .read_to_end(&mut v)
            .map_err(|e| RudofError::ReadError {
                error: format!("{e}"),
            })?;
        let s = String::from_utf8(v).map_err(|e| RudofError::Utf8Error {
            error: format!("{e}"),
        })?;
        let shapemap = match shapemap_format {
            ShapeMapFormat::Compact => {
                let shapemap = ShapeMapParser::parse(
                    s.as_str(),
                    &Some(self.nodes_prefixmap()),
                    &self.shex_shapes_prefixmap(),
                )
                .map_err(|e| RudofError::ShapeMapParseError {
                    str: s.to_string(),
                    error: format!("{e}"),
                })?;
                Ok(shapemap)
            }
            ShapeMapFormat::JSON => todo!(),
        }?;
        self.shapemap = Some(shapemap);
        Ok(())
    }

    pub fn reset_shapemap(&mut self) {
        self.shapemap = None
    }

    /// Returns the RDF data prefixmap
    pub fn nodes_prefixmap(&self) -> PrefixMap {
        self.rdf_data.prefixmap_in_memory()
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

    /// Obtains the current `shex_schema` after resolving import declarations
    ///
    /// If the import declarations in the current schema have not been resolved, it resolves them
    pub fn shex_schema_without_imports(&mut self) -> Result<SchemaWithoutImports> {
        match &self.resolved_shex_schema {
            None => match &self.shex_schema {
                Some(schema) => {
                    let schema_resolved = SchemaWithoutImports::resolve_imports(
                        schema,
                        &Some(schema.source_iri()),
                        Some(&ResolveMethod::default()),
                    )
                    .map_err(|e| RudofError::ResolvingImportsShExSchema {
                        error: format!("{e}"),
                    })?;
                    self.resolved_shex_schema = Some(schema_resolved.clone());
                    Ok(schema_resolved)
                }
                None => Err(RudofError::NoShExSchemaForResolvingImports),
            },
            Some(resolved_schema) => Ok(resolved_schema.clone()),
        }
    }
}

fn shacl_schema_from_data<RDF: FocusRDF + Debug>(rdf_data: RDF) -> Result<ShaclSchema<RDF>> {
    let schema = ShaclParser::new(rdf_data)
        .parse()
        .map_err(|e| RudofError::SHACLParseError {
            error: format!("{e}"),
        })?;
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
    }
}

#[cfg(test)]
mod tests {
    use iri_s::iri;
    use shacl_ast::ShaclFormat;
    use shacl_validation::shacl_processor::ShaclValidationMode;
    use shapemap::ShapeMapFormat;
    use shex_ast::{ir::shape_label::ShapeLabel, Node};
    use shex_validation::ShExFormat;

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
        let mut rudof = Rudof::new(&RudofConfig::default());
        rudof
            .read_data(
                data.as_bytes(),
                &srdf::RDFFormat::Turtle,
                None,
                &srdf::ReaderMode::Strict,
            )
            .unwrap();

        rudof
            .read_shex(shex.as_bytes(), &ShExFormat::ShExC, None)
            .unwrap();
        rudof
            .read_shapemap(shapemap.as_bytes(), &ShapeMapFormat::default())
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
        let mut rudof = Rudof::new(&RudofConfig::default());
        rudof
            .read_data(
                data.as_bytes(),
                &srdf::RDFFormat::Turtle,
                None,
                &srdf::ReaderMode::Strict,
            )
            .unwrap();

        rudof
            .read_shex(shex.as_bytes(), &ShExFormat::ShExC, None)
            .unwrap();
        rudof
            .read_shapemap(shapemap.as_bytes(), &ShapeMapFormat::default())
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
        let mut rudof = Rudof::new(&RudofConfig::default());
        rudof
            .read_data(
                data.as_bytes(),
                &srdf::RDFFormat::Turtle,
                None,
                &srdf::ReaderMode::Strict,
            )
            .unwrap();

        rudof
            .read_shex(shex.as_bytes(), &ShExFormat::ShExC, None)
            .unwrap();
        rudof
            .read_shapemap(shapemap.as_bytes(), &ShapeMapFormat::default())
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
        let mut rudof = Rudof::new(&RudofConfig::default());
        rudof
            .read_data(
                data.as_bytes(),
                &srdf::RDFFormat::Turtle,
                None,
                &srdf::ReaderMode::Strict,
            )
            .unwrap();

        rudof
            .read_shacl(
                shacl.as_bytes(),
                &ShaclFormat::Turtle,
                None,
                &srdf::ReaderMode::Lax,
            )
            .unwrap();
        let result = rudof
            .validate_shacl(
                &ShaclValidationMode::Native,
                &crate::ShapesGraphSource::CurrentSchema,
            )
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
        let mut rudof = Rudof::new(&RudofConfig::default());
        rudof
            .read_data(
                data.as_bytes(),
                &srdf::RDFFormat::Turtle,
                None,
                &srdf::ReaderMode::Strict,
            )
            .unwrap();

        rudof
            .read_shacl(
                shacl.as_bytes(),
                &ShaclFormat::Turtle,
                None,
                &srdf::ReaderMode::Lax,
            )
            .unwrap();
        let result = rudof
            .validate_shacl(
                &ShaclValidationMode::Native,
                &crate::ShapesGraphSource::CurrentSchema,
            )
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
        let mut rudof = Rudof::new(&RudofConfig::default());
        rudof
            .read_data(
                data.as_bytes(),
                &srdf::RDFFormat::Turtle,
                None,
                &srdf::ReaderMode::Strict,
            )
            .unwrap();
        let result = rudof
            .validate_shacl(
                &ShaclValidationMode::Native,
                &crate::ShapesGraphSource::CurrentData,
            )
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
        let mut rudof = Rudof::new(&RudofConfig::default());
        rudof
            .read_data(
                data.as_bytes(),
                &srdf::RDFFormat::Turtle,
                None,
                &srdf::ReaderMode::Strict,
            )
            .unwrap();
        let result = rudof
            .validate_shacl(
                &ShaclValidationMode::Native,
                &crate::ShapesGraphSource::CurrentData,
            )
            .unwrap();
        assert!(result.conforms())
    }
}
