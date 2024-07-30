use std::collections::HashSet;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::ConstraintComponent;
use crate::helper::term::Term;
use crate::validation_report::report::ValidationReport;

/// The property sh:uniqueLang can be set to true to specify that no pair of
///  value nodes may use the same language tag.
///
/// https://www.w3.org/TR/shacl/#UniqueLangConstraintComponent
pub(crate) struct UniqueLang {
    unique_lang: bool,
}

impl UniqueLang {
    pub fn new(unique_lang: bool) -> Self {
        UniqueLang { unique_lang }
    }
}

impl<S> ConstraintComponent<S> for UniqueLang {
    fn evaluate(
        &self,
        _: &S,
        value_nodes: HashSet<Term>,
        report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        if self.unique_lang {
            let mut langs = HashSet::new();
            for node in &value_nodes {
                if let Term::Literal(literal) = node {
                    if langs.contains(&literal.lang()) {
                        <UniqueLang as ConstraintComponent<S>>::make_validation_result(
                            self,
                            Some(node),
                            report,
                        )
                    } else {
                        langs.insert(literal.lang());
                    }
                }
            }
        }
        Ok(())
    }
}
