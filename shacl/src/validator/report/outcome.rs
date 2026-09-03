use crate::validator::report::{Evidence, ValidationResult};

/// Accumulates both violations and evidence while validating a `(node,
/// shape)` pair, threaded through the validator in place of a bare
/// `Vec<ValidationResult>`.
///
/// Both vectors are always populated internally regardless of the user's
/// `store_errors`/`store_evidences` config — that config only filters what
/// ends up in the final [`crate::validator::report::ValidationReport`].
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ValidationOutcome {
    violations: Vec<ValidationResult>,
    evidences: Vec<Evidence>,
}

impl ValidationOutcome {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_violation(violation: ValidationResult) -> Self {
        Self {
            violations: vec![violation],
            evidences: Vec::new(),
        }
    }

    pub fn from_evidence(evidence: Evidence) -> Self {
        Self {
            violations: Vec::new(),
            evidences: vec![evidence],
        }
    }

    pub fn from_violations(violations: Vec<ValidationResult>) -> Self {
        Self {
            violations,
            evidences: Vec::new(),
        }
    }

    pub fn from_evidences(evidences: Vec<Evidence>) -> Self {
        Self {
            violations: Vec::new(),
            evidences,
        }
    }

    pub fn push_violation(&mut self, violation: ValidationResult) {
        self.violations.push(violation);
    }

    pub fn push_evidence(&mut self, evidence: Evidence) {
        self.evidences.push(evidence);
    }

    pub fn extend(&mut self, other: ValidationOutcome) {
        self.violations.extend(other.violations);
        self.evidences.extend(other.evidences);
    }

    /// A `(node, shape)` pair conforms iff it produced no violations.
    pub fn conforms(&self) -> bool {
        self.violations.is_empty()
    }

    pub fn is_empty(&self) -> bool {
        self.violations.is_empty() && self.evidences.is_empty()
    }

    pub fn violations(&self) -> &[ValidationResult] {
        &self.violations
    }

    pub fn evidences(&self) -> &[Evidence] {
        &self.evidences
    }

    pub fn into_parts(self) -> (Vec<ValidationResult>, Vec<Evidence>) {
        (self.violations, self.evidences)
    }
}

impl FromIterator<ValidationOutcome> for ValidationOutcome {
    fn from_iter<I: IntoIterator<Item = ValidationOutcome>>(iter: I) -> Self {
        let mut acc = ValidationOutcome::new();
        for outcome in iter {
            acc.extend(outcome);
        }
        acc
    }
}

impl Extend<ValidationOutcome> for ValidationOutcome {
    fn extend<I: IntoIterator<Item = ValidationOutcome>>(&mut self, iter: I) {
        for outcome in iter {
            ValidationOutcome::extend(self, outcome);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Severity;
    use rudof_iri::IriS;

    fn obj(s: &str) -> rudof_rdf::rdf_core::term::Object {
        rudof_rdf::rdf_core::term::Object::iri(IriS::new_unchecked(s))
    }

    #[test]
    fn empty_outcome_conforms() {
        let outcome = ValidationOutcome::new();
        assert!(outcome.conforms());
        assert!(outcome.is_empty());
    }

    #[test]
    fn outcome_with_violation_does_not_conform() {
        let vr = ValidationResult::new(obj("http://ex/n"), obj("http://ex/c"), Severity::Violation);
        let outcome = ValidationOutcome::from_violation(vr);
        assert!(!outcome.conforms());
        assert_eq!(outcome.violations().len(), 1);
    }

    #[test]
    fn outcome_with_only_evidence_conforms() {
        let ev = Evidence::new(obj("http://ex/n"), obj("http://ex/c"));
        let outcome = ValidationOutcome::from_evidence(ev);
        assert!(outcome.conforms());
        assert_eq!(outcome.evidences().len(), 1);
    }

    #[test]
    fn extend_merges_both_vectors() {
        let vr = ValidationResult::new(obj("http://ex/n"), obj("http://ex/c"), Severity::Violation);
        let ev = Evidence::new(obj("http://ex/n2"), obj("http://ex/c2"));
        let mut a = ValidationOutcome::from_violation(vr);
        let b = ValidationOutcome::from_evidence(ev);
        a.extend(b);
        assert_eq!(a.violations().len(), 1);
        assert_eq!(a.evidences().len(), 1);
        assert!(!a.conforms());
    }

    #[test]
    fn from_iter_collects_across_outcomes() {
        let outcomes = vec![
            ValidationOutcome::from_evidence(Evidence::new(obj("http://ex/n1"), obj("http://ex/c"))),
            ValidationOutcome::from_evidence(Evidence::new(obj("http://ex/n2"), obj("http://ex/c"))),
        ];
        let acc: ValidationOutcome = outcomes.into_iter().collect();
        assert_eq!(acc.evidences().len(), 2);
        assert!(acc.conforms());
    }
}
