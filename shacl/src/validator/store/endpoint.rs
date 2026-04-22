use crate::error::ValidationError;
use crate::validator::store::Store;
use rudof_iri::iri;
use prefixmap::PrefixMap;
use rudof_rdf::rdf_impl::SparqlEndpoint;

#[derive(Debug, Clone)]
pub struct Endpoint {
    store: SparqlEndpoint,
}

impl Endpoint {
    pub fn new(iri: &str, pm: &PrefixMap) -> Result<Self, ValidationError> {
        match SparqlEndpoint::new(&iri!(iri), pm) {
            Ok(store) => Ok(Self { store }),
            Err(_) => Err(ValidationError::SparqlCreation),
        }
    }
}

impl From<SparqlEndpoint> for Endpoint {
    fn from(value: SparqlEndpoint) -> Self {
        Self { store: value }
    }
}

impl Store<SparqlEndpoint> for Endpoint {
    fn store(&self) -> &SparqlEndpoint {
        &self.store
    }
}
