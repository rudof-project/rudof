use iri_s::IriS;
use prefixmap::PrefixMap;
use rdf::rdf_impl::SparqlEndpoint;

use crate::validate_error::ValidateError;

use super::Store;

#[derive(Debug, Clone)]
pub struct Endpoint {
    store: SparqlEndpoint,
}

impl Endpoint {
    pub fn new(iri: &str, prefixmap: &PrefixMap) -> Result<Self, Box<ValidateError>> {
        match SparqlEndpoint::new(&IriS::new_unchecked(iri), prefixmap) {
            Ok(store) => Ok(Self { store }),
            Err(_) => Err(Box::new(ValidateError::SPARQLCreation)),
        }
    }

    pub fn from_sparql(sparql: SparqlEndpoint) -> Endpoint {
        Endpoint { store: sparql }
    }
}

impl Store<SparqlEndpoint> for Endpoint {
    fn store(&self) -> &SparqlEndpoint {
        &self.store
    }
}
