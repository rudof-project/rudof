use shacl_ir::compiled::schema::SchemaIR;
use shacl_rdf::rdf_to_shacl::ShaclParser;
use srdf::RDFFormat;
use srdf::ReaderMode;
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
    pub fn load<R: BufRead>(
        reader: R,
        rdf_format: RDFFormat,
        base: Option<&str>,
    ) -> Result<SchemaIR<SRDFGraph>, ValidateError> {
        let rdf = SRDFGraph::from_reader(reader, &rdf_format, base, &ReaderMode::default())?;
        match ShaclParser::new(rdf).parse() {
            Ok(schema) => {
                let schema_compiled = schema.try_into()?;
                Ok(schema_compiled)
            }
            Err(error) => Err(ValidateError::ShaclParser(error)),
        }
    }
}
