use iri_s::iri;
use prefixmap::PrefixMap;
use rudof_rdf::rdf_impl::SparqlEndpoint;
use crate::validation::error::ValidationError;
use crate::validation::store::Store;

#[derive(Debug, Clone)]
pub(crate) struct Endpoint {
    store: SparqlEndpoint,
}

impl Endpoint {
    pub fn new(iri: &str, pm: &PrefixMap) -> Result<Self, ValidationError> {
        match SparqlEndpoint::new(&iri!(iri), pm) {
            Ok(store) => Ok(Self { store }),
            Err(_) => Err(ValidationError::SparqlCreation)
        }
    }

    pub fn from_sparql(sparql: SparqlEndpoint) -> Self {
        Self { store: sparql }
    }
}

impl Store<SparqlEndpoint> for Endpoint {
    fn store(&self) -> &SparqlEndpoint {
        &self.store
    }
}