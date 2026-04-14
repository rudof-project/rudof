use prefixmap::PrefixMap;
use rudof_rdf::rdf_impl::SparqlEndpoint;
use sparql_service::RdfData;
use crate::ir::IRSchema;
use crate::validation::engine::{Engine, NativeEngine, SparqlEngine};
use crate::validation::error::ValidationError;
use crate::validation::mode::ShaclValidationMode;
use crate::validation::processor::ShaclProcessor;
use crate::validation::report::ValidationReport;
use crate::validation::store::{Endpoint, Store};
use crate::validation::validator::Validate;

// TODO - Move to validation::algorithms module
/// The endpoint Graph Validation Algorithm
pub(crate) struct EndpointValidation {
    store: Endpoint,
}

impl EndpointValidation {
    pub fn new(iri: &str, pm: &PrefixMap) -> Result<Self, ValidationError> {
        Ok(Self { store: Endpoint::new(iri, pm)? })
    }
}

impl ShaclProcessor<SparqlEndpoint> for EndpointValidation {
    fn store(&self) -> &SparqlEndpoint {
        self.store.store()
    }

    fn runner(mode: &ShaclValidationMode) -> Box<dyn Engine<SparqlEndpoint>> {
        match mode {
            ShaclValidationMode::Native => Box::new(NativeEngine::new()),
            ShaclValidationMode::Sparql => Box::new(SparqlEngine::new()),
        }
    }
}

impl From<SparqlEndpoint> for EndpointValidation {
    fn from(value: SparqlEndpoint) -> Self {
        Self { store: value.into() }
    }
}