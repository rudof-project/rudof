use crate::{RudofConfig, RudofError};
use iri_s::IriS;
use prefixmap::PrefixMap;
use shacl_ast::ast::Schema as ShaclSchema;
use shacl_ast::compiled::schema::CompiledSchema as ShaclCompiledSchema;
use shacl_ast::ShaclParser;
use shacl_validation::shacl_processor::{
    EndpointValidation, GraphValidation, ShaclProcessor, ShaclValidationMode,
};
use shacl_validation::store::graph::Graph;
use shacl_validation::validation_report::report::ValidationReport;
use shapemap::{query_shape_map::QueryShapeMap, ResultShapeMap};
use shapemap::{NodeSelector, ShapeMapFormat, ShapeSelector};
use shex_ast::ast::Schema as ShExSchema;
use shex_ast::compiled::compiled_schema::CompiledSchema;
use shex_compact::ShExParser;
use shex_validation::{ResolveMethod, SchemaWithoutImports};
use sparql_service::RdfData;
use srdf::{SRDFBasic, SRDFGraph};
use std::str::FromStr;
use std::{io, result};

// This structs are re-exported as they may be needed in main
pub use shacl_ast::ShaclFormat;
pub use shex_compact::{ShExFormatter, ShapeMapParser, ShapemapFormatter};
pub use shex_validation::Validator as ShExValidator;
pub use shex_validation::{ShExFormat, ValidatorConfig};
pub use srdf::{RDFFormat, ReaderMode, SRDFSparql};

pub type Result<T> = result::Result<T, RudofError>;

/// This represents the public API to interact with `rudof`
pub struct Rudof {
    config: RudofConfig,
    rdf_data: RdfData,
    shex_schema: Option<ShExSchema>,
    shacl_schema: Option<ShaclSchema>,
    resolved_shex_schema: Option<SchemaWithoutImports>,
    shex_validator: Option<ShExValidator>,
    shapemap: Option<QueryShapeMap>,
}

impl Rudof {
    pub fn new(config: &RudofConfig) -> Rudof {
        Rudof {
            config: config.clone(),
            shex_schema: None,
            shacl_schema: None,
            resolved_shex_schema: None,
            shex_validator: None,
            rdf_data: RdfData::new(),
            shapemap: None,
        }
    }

    /// Reads a `ShaclSchema`` and replaces the current one
    /// It also updates the current Shacl processor with the new ShaclSchema
    /// - `base` is used to resolve relative IRIs
    /// - `format` indicates the Shacl format
    pub fn read_shacl<R: io::Read>(
        &mut self,
        reader: R,
        base: Option<&str>,
        format: &ShaclFormat,
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
        let rdf_graph = parse_data(reader, &format, base, reader_mode)?;
        let schema =
            ShaclParser::new(rdf_graph)
                .parse()
                .map_err(|e| RudofError::SHACLParseError {
                    error: format!("{e}"),
                })?;
        self.shacl_schema = Some(schema);
        Ok(())
    }

    /// Reads a ShExSchema and replaces the current one
    /// It also updates the current ShEx validator with the new ShExSchema
    /// - `base` is used to resolve relative IRIs
    /// - `format` indicates the ShEx format according to [`ShExFormat`](https://docs.rs/shex_validation/latest/shex_validation/shex_format/enum.ShExFormat.html)
    pub fn read_shex<R: io::Read>(
        &mut self,
        reader: R,
        base: Option<&str>,
        format: &ShExFormat,
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
        let mut schema = CompiledSchema::new();
        schema
            .from_schema_json(&schema_json)
            .map_err(|e| RudofError::CompilingSchemaError {
                error: format!("{e}"),
            })?;
        self.shex_validator = Some(ShExValidator::new(schema, &self.config.validator_config()));
        Ok(())
    }

    /// Get the current SHACL Schema
    pub fn shacl_schema(&self) -> Option<&ShaclSchema> {
        self.shacl_schema.as_ref()
    }

    pub fn validate_shacl(&mut self) -> Result<ValidationReport> {
        if let Some(schema) = &self.shacl_schema {
            let schema_ast = Box::new(schema.clone());
            if let Some(data) = self.rdf_data.graph() {
                let validator = GraphValidation::from_graph(
                    Graph::from_graph(data.to_owned()),
                    ShaclValidationMode::Native,
                );
                let compiled_schema: ShaclCompiledSchema<SRDFGraph> = schema
                    .to_owned()
                    .try_into()
                    .map_err(|e| RudofError::SHACLCompilationError {
                        error: format!("{e}"),
                        schema: schema_ast.clone(),
                    })?;
                let result =
                    ShaclProcessor::validate(&validator, &compiled_schema).map_err(|e| {
                        RudofError::SHACLValidationError {
                            error: format!("{e}"),
                            schema: schema_ast,
                        }
                    })?;
                Ok(result)
            } else if let Some(endpoint) = self.rdf_data.first_endpoint() {
                let validator = EndpointValidation::from_sparql(
                    endpoint.to_owned(),
                    ShaclValidationMode::Sparql,
                )
                .map_err(|e| RudofError::SHACLEndpointValidationCreation {
                    endpoint: endpoint.clone(),
                    error: format!("{e}"),
                })?;
                let compiled_schema: ShaclCompiledSchema<SRDFSparql> = schema
                    .to_owned()
                    .try_into()
                    .map_err(|e| RudofError::SHACLCompilationError {
                        error: format!("{e}"),
                        schema: schema_ast.clone(),
                    })?;
                let result =
                    ShaclProcessor::validate(&validator, &compiled_schema).map_err(|e| {
                        RudofError::SHACLValidationError {
                            error: format!("{e}"),
                            schema: schema_ast,
                        }
                    })?;
                Ok(result)
            } else {
                Err(RudofError::NoGraphNoFirstEndpoint)
            }
        } else {
            todo!()
        }
    }

    pub fn validate_shex(&mut self) -> Result<ResultShapeMap> {
        let schema_str = format!("{:?}", self.shex_validator);
        match self.shex_validator {
            None => Err(RudofError::ShExValidatorUndefined {}),
            Some(ref mut validator) => match &self.shapemap {
                None => Err(RudofError::NoShapeMap { schema: schema_str }),
                Some(shapemap) => {
                    validator
                        .validate_shapemap(shapemap, &self.rdf_data)
                        .map_err(|e| RudofError::ShExValidatorError {
                            schema: schema_str.clone(),
                            rdf_data: format!("{:?}", self.rdf_data),
                            query_map: format!("{shapemap:?}"),
                            error: format!("{e}"),
                        })?;
                    let result = &validator
                        .result_map(Some(self.rdf_data.prefixmap_in_memory()))
                        .map_err(|e| RudofError::ShExValidatorObtainingResultMapError {
                            schema: schema_str,
                            rdf_data: format!("{:?}", self.rdf_data),
                            shapemap: format!("{shapemap:?}"),
                            error: format!("{e}"),
                        })?;
                    Ok(result.clone())
                }
            },
        }
    }

    /// Add an endpoint to the current RDF data
    pub fn add_endpoint(&mut self, iri: &IriS) -> Result<()> {
        let sparql_endpoint =
            SRDFSparql::new(iri).map_err(|e| RudofError::AddingEndpointError {
                iri: iri.clone(),
                error: format!("{e}"),
            })?;
        self.rdf_data.add_endpoint(sparql_endpoint);
        Ok(())
    }

    /// Parses an RDF graph from a reader and merges it with the current graph
    pub fn merge_data_from_reader<R: io::Read>(
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

    /// Update current shapemap from reader
    pub fn shapemap_from_reader<R: io::Read>(
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

    /// Get current shapemap
    pub fn get_shapemap(&self) -> Option<QueryShapeMap> {
        self.shapemap.clone()
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

    /// Get current ShEx schema
    pub fn shex_schema(&self) -> Option<&ShExSchema> {
        self.shex_schema.as_ref()
    }

    /// Get current RDF Data
    pub fn rdf_data(&self) -> &RdfData {
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

fn parse_data<R: io::Read>(
    reader: R,
    data_format: &RDFFormat,
    base: Option<&str>,
    reader_mode: &ReaderMode,
) -> Result<SRDFGraph> {
    let data = SRDFGraph::from_reader(reader, data_format, base, reader_mode).map_err(|e| {
        RudofError::ParsingRDFDataReader {
            error: format!("{e}"),
        }
    })?;
    Ok(data)
}

#[cfg(test)]
mod tests {
    use iri_s::iri;
    use shapemap::ShapeMapFormat;
    use shex_ast::{compiled::shape_label::ShapeLabel, Node};
    use shex_validation::ShExFormat;

    use crate::RudofConfig;

    use super::Rudof;

    #[test]
    fn test_shex_validation_ok() {
        let data = r#"<http://example/x> <http://example/p> 23 ."#;
        let shex = r#"<http://example/S> { <http://example/p> . }"#;
        let shapemap = r#"<http://example/x>@<http://example/S>"#;
        let mut rudof = Rudof::new(&RudofConfig::default());
        rudof
            .merge_data_from_reader(
                data.as_bytes(),
                &srdf::RDFFormat::Turtle,
                None,
                &srdf::ReaderMode::Strict,
            )
            .unwrap();

        rudof
            .read_shex(shex.as_bytes(), None, &ShExFormat::ShExC)
            .unwrap();
        rudof
            .shapemap_from_reader(shapemap.as_bytes(), &ShapeMapFormat::default())
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
            .merge_data_from_reader(
                data.as_bytes(),
                &srdf::RDFFormat::Turtle,
                None,
                &srdf::ReaderMode::Strict,
            )
            .unwrap();

        rudof
            .read_shex(shex.as_bytes(), None, &ShExFormat::ShExC)
            .unwrap();
        rudof
            .shapemap_from_reader(shapemap.as_bytes(), &ShapeMapFormat::default())
            .unwrap();
        let result = rudof.validate_shex().unwrap();
        let node = Node::iri(iri!("http://example/x"));
        let shape = ShapeLabel::iri(iri!("http://example/S"));
        assert!(result.get_info(&node, &shape).unwrap().is_non_conformant(),)
    }
}
