use crate::validate_error::ValidateError;
use rudof_rdf::rdf_core::RDFFormat;
use rudof_rdf::rdf_impl::{InMemoryGraph, ReaderMode};
use shacl::ir::IRSchema;
use shacl::rdf::ShaclParser;
use std::io::BufRead;

pub mod graph;
pub mod sparql;

pub struct ShaclDataManager;

impl ShaclDataManager {
    pub fn load<R: BufRead>(
        reader: &mut R,
        source_name: &str,
        rdf_format: RDFFormat,
        base: Option<&str>,
    ) -> Result<IRSchema, Box<ValidateError>> {
        let rdf = InMemoryGraph::from_reader(reader, source_name, &rdf_format, base, &ReaderMode::default())
            .map_err(|e| Box::new(ValidateError::Graph(e)))?;
        match ShaclParser::new(rdf).parse() {
            Ok(schema) => {
                let schema_compiled = schema
                    .try_into()
                    .map_err(|e| ValidateError::CompiledShacl { error: Box::new(e) })?;
                Ok(schema_compiled)
            },
            Err(error) => Err(Box::new(ValidateError::ShaclParser(error.into()))),
        }
    }
}
