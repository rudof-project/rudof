use crate::ir::ShapeLabelIdx;
use crate::validator::cache::ValidationCache;
use crate::validator::report::ValidationResult;
use dashmap::DashMap;
use rudof_rdf::rdf_core::term::Object;

/// Multi-threaded validation cache
///
/// The cache is designed to be wrapped in an [`std::sync::Arc`]
#[derive(Debug, Default)]
pub(crate) struct ParallelValidationCache {
    cache: DashMap<(Object, ShapeLabelIdx), Vec<ValidationResult>>,
}

impl ParallelValidationCache {
    pub fn new() -> Self {
        Self { cache: DashMap::new() }
    }
}

impl ValidationCache for ParallelValidationCache {
    fn record(&self, node: Object, shape_idx: ShapeLabelIdx, results: Vec<ValidationResult>) {
        self.cache.insert((node, shape_idx), results);
    }

    fn has_validated(&self, node: &Object, shape_idx: ShapeLabelIdx) -> bool {
        self.cache.contains_key(&(node.clone(), shape_idx))
    }

    fn get_results(&self, node: &Object, shape_idx: ShapeLabelIdx) -> Option<Vec<ValidationResult>> {
        self.cache.get(&(node.clone(), shape_idx)).map(|r| r.clone())
    }
}
