use iri_s::IriS;
use srdf::SRDFSparql;

use crate::validate_error::ValidateError;

use super::Store;

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
}

impl Store<SRDFSparql> for Sparql {
    fn store(&self) -> &SRDFSparql {
        &self.store
    }
}
