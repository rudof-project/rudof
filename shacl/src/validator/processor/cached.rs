use crate::error::ValidationError;
use crate::ir::IRSchema;
use crate::validator::engine::{Engine, NativeEngine, SparqlEngine};
use crate::validator::processor::ShaclProcessor;
use crate::validator::store::CachedRdfData;
use crate::validator::{FetchStrategy, ShaclConfig, ShaclValidationMode, prefetch};
use sparql_service::RdfData;

/// Validation of data that comes, at least partly, from remote SPARQL
/// endpoints. The data is read through a [`CachedRdfData`], so that no lookup
/// is sent twice, and is never scanned as a whole. With
/// [`FetchStrategy::ByShape`], the data each shape needs is fetched in
/// advance, many nodes per query (see [`prefetch`]).
#[derive(Debug)]
pub struct CachedDataValidation {
    store: CachedRdfData,
}

impl CachedDataValidation {
    pub fn new(data: RdfData) -> Self {
        Self {
            store: CachedRdfData::new(data),
        }
    }

    pub fn store(&self) -> &CachedRdfData {
        &self.store
    }
}

impl ShaclProcessor<CachedRdfData> for CachedDataValidation {
    fn store(&self) -> &CachedRdfData {
        &self.store
    }

    fn runner(mode: &ShaclValidationMode, config: &ShaclConfig) -> Box<dyn Engine<CachedRdfData>> {
        match mode {
            ShaclValidationMode::Native => Box::new(NativeEngine::new(config.recursion_semantics())),
            ShaclValidationMode::Sparql => Box::new(SparqlEngine::new(config.recursion_semantics())),
        }
    }

    fn prepare_store(&mut self) -> Result<(), ValidationError> {
        self.store.inner_mut().check_store().map_err(ValidationError::from)
    }

    fn use_class_index(&self) -> bool {
        false
    }

    fn before_validation(
        &mut self,
        shapes_graph: &IRSchema,
        runner: &dyn Engine<CachedRdfData>,
        config: &ShaclConfig,
    ) -> Result<(), ValidationError> {
        if config.fetch_strategy() == FetchStrategy::ByShape {
            let store = std::mem::replace(&mut self.store, CachedRdfData::new(RdfData::new()));
            self.store = store.with_batch_size(config.prefetch_batch_size());
            prefetch::prefetch(&self.store, shapes_graph, runner, config.prefetch_max_rounds())?;
        }
        Ok(())
    }
}

impl From<RdfData> for CachedDataValidation {
    fn from(data: RdfData) -> Self {
        Self::new(data)
    }
}
