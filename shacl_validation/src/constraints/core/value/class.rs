use std::collections::HashSet;

use indoc::formatdoc;
use shacl_ast::Schema;
use srdf::{QuerySRDF, RDFNode, SRDFBasic, SRDF};

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::DefaultConstraintComponent;
use crate::constraints::SparqlConstraintComponent;
use crate::runner::sparql_runner::SparqlValidatorRunner;
use crate::runner::srdf_runner::DefaultValidatorRunner;
use crate::validation_report::report::ValidationReport;

/// The condition specified by sh:class is that each value node is a SHACL
/// instance of a given type.
///
/// https://www.w3.org/TR/shacl/#ClassConstraintComponent
pub(crate) struct Class<S: SRDFBasic> {
    class_rule: Option<S::IRI>,
}

impl<S: SRDFBasic> Class<S> {
    pub fn new(class_rule: RDFNode) -> Self {
        Class {
            class_rule: S::term_as_iri(&S::object_as_term(&class_rule)),
        }
    }
}

impl<S: SRDF + 'static> DefaultConstraintComponent<S> for Class<S> {
    fn evaluate_default(
        &self,
        _store: &S,
        _schema: &Schema,
        _: &DefaultValidatorRunner,
        _value_nodes: &HashSet<S::Term>,
        _report: &mut ValidationReport<S>,
    ) -> Result<bool, ConstraintError> {
        Err(ConstraintError::NotImplemented)
    }
}

impl<S: QuerySRDF + 'static> SparqlConstraintComponent<S> for Class<S> {
    fn evaluate_sparql(
        &self,
        store: &S,
        _: &Schema,
        _: &SparqlValidatorRunner,
        value_nodes: &HashSet<S::Term>,
        report: &mut ValidationReport<S>,
    ) -> Result<bool, ConstraintError> {
        let mut ans = true;
        for node in value_nodes {
            if let Some(class_rule) = &self.class_rule {
                let query = formatdoc! {"
                    PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
                    PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>
                    ASK {{ {} rdf:type/rdfs:subClassOf* {} }}
                ", node, class_rule,
                };
                let ask = match store.query_ask(&query) {
                    Ok(ask) => ask,
                    Err(_) => return Err(ConstraintError::Query),
                };
                if !ask {
                    ans = false;
                    report.make_validation_result(Some(node));
                }
            } else {
                ans = false;
                report.make_validation_result(Some(node))
            }
        }
        Ok(ans)
    }
}
