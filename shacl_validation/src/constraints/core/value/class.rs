use std::collections::HashSet;

use indoc::formatdoc;
use srdf::{RDFNode, SRDFBasic, SRDF};

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::ConstraintComponent;
use crate::helper::oxigraph::ask;
use crate::helper::term::Term;
use crate::runner::oxigraph::OxigraphStore;
use crate::validation_report::report::ValidationReport;

/// The condition specified by sh:class is that each value node is a SHACL
/// instance of a given type.
///
/// https://www.w3.org/TR/shacl/#ClassConstraintComponent
pub(crate) struct Class {
    class_rule: Option<RDFNode>,
}

impl Class {
    pub fn new(class_rule: RDFNode) -> Self {
        let class_rule = match class_rule {
            RDFNode::Iri(i) => Some(RDFNode::Iri(i)),
            RDFNode::BlankNode(_) => None,
            RDFNode::Literal(_) => None,
        };
        Class { class_rule }
    }
}

impl<S: SRDF + SRDFBasic> ConstraintComponent<S> for Class {
    fn evaluate(
        &self,
        _store: &S,
        _value_nodes: HashSet<Term>,
        _report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        Err(ConstraintError::NotImplemented)
    }
}

impl<'a> ConstraintComponent<OxigraphStore<'a>> for Class {
    fn evaluate(
        &self,
        store: &OxigraphStore<'a>,
        value_nodes: HashSet<Term>,
        report: &mut ValidationReport,
    ) -> Result<(), ConstraintError> {
        for node in &value_nodes {
            match &self.class_rule {
                Some(class_rule) => {
                    let query = formatdoc! {"
                            PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
                            PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>
                            ASK {{ {} rdf:type/rdfs:subClassOf* {} }}
                        ", node, class_rule,
                    };
                    if !ask(store, query)? {
                        <Class as ConstraintComponent<OxigraphStore<'a>>>::make_validation_result(
                            self,
                            Some(node),
                            report,
                        );
                    }
                }
                None => <Class as ConstraintComponent<OxigraphStore<'a>>>::make_validation_result(
                    self,
                    Some(node),
                    report,
                ),
            }
        }
        Ok(())
    }
}
