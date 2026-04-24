use std::collections::HashMap;
use std::sync::Mutex;
use rudof_rdf::rdf_core::term::Object;
use crate::ir::ShapeLabelIdx;
use crate::validator::cache::ValidationCache;
use crate::validator::report::ValidationResult;

/// Single-threaded validation cache
///
/// Use [`crate::validator::cache::ParallelValidationCache`] when cache must
/// be shared across threads
#[derive(Debug, Default)]
pub(crate) struct SingleValidationCache {
    cache: Mutex<HashMap<(Object, ShapeLabelIdx), Vec<ValidationResult>>>,
}

impl SingleValidationCache {
    pub fn new() -> Self {
        Self { cache: Mutex::new(HashMap::new()) }
    }
}

impl ValidationCache for SingleValidationCache {
    fn record(&self, node: Object, shape_idx: ShapeLabelIdx, results: Vec<ValidationResult>) {
        self
            .cache
            .lock()
            .expect("SingleValidationCache lock poisoned")
            .insert((node, shape_idx), results);
    }

    fn has_validated(&self, node: &Object, shape_idx: ShapeLabelIdx) -> bool {
        self
            .cache
            .lock()
            .expect("SingleValidationCache lock poisoned")
            .contains_key(&(node.clone(), shape_idx))
    }

    fn get_results(&self, node: &Object, shape_idx: ShapeLabelIdx) -> Option<Vec<ValidationResult>> {
        self
            .cache
            .lock()
            .expect("SingleValidationCache lock poisoned")
            .get(&(node.clone(), shape_idx))
            .cloned()
    }
}