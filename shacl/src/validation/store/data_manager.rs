use std::io::BufRead;
use rudof_rdf::rdf_core::RDFFormat;
use rudof_rdf::rdf_impl::{InMemoryGraph, ReaderMode};
use crate::ir::IRSchema;
use crate::rdf::ShaclParser;
use crate::validation::error::ValidationError;

pub(crate) struct ShaclDataManager;

impl ShaclDataManager {

    pub fn load<R: BufRead>(
        reader: &mut R,
        source_name: &str,
        rdf_format: &RDFFormat,
        base: Option<&str>,
    ) -> Result<IRSchema, ValidationError> {
        let graph = InMemoryGraph::from_reader(reader, source_name, rdf_format, base, &ReaderMode::default())?;

        match ShaclParser::new(graph).parse() {
            Ok(ast) => Ok(IRSchema::compile(&ast)?),
            Err(err) => Err(err.into()),
        }
    }
}