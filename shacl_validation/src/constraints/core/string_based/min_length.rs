use std::collections::HashSet;

use indoc::formatdoc;
use srdf::{SRDFBasic, SRDF};

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::ConstraintComponent;
use crate::helper::oxigraph::ask;
use crate::helper::term::Term;
use crate::runner::oxigraph::OxigraphStore;
use crate::validation_report::report::ValidationReport;

/// sh:minLength specifies the minimum string length of each value node that
/// satisfies the condition. This can be applied to any literals and IRIs, but
/// not to blank nodes.
///
/// https://www.w3.org/TR/shacl/#MinLengthConstraintComponent
pub(crate) struct MinLength {
    min_length: isize,
}

impl MinLength {
    pub fn new(min_length: isize) -> Self {
        MinLength { min_length }
    }
}

impl<S: SRDF + SRDFBasic> ConstraintComponent<S> for MinLength {
    fn evaluate(
        &self,
        _store: &S,
        _value_nodes: HashSet<Term>,
        _report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        Err(ConstraintError::NotImplemented)
    }
}

impl<'a> ConstraintComponent<OxigraphStore<'a>> for MinLength {
    fn evaluate(
        &self,
        store: &OxigraphStore<'a>,
        value_nodes: HashSet<Term>,
        report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        for node in &value_nodes {
            if node.is_blank_node() || node.is_triple() {
                <MinLength as ConstraintComponent<OxigraphStore<'a>>>::make_validation_result(
                    self,
                    Some(node),
                    report,
                );
            } else {
                let query = formatdoc! {
                    " ASK {{ FILTER (STRLEN(str({})) >= {}) }} ",
                    node, self.min_length
                };
                if !ask(store, query)? {
                    <MinLength as ConstraintComponent<OxigraphStore<'a>>>::make_validation_result(
                        self,
                        Some(node),
                        report,
                    );
                }
            }
        }
        Ok(())
    }
}
