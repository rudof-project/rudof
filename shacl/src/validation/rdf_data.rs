use sparql_service::RdfData;
use crate::ir::IRSchema;
use crate::validation::engine::{Engine, NativeEngine, SparqlEngine};
use crate::validation::error::ValidationError;
use crate::validation::mode::ShaclValidationMode;
use crate::validation::processor::ShaclProcessor;
use crate::validation::report::ValidationReport;
use crate::validation::validator::Validate;

// TODO - move to validation::algorithms module
#[derive(Debug)]
pub(crate) struct ValidationData {
    data: RdfData,
}

impl ValidationData {
    pub fn new(data: RdfData) -> Self {
        Self { data }
    }
}

impl ShaclProcessor<RdfData> for ValidationData {
    fn store(&self) -> &RdfData {
        &self.data
    }

    fn runner(mode: &ShaclValidationMode) -> Box<dyn Engine<RdfData>> {
        match mode {
            ShaclValidationMode::Native => Box::new(NativeEngine::new()),
            ShaclValidationMode::Sparql => Box::new(SparqlEngine::new()),
        }
    }
}

impl From<RdfData> for ValidationData {
    fn from(value: RdfData) -> Self {
        Self::new(value)
    }
}