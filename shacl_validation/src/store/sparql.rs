use iri_s::IriS;
use srdf::SRDFSparql;

use crate::validate_error::ValidateError;

pub struct Sparql {
    store: SRDFSparql,
}

impl Sparql {
    pub fn new(path: &str) -> Result<Self, ValidateError> {
        let store = match SRDFSparql::new(&IriS::new_unchecked(path)) {
            Ok(rdf) => rdf,
            Err(_) => return Err(ValidateError::SPARQLCreation),
        };
        Ok(Self { store })
    }

    pub(crate) fn store(&self) -> &SRDFSparql {
        &self.store
    }
}
