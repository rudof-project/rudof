use crate::error::ValidationError;
use crate::validator::ShaclValidationMode;
use crate::validator::engine::{Engine, NativeEngine, SparqlEngine};
use crate::validator::processor::ShaclProcessor;
use crate::validator::store::{Endpoint, Store};
use prefixmap::PrefixMap;
use rudof_rdf::rdf_impl::OxigraphEndpoint;

// TODO - Move to validation::algorithms module
/// The endpoint Graph Validation Algorithm
pub struct EndpointValidation {
    store: Endpoint,
}

impl EndpointValidation {
    pub fn new(iri: &str, pm: &PrefixMap) -> Result<Self, ValidationError> {
        Ok(Self {
            store: Endpoint::new(iri, pm)?,
        })
    }
}

impl ShaclProcessor<OxigraphEndpoint> for EndpointValidation {
    fn store(&self) -> &OxigraphEndpoint {
        self.store.store()
    }

    fn runner(mode: &ShaclValidationMode) -> Box<dyn Engine<OxigraphEndpoint>> {
        match mode {
            ShaclValidationMode::Native => Box::new(NativeEngine::new()),
            ShaclValidationMode::Sparql => Box::new(SparqlEngine::new()),
        }
    }
}

impl From<OxigraphEndpoint> for EndpointValidation {
    fn from(value: OxigraphEndpoint) -> Self {
        Self { store: value.into() }
    }
}
