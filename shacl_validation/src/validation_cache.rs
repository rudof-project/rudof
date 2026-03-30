use crate::validation_report::result::ValidationResult;
use rudof_rdf::rdf_core::term::Object;
use shacl::ir::ShapeLabelIdx;
use std::collections::HashMap;

/// A shared cache for SHACL validation results.
///
/// This cache stores `(node, shape_idx) → Vec<ValidationResult>` mappings
/// to avoid redundant validation of the same node against the same shape.
///
/// It is used across the entire validation process by both `NativeEngine` and `SparqlEngine`.
#[derive(Debug, Clone, Default)]
pub struct ValidationCache {
    cache: HashMap<(Object, ShapeLabelIdx), Vec<ValidationResult>>,
}

impl ValidationCache {
    pub fn new() -> Self {
        Self { cache: HashMap::new() }
    }

    /// Record the validation results for a given `(node, shape_idx)` pair.
    pub fn record(&mut self, node: Object, shape_idx: ShapeLabelIdx, results: Vec<ValidationResult>) {
        self.cache.insert((node, shape_idx), results);
    }

    /// Check whether a given `(node, shape_idx)` pair has already been validated.
    pub fn has_validated(&self, node: &Object, shape_idx: ShapeLabelIdx) -> bool {
        self.cache.contains_key(&(node.clone(), shape_idx))
    }

    /// Get the cached validation results for a given `(node, shape_idx)` pair, if any.
    pub fn get_results(&self, node: &Object, shape_idx: ShapeLabelIdx) -> Option<&Vec<ValidationResult>> {
        self.cache.get(&(node.clone(), shape_idx))
    }
}
