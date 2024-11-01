use iri_s::IriS;
use prefixmap::PrefixMap;
use srdf::SRDFSparql;

use crate::validate_error::ValidateError;

use super::Store;

#[derive(Debug, Clone)]
pub struct Endpoint {
    store: SRDFSparql,
}

impl Endpoint {
    pub fn new(iri: &str, prefixmap: &PrefixMap) -> Result<Self, ValidateError> {
        match SRDFSparql::new(&IriS::new_unchecked(iri), prefixmap) {
            Ok(store) => Ok(Self { store }),
            Err(_) => Err(ValidateError::SPARQLCreation),
        }
    }

    pub fn from_sparql(sparql: SRDFSparql) -> Endpoint {
        Endpoint { store: sparql }
    }
}

impl Store<SRDFSparql> for Endpoint {
    fn store(&self) -> &SRDFSparql {
        &self.store
    }
}
