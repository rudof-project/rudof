use std::collections::HashSet;

use indoc::formatdoc;
use srdf::{SRDFBasic, SRDF};

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::ConstraintComponent;
use crate::helper::oxigraph::ask;
use crate::helper::term::Term;
use crate::runner::oxigraph::OxigraphStore;
use crate::validation_report::report::ValidationReport;

/// sh:property can be used to specify that each value node has a given property
/// shape.
///
/// https://www.w3.org/TR/shacl/#PropertyShapeComponent
pub(crate) struct Pattern {
    pattern: String,
    flags: Option<String>,
}

impl Pattern {
    pub fn new(pattern: String, flags: Option<String>) -> Self {
        Pattern { pattern, flags }
    }
}

impl<S: SRDF + SRDFBasic> ConstraintComponent<S> for Pattern {
    fn evaluate(
        &self,
        _store: &S,
        _value_nodes: HashSet<Term>,
        _report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        Err(ConstraintError::NotImplemented)
    }
}

impl<'a> ConstraintComponent<OxigraphStore<'a>> for Pattern {
    fn evaluate(
        &self,
        store: &OxigraphStore<'a>,
        value_nodes: HashSet<Term>,
        report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        for node in &value_nodes {
            if node.is_blank_node() || node.is_triple() {
                <Pattern as ConstraintComponent<OxigraphStore<'a>>>::make_validation_result(
                    self,
                    Some(node),
                    report,
                );
            } else {
                let query = match &self.flags {
                    Some(flags) => formatdoc! {
                        "ASK {{
                            FILTER (regex(str({}), {}, {}))
                        }}",
                        node, self.pattern, flags
                    },
                    None => formatdoc! {
                        "ASK {{
                            FILTER (regex(str({}), {}))
                        }}",
                        node, self.pattern
                    },
                };
                if !ask(store, query)? {
                    <Pattern as ConstraintComponent<OxigraphStore<'a>>>::make_validation_result(
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
