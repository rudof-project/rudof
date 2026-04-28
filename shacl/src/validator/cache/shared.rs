use crate::ir::ShapeLabelIdx;
use crate::validator::cache::ValidationCache;
use crate::validator::cache::parallel::ParallelValidationCache;
use crate::validator::report::ValidationResult;
use rudof_rdf::rdf_core::term::Object;
use std::sync::Arc;

/// A cheapy-cloneable wrapper for [`ParallelValidationCache`] shared behind an [`Arc`]
#[derive(Debug, Clone, Default)]
pub(crate) struct SharedValidationCache(Arc<ParallelValidationCache>);

impl SharedValidationCache {
    pub fn new() -> Self {
        Self(Arc::new(ParallelValidationCache::new()))
    }
}

impl ValidationCache for SharedValidationCache {
    fn record(&self, node: Object, shape_idx: ShapeLabelIdx, results: Vec<ValidationResult>) {
        self.0.record(node, shape_idx, results);
    }

    fn has_validated(&self, node: &Object, shape_idx: ShapeLabelIdx) -> bool {
        self.0.has_validated(node, shape_idx)
    }

    fn get_results(&self, node: &Object, shape_idx: ShapeLabelIdx) -> Option<Vec<ValidationResult>> {
        self.0.get_results(node, shape_idx)
    }
}
