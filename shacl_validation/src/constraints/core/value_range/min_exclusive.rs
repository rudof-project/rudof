use std::collections::HashSet;

use indoc::formatdoc;
use srdf::literal::Literal;
use srdf::{SRDFBasic, SRDF};

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::ConstraintComponent;
use crate::helper::oxigraph::ask;
use crate::helper::term::Term;
use crate::runner::oxigraph::OxigraphStore;
use crate::validation_report::report::ValidationReport;

/// https://www.w3.org/TR/shacl/#MinExclusiveConstraintComponent
pub(crate) struct MinExclusive {
    literal: Literal,
}

impl MinExclusive {
    pub fn new(literal: Literal) -> Self {
        MinExclusive { literal }
    }
}

impl<S: SRDF + SRDFBasic> ConstraintComponent<S> for MinExclusive {
    fn evaluate(
        &self,
        _store: &S,
        _value_nodes: HashSet<Term>,
        _report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        Err(ConstraintError::NotImplemented)
    }
}

impl<'a> ConstraintComponent<OxigraphStore<'a>> for MinExclusive {
    fn evaluate(
        &self,
        store: &OxigraphStore<'a>,
        value_nodes: HashSet<Term>,
        report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        for node in &value_nodes {
            let query = formatdoc! {
                " ASK {{ FILTER ({} < {}) }} ",
                node, self.literal
            };
            println!("{}", query);
            if !ask(store, query)? {
                <MinExclusive as ConstraintComponent<OxigraphStore<'a>>>::make_validation_result(
                    self,
                    Some(node),
                    report,
                );
            }
        }
        Ok(())
    }
}
