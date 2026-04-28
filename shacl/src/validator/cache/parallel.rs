use crate::ir::ShapeLabelIdx;
use crate::validator::cache::ValidationCache;
use crate::validator::report::ValidationResult;
use dashmap::DashMap;
use rudof_rdf::rdf_core::term::Object;
use std::collections::HashMap;

/// Multi-threaded validation cache
///
/// The cache is designed to be wrapped in an [`std::sync::Arc`]
#[derive(Debug, Default)]
pub(crate) struct ParallelValidationCache {
    cache: DashMap<ShapeLabelIdx, HashMap<Object, Vec<ValidationResult>>>,
}

impl ParallelValidationCache {
    pub fn new() -> Self {
        Self { cache: DashMap::new() }
    }
}

impl ValidationCache for ParallelValidationCache {
    fn record(&self, node: Object, shape_idx: ShapeLabelIdx, results: Vec<ValidationResult>) {
        self.cache.entry(shape_idx).or_default().insert(node, results);
    }

    fn has_validated(&self, node: &Object, shape_idx: ShapeLabelIdx) -> bool {
        self.cache.get(&shape_idx).map_or(false, |m| m.contains_key(node))
    }

    fn get_results(&self, node: &Object, shape_idx: ShapeLabelIdx) -> Option<Vec<ValidationResult>> {
        self.cache.get(&shape_idx)?.get(node).cloned()
    }
}
