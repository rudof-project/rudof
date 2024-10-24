use shacl_ast::compiled::schema::CompiledSchema;
use shacl_ast::ShaclParser;
use srdf::RDFFormat;
use srdf::ReaderMode;
use srdf::SRDFBasic;
use srdf::SRDFGraph;
use std::io::BufRead;

use crate::validate_error::ValidateError;

pub mod graph;
pub mod sparql;

pub trait Store<S> {
    fn store(&self) -> &S;
}

pub struct ShaclDataManager;

impl ShaclDataManager {
    pub fn load<S: SRDFBasic, R: BufRead>(
        reader: R,
        rdf_format: RDFFormat,
        base: Option<&str>,
    ) -> Result<CompiledSchema<S>, ValidateError> {
        let rdf = SRDFGraph::from_reader(reader, &rdf_format, base, &ReaderMode::default())?;

        match ShaclParser::new(rdf).parse() {
            Ok(schema) => Ok(schema.try_into()?),
            Err(error) => Err(ValidateError::ShaclParser(error)),
        }
    }
}
