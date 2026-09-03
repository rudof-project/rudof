use crate::ir::ShapeLabelIdx;
use crate::validator::report::ValidationOutcome;
use crate::validator::typing::{ObservableTyping, Typing, Verdict};
use either::Either;
use rudof_rdf::rdf_core::term::Object;
use std::sync::{Arc, Mutex};

/// A cheaply-cloneable, thread-safe wrapper around [`ObservableTyping`],
/// shared across the engines that validate a topological level of shapes
/// in parallel (see `crate::validator::processor::ShaclProcessor::validate`).
///
/// `rudof_typing::Typing::insert` takes `&mut self`, which each engine fork
/// satisfies trivially since it owns its own `SharedTyping` binding — the
/// mutation is still visible across forks because the underlying
/// `ObservableTyping` is shared behind the `Arc<Mutex<_>>`.
#[derive(Debug, Clone, Default)]
pub(crate) struct SharedTyping(Arc<Mutex<ObservableTyping>>);

impl SharedTyping {
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(ObservableTyping::default())))
    }

    /// Records the outcome of validating `(node, shape_idx)`. Converts the
    /// paired `ValidationOutcome` (both violations and evidence, always
    /// computed) into the cache's `Either`-based [`Verdict`]: a pair either
    /// conforms (evidence) or doesn't (violations), never both.
    pub fn record(&self, node: Object, shape_idx: ShapeLabelIdx, outcome: ValidationOutcome) {
        let (violations, evidences) = outcome.into_parts();
        let verdict: Verdict = if violations.is_empty() {
            Either::Right(evidences)
        } else {
            Either::Left(violations)
        };
        self.0
            .lock()
            .expect("SharedTyping lock poisoned")
            .insert((node, shape_idx), verdict);
    }

    pub fn has_validated(&self, node: &Object, shape_idx: ShapeLabelIdx) -> bool {
        self.0
            .lock()
            .expect("SharedTyping lock poisoned")
            .get(&(node.clone(), shape_idx))
            .is_some()
    }

    pub fn get_outcome(&self, node: &Object, shape_idx: ShapeLabelIdx) -> Option<ValidationOutcome> {
        let verdict = self
            .0
            .lock()
            .expect("SharedTyping lock poisoned")
            .get(&(node.clone(), shape_idx))
            .cloned()?;
        Some(verdict_to_outcome(verdict))
    }
}

fn verdict_to_outcome(verdict: Verdict) -> ValidationOutcome {
    match verdict {
        Either::Left(violations) => ValidationOutcome::from_violations(violations),
        Either::Right(evidences) => ValidationOutcome::from_evidences(evidences),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Severity;
    use crate::validator::report::{Evidence, ValidationResult};
    use rudof_iri::IriS;

    fn obj(s: &str) -> Object {
        Object::iri(IriS::new_unchecked(s))
    }

    #[test]
    fn unrecorded_pair_is_not_validated_and_has_no_cached_outcome() {
        let typing = SharedTyping::new();
        let idx = ShapeLabelIdx::new(0);
        assert!(!typing.has_validated(&obj("http://ex/n"), idx));
        assert!(typing.get_outcome(&obj("http://ex/n"), idx).is_none());
    }

    #[test]
    fn recording_a_conformant_outcome_caches_evidence_only() {
        let typing = SharedTyping::new();
        let idx = ShapeLabelIdx::new(0);
        let node = obj("http://ex/n");
        let evidence = Evidence::new(node.clone(), obj("http://ex/comp"));
        typing.record(node.clone(), idx, ValidationOutcome::from_evidence(evidence));

        assert!(typing.has_validated(&node, idx));
        let cached = typing.get_outcome(&node, idx).unwrap();
        assert!(cached.conforms());
        assert_eq!(cached.evidences().len(), 1);
        assert!(cached.violations().is_empty());
    }

    #[test]
    fn recording_a_non_conformant_outcome_caches_violations_only() {
        let typing = SharedTyping::new();
        let idx = ShapeLabelIdx::new(0);
        let node = obj("http://ex/n");
        let violation = ValidationResult::new(node.clone(), obj("http://ex/comp"), Severity::Violation);
        typing.record(node.clone(), idx, ValidationOutcome::from_violation(violation));

        let cached = typing.get_outcome(&node, idx).unwrap();
        assert!(!cached.conforms());
        assert_eq!(cached.violations().len(), 1);
        assert!(cached.evidences().is_empty());
    }

    #[test]
    fn clones_share_the_same_underlying_cache() {
        let typing = SharedTyping::new();
        let clone = typing.clone();
        let idx = ShapeLabelIdx::new(0);
        let node = obj("http://ex/n");
        clone.record(
            node.clone(),
            idx,
            ValidationOutcome::from_evidence(Evidence::new(node.clone(), obj("http://ex/comp"))),
        );
        assert!(typing.has_validated(&node, idx));
    }
}
