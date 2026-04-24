use rudof_rdf::rdf_core::term::Object;
use crate::ir::ShapeLabelIdx;
use crate::validator::report::ValidationResult;

mod parallel;
mod shared;
mod single;

pub(crate) use parallel::ParallelValidationCache;
pub(crate) use shared::SharedValidationCache;
pub(crate) use single::SingleValidationCache;

/// Validation cache trait
pub(crate) trait ValidationCache: Send + Sync {
    /// Record the validation results for a `(node, shape_idx)` pair.
    fn record(&self, node: Object, shape_idx: ShapeLabelIdx, results: Vec<ValidationResult>);

    /// Returns `true` if `(node, shape_idx)` has already been validated.
    fn has_validated(&self, node: &Object, shape_idx: ShapeLabelIdx) -> bool;

    /// Returns the cached results for `(node, shape_idx)`, if any.
    ///
    /// Returns an owned [`Vec`] to avoid typing the lifetime to an internal lock guard.
    fn get_results(&self, node: &Object, shape_idx: ShapeLabelIdx) -> Option<Vec<ValidationResult>>;
}
