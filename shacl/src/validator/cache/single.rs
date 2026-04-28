use crate::ir::ShapeLabelIdx;
use crate::validator::cache::ValidationCache;
use crate::validator::report::ValidationResult;
use rudof_rdf::rdf_core::term::Object;
use std::collections::HashMap;
use std::sync::Mutex;

/// Single-threaded validation cache
///
/// Use [`crate::validator::cache::ParallelValidationCache`] when cache must
/// be shared across threads
#[allow(dead_code)]
#[derive(Debug, Default)]
pub(crate) struct SingleValidationCache {
    cache: Mutex<HashMap<ShapeLabelIdx, HashMap<Object, Vec<ValidationResult>>>>,
}

#[allow(dead_code)]
impl SingleValidationCache {
    pub fn new() -> Self {
        Self {
            cache: Mutex::new(HashMap::new()),
        }
    }
}

impl ValidationCache for SingleValidationCache {
    fn record(&self, node: Object, shape_idx: ShapeLabelIdx, results: Vec<ValidationResult>) {
        self.cache
            .lock()
            .expect("SingleValidationCache lock poisoned")
            .entry(shape_idx)
            .or_default()
            .insert(node, results);
    }

    fn has_validated(&self, node: &Object, shape_idx: ShapeLabelIdx) -> bool {
        self.cache
            .lock()
            .expect("SingleValidationCache lock poisoned")
            .get(&shape_idx)
            .is_some_and(|m| m.contains_key(node))
    }

    fn get_results(&self, node: &Object, shape_idx: ShapeLabelIdx) -> Option<Vec<ValidationResult>> {
        self.cache
            .lock()
            .expect("SingleValidationCache lock poisoned")
            .get(&shape_idx)?
            .get(node)
            .cloned()
    }
}
