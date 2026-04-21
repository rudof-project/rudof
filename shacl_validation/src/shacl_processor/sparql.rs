use crate::shacl_engine::engine::Engine;
use crate::shacl_engine::native::NativeEngine;
use crate::shacl_engine::sparql::SparqlEngine;
use crate::store::Store;
use crate::store::sparql::Endpoint;
use crate::validate_error::ValidateError;
use crate::validation_report::report::ValidationReport;
use prefixmap::PrefixMap;
use rudof_rdf::rdf_impl::SparqlEndpoint;
use shacl_ir::compiled::schema_ir::SchemaIR;
use sparql_service::RdfData;
use std::fmt::Debug;

use super::{ShaclProcessor, ShaclValidationMode, do_validate};

#[derive(Debug)]
pub struct RdfDataValidation {
    data: RdfData,
    mode: ShaclValidationMode,
}

impl RdfDataValidation {
    pub fn from_rdf_data(data: RdfData, mode: ShaclValidationMode) -> Self {
        Self { data, mode }
    }
}

impl ShaclProcessor<RdfData> for RdfDataValidation {
    fn validate(&mut self, shapes_graph: &SchemaIR) -> Result<ValidationReport, Box<ValidateError>> {
        let runner: Box<dyn Engine<RdfData>> = match self.mode {
            ShaclValidationMode::Native => Box::new(NativeEngine::new()),
            ShaclValidationMode::Sparql => Box::new(SparqlEngine::new()),
        };
        do_validate(&self.data, runner, shapes_graph)
    }
}

/// The Endpoint Graph Validation algorithm.
pub struct EndpointValidation {
    store: Endpoint,
    mode: ShaclValidationMode,
}

impl EndpointValidation {
    pub fn new(iri: &str, prefixmap: &PrefixMap, mode: ShaclValidationMode) -> Result<Self, Box<ValidateError>> {
        Ok(EndpointValidation {
            store: Endpoint::new(iri, prefixmap)?,
            mode,
        })
    }

    pub fn from_sparql(sparql: SparqlEndpoint, mode: ShaclValidationMode) -> Result<Self, Box<ValidateError>> {
        let store = Endpoint::from_sparql(sparql);
        Ok(EndpointValidation { store, mode })
    }
}

impl ShaclProcessor<SparqlEndpoint> for EndpointValidation {
    fn validate(&mut self, shapes_graph: &SchemaIR) -> Result<ValidationReport, Box<ValidateError>> {
        let store = self.store.store();
        let runner: Box<dyn Engine<SparqlEndpoint>> = match self.mode {
            ShaclValidationMode::Native => Box::new(NativeEngine::new()),
            ShaclValidationMode::Sparql => Box::new(SparqlEngine::new()),
        };
        do_validate(store, runner, shapes_graph)
    }
}
