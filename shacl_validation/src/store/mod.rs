use std::path::Path;
use std::str::FromStr;

use oxiri::Iri;
use shacl_ast::Schema;
use shacl_ast::ShaclParser;
use srdf::RDFFormat;
use srdf::SRDFGraph;

use crate::validate_error::ValidateError;

pub mod graph;
pub mod sparql;

pub trait Store<S> {
    fn store(&self) -> &S;
}

pub struct ShaclDataManager;

impl ShaclDataManager {
    pub fn load(
        path: &Path,
        rdf_format: RDFFormat,
        base: Option<&str>,
    ) -> Result<Schema, ValidateError> {
        let rdf = SRDFGraph::from_path(
            path,
            &rdf_format,
            match base {
                Some(base) => Some(Iri::from_str(base)?),
                None => None,
            },
        )?;

        match ShaclParser::new(rdf).parse() {
            Ok(schema) => Ok(schema),
            Err(_) => Err(ValidateError::GraphCreation),
        }
    }
}
