use iri_s::IriS;
use prefixmap::PrefixMap;
use shapemap::{query_shape_map::QueryShapeMap, ResultShapeMap};
use shex_ast::ast::Schema as SchemaJson;
use shex_ast::compiled::compiled_schema::CompiledSchema;
use shex_compact::{ShExParser, ShapeMapParser};
use shex_validation::{ShExFormat, Validator as ShExValidator};
use sparql_service::RdfData;
use srdf::{RDFFormat, ReaderMode};
use std::str::FromStr;
use std::{io, result};

use crate::{RudofConfig, RudofError};

pub type Result<T> = result::Result<T, RudofError>;

/// This represents the public API to interact with `rudof`
pub struct Rudof {
    config: RudofConfig,
    rdf_data: RdfData,
    shex_validator: Option<ShExValidator>,
    shapemap: Option<QueryShapeMap>,
}

impl Rudof {
    pub fn new(config: &RudofConfig) -> Rudof {
        Rudof {
            config: config.clone(),
            shex_validator: None,
            rdf_data: RdfData::new(),
            shapemap: None,
        }
    }

    /// Initialize a ShEx validator
    ///
    /// - `base` is used to resolve relative IRIs
    /// - `format` indicates the ShEx format according to [`ShExFormat`](https://docs.rs/shex_validation/latest/shex_validation/shex_format/enum.ShExFormat.html)
    pub fn read_shex_validator<R: io::Read>(
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
                    SchemaJson::from_reader(reader).map_err(|e| RudofError::ShExJParserError {
                        error: format!("{e}"),
                    })?;
                Ok(schema_json)
            }
        }?;
        let mut schema = CompiledSchema::new();
        schema
            .from_schema_json(&schema_json)
            .map_err(|e| RudofError::CompilingSchemaError {
                error: format!("{e}"),
            })?;
        self.shex_validator = Some(ShExValidator::new(schema, &self.config.validator_config()));
        Ok(())
    }

    pub fn validate_shex(&mut self) -> Result<ResultShapeMap> {
        let schema_str = format!("{:?}", self.shex_validator);
        match self.shex_validator {
            None => Err(RudofError::ShExValidatorUndefined {}),
            Some(ref mut validator) => match &self.shapemap {
                None => todo!(),
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
                        .result_map(Some(self.rdf_data.prefixmap()))
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

    pub fn shapemap_from_reader<R: io::Read>(&mut self, mut reader: R) -> Result<()> {
        let mut v = Vec::new();
        reader
            .read_to_end(&mut v)
            .map_err(|e| RudofError::ReadError {
                error: format!("{e}"),
            })?;
        let s = String::from_utf8(v).map_err(|e| RudofError::Utf8Error {
            error: format!("{e}"),
        })?;
        let shapemap = ShapeMapParser::parse(
            s.as_str(),
            &Some(self.nodes_prefixmap()),
            &self.shex_shapes_prefixmap(),
        )
        .map_err(|e| RudofError::ShapeMapParseError {
            str: s.to_string(),
            error: format!("{e}"),
        })?;
        self.shapemap = Some(shapemap);
        Ok(())
    }

    /// Returns the RDF data prefixmap
    pub fn nodes_prefixmap(&self) -> PrefixMap {
        self.rdf_data.prefixmap()
    }

    /// Returns the shapes prefixmap
    ///
    /// If no ShEx schema has been set, returns None
    pub fn shex_shapes_prefixmap(&self) -> Option<PrefixMap> {
        self.shex_validator
            .as_ref()
            .map(|validator| validator.shapes_prefixmap())
    }
}

#[cfg(test)]
mod tests {
    use iri_s::iri;
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
            .read_shex_validator(shex.as_bytes(), None, &ShExFormat::ShExC)
            .unwrap();
        rudof.shapemap_from_reader(shapemap.as_bytes()).unwrap();
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
            .read_shex_validator(shex.as_bytes(), None, &ShExFormat::ShExC)
            .unwrap();
        rudof.shapemap_from_reader(shapemap.as_bytes()).unwrap();
        let result = rudof.validate_shex().unwrap();
        let node = Node::iri(iri!("http://example/x"));
        let shape = ShapeLabel::iri(iri!("http://example/S"));
        assert!(result.get_info(&node, &shape).unwrap().is_non_conformant(),)
    }
}
