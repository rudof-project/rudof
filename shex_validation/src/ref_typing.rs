use crate::validator_error::ValidatorError;
use shex_ast::Node;
use shex_ast::ShapeLabelIdx;
use std::collections::HashMap;
use std::collections::HashSet;

/// Tracks which `(Node, ShapeLabelIdx)` pairs have already been proved (`passed`),
/// and, for pairs whose proof was attempted and failed, the errors that caused
/// the failure. This lets callers that consume a failed pending reference (e.g.
/// `FailedPending`) explain *why* the reference failed, not just that it did.
#[derive(Debug, Clone, Default)]
pub(crate) struct RefTyping {
    passed: HashSet<(Node, ShapeLabelIdx)>,
    errors: HashMap<(Node, ShapeLabelIdx), Vec<ValidatorError>>,
}

impl RefTyping {
    pub(crate) fn new() -> Self {
        RefTyping::default()
    }

    pub(crate) fn contains(&self, pair: &(Node, ShapeLabelIdx)) -> bool {
        self.passed.contains(pair)
    }

    pub(crate) fn insert_passed(&mut self, node: Node, idx: ShapeLabelIdx) {
        self.passed.insert((node, idx));
    }

    pub(crate) fn insert_failed(&mut self, node: Node, idx: ShapeLabelIdx, errors: Vec<ValidatorError>) {
        self.errors.entry((node, idx)).or_default().extend(errors);
    }

    pub(crate) fn errors_for(&self, pair: &(Node, ShapeLabelIdx)) -> Vec<ValidatorError> {
        self.errors.get(pair).cloned().unwrap_or_default()
    }
}
