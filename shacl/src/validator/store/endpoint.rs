use crate::error::ValidationError;
use crate::validator::store::Store;
use prefixmap::PrefixMap;
use rudof_iri::iri;
use rudof_rdf::rdf_impl::OxigraphEndpoint;

#[derive(Debug, Clone)]
pub struct Endpoint {
    store: OxigraphEndpoint,
}

impl Endpoint {
    pub fn new(iri: &str, pm: &PrefixMap) -> Result<Self, ValidationError> {
        match OxigraphEndpoint::new(&iri!(iri), pm) {
            Ok(store) => Ok(Self { store }),
            Err(e) => Err(e.into()),
        }
    }
}

impl From<OxigraphEndpoint> for Endpoint {
    fn from(value: OxigraphEndpoint) -> Self {
        Self { store: value }
    }
}

impl Store<OxigraphEndpoint> for Endpoint {
    fn store(&self) -> &OxigraphEndpoint {
        &self.store
    }
}
