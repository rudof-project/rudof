use indoc::formatdoc;
use srdf::RDF_TYPE;
use srdf::{QuerySRDF, RDFNode, SRDFBasic, RDFS_SUBCLASS_OF, SRDF};

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::DefaultConstraintComponent;
use crate::constraints::SparqlConstraintComponent;
use crate::context::Context;
use crate::executor::DefaultExecutor;
use crate::executor::QueryExecutor;
use crate::executor::SHACLExecutor;
use crate::helper::srdf::get_objects_for;
use crate::shape::ValueNode;
use crate::validation_report::report::ValidationReport;

/// The condition specified by sh:class is that each value node is a SHACL
/// instance of a given type.
///
/// https://www.w3.org/TR/shacl/#ClassConstraintComponent
pub(crate) struct Class<S: SRDFBasic> {
    class_rule: S::Term,
}

impl<S: SRDFBasic> Class<S> {
    pub fn new(class_rule: RDFNode) -> Self {
        Class {
            class_rule: S::object_as_term(&class_rule),
        }
    }
}

impl<S: SRDF + 'static> DefaultConstraintComponent<S> for Class<S> {
    fn evaluate_default(
        &self,
        executor: &DefaultExecutor<S>,
        context: &Context,
        value_nodes: &ValueNode<S>,
        report: &mut ValidationReport<S>,
    ) -> Result<bool, ConstraintError> {
        let mut ans = true;
        for (focus_node, value_nodes) in value_nodes {
            for value_node in value_nodes {
                // if the node is a literal...
                if S::term_is_literal(value_node) {
                    report.make_validation_result(focus_node, context, Some(value_node));
                    ans = false;
                    continue;
                }
                // or a non-literal that is not a SHACL instance of the provided
                // class...
                let is_class_valid =
                    get_objects_for(executor.store(), value_node, &S::iri_s2iri(&RDF_TYPE))?
                        .iter()
                        .any(|ctype| {
                            ctype == &self.class_rule
                                || get_objects_for(
                                    executor.store(),
                                    ctype,
                                    &S::iri_s2iri(&RDFS_SUBCLASS_OF),
                                )
                                .unwrap_or_default()
                                .contains(&self.class_rule)
                        });
                // ... validation result
                if !is_class_valid {
                    report.make_validation_result(focus_node, context, Some(value_node));
                    ans = false;
                }
            }
        }
        Ok(ans)
    }
}

impl<S: QuerySRDF + 'static> SparqlConstraintComponent<S> for Class<S> {
    fn evaluate_sparql(
        &self,
        executor: &QueryExecutor<S>,
        context: &Context,
        value_nodes: &ValueNode<S>,
        report: &mut ValidationReport<S>,
    ) -> Result<bool, ConstraintError> {
        let mut ans = true;
        for (focus_node, value_nodes) in value_nodes {
            for value_node in value_nodes {
                let query = formatdoc! {"
                    PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
                    PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>
                    ASK {{ {} rdf:type/rdfs:subClassOf* {} }}
                ", value_node, self.class_rule,
                };
                let ask = match executor.store().query_ask(&query) {
                    Ok(ask) => ask,
                    Err(_) => return Err(ConstraintError::Query),
                };
                if !ask {
                    ans = false;
                    report.make_validation_result(focus_node, context, Some(value_node));
                }
            }
        }

        Ok(ans)
    }
}
