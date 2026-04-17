use sparql_service::RdfData;
use crate::validator::engine::{Engine, NativeEngine, SparqlEngine};
use crate::validator::processor::ShaclProcessor;
use crate::validator::ShaclValidationMode;

// TODO - move to validation::algorithms module
#[derive(Debug)]
pub struct DataValidation {
    data: RdfData,
}

impl DataValidation {
    pub fn new(data: RdfData) -> Self {
        Self { data }
    }
}

impl ShaclProcessor<RdfData> for DataValidation {
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

impl From<RdfData> for DataValidation {
    fn from(value: RdfData) -> Self {
        Self::new(value)
    }
}