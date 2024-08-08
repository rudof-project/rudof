use std::collections::HashSet;

use indoc::formatdoc;
use shacl_ast::Schema;
use srdf::RDF_TYPE;
use srdf::{QuerySRDF, RDFNode, SRDFBasic, RDFS_SUBCLASS_OF, SRDF};

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::DefaultConstraintComponent;
use crate::constraints::SparqlConstraintComponent;
use crate::helper::srdf::get_objects_for;
use crate::runner::sparql_runner::SparqlValidatorRunner;
use crate::runner::srdf_runner::DefaultValidatorRunner;
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
        store: &S,
        _: &Schema,
        _: &DefaultValidatorRunner,
        value_nodes: &HashSet<S::Term>,
        report: &mut ValidationReport<S>,
    ) -> Result<bool, ConstraintError> {
        let mut ans = true;
        for node in value_nodes {
            let is_class_valid = get_objects_for(store, node, &S::iri_s2iri(&RDF_TYPE))?
                .iter()
                .any(|ctype| {
                    ctype == &self.class_rule
                        || get_objects_for(store, ctype, &S::iri_s2iri(&RDFS_SUBCLASS_OF))
                            .unwrap_or_default()
                            .contains(&self.class_rule)
                });
            if !is_class_valid {
                report.make_validation_result(Some(node));
                ans = false;
            }
        }
        Ok(ans)
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
            let query = formatdoc! {"
                    PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
                    PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>
                    ASK {{ {} rdf:type/rdfs:subClassOf* {} }}
                ", node, self.class_rule,
            };
            let ask = match store.query_ask(&query) {
                Ok(ask) => ask,
                Err(_) => return Err(ConstraintError::Query),
            };
            if !ask {
                ans = false;
                report.make_validation_result(Some(node));
            }
        }

        Ok(ans)
    }
}
