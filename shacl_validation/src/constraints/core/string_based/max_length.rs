use std::collections::HashSet;

use indoc::formatdoc;
use srdf::{SRDFBasic, SRDF};

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::ConstraintComponent;
use crate::helper::oxigraph::ask;
use crate::helper::term::Term;
use crate::runner::oxigraph::OxigraphStore;
use crate::validation_report::report::ValidationReport;

/// sh:maxLength specifies the maximum string length of each value node that
/// satisfies the condition. This can be applied to any literals and IRIs, but
/// not to blank nodes.
///
/// https://www.w3.org/TR/shacl/#MaxLengthConstraintComponent
pub(crate) struct MaxLength {
    max_length: isize,
}

impl MaxLength {
    pub fn new(max_length: isize) -> Self {
        MaxLength { max_length }
    }
}

impl<S: SRDF + SRDFBasic> ConstraintComponent<S> for MaxLength {
    fn evaluate(
        &self,
        _store: &S,
        _value_nodes: HashSet<Term>,
        _report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        Err(ConstraintError::NotImplemented)
    }
}

impl<'a> ConstraintComponent<OxigraphStore<'a>> for MaxLength {
    fn evaluate(
        &self,
        store: &OxigraphStore<'a>,
        value_nodes: HashSet<Term>,
        report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        for node in &value_nodes {
            if node.is_blank_node() || node.is_triple() {
                <MaxLength as ConstraintComponent<OxigraphStore<'a>>>::make_validation_result(
                    self,
                    Some(node),
                    report,
                );
            } else {
                let query = formatdoc! {
                    " ASK {{ FILTER (STRLEN(str({})) <= {}) }} ",
                    node, self.max_length
                };
                if !ask(store, query)? {
                    <MaxLength as ConstraintComponent<OxigraphStore<'a>>>::make_validation_result(
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
