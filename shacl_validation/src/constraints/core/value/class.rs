use indoc::formatdoc;
use srdf::RDF_TYPE;
use srdf::{QuerySRDF, RDFNode, SRDFBasic, RDFS_SUBCLASS_OF, SRDF};
use std::collections::HashSet;
use std::sync::Arc;

use crate::constraints::DefaultConstraintComponent;
use crate::constraints::SparqlConstraintComponent;
use crate::context::EvaluationContext;
use crate::context::ValidationContext;
use crate::helper::srdf::get_objects_for;
use crate::runner::default_runner::DefaultValidatorRunner;
use crate::runner::query_runner::QueryValidatorRunner;
use crate::validation_report::result::{LazyValidationIterator, ValidationResult};
use crate::value_nodes::ValueNodes;

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

impl<S: SRDF> DefaultConstraintComponent<S> for Class<S> {
    fn evaluate_default<'a>(
        &'a self,
        validation_context: &'a ValidationContext<'a, S>,
        evaluation_context: EvaluationContext<'a>,
        value_nodes: &'a ValueNodes<S>,
    ) -> LazyValidationIterator<'a, S> {
        let results = value_nodes
            .into_iter()
            .flat_map(move |(focus_node, value_node)| {
                if S::term_is_literal(&value_node) {
                    let result =
                        ValidationResult::new(focus_node, &evaluation_context, Some(value_node));
                    Some(result)
                } else {
                    let objects = match get_objects_for(
                        validation_context.store(),
                        &value_node,
                        &S::iri_s2iri(&RDF_TYPE),
                    ) {
                        Ok(objects) => objects,
                        Err(_) => HashSet::new(),
                    };

                    let is_class_valid = objects.iter().any(|ctype| {
                        ctype == &self.class_rule
                            || get_objects_for(
                                validation_context.store(),
                                ctype,
                                &S::iri_s2iri(&RDFS_SUBCLASS_OF),
                            )
                            .unwrap_or_default()
                            .contains(&self.class_rule)
                    });

                    if !is_class_valid {
                        Some(ValidationResult::new(
                            focus_node,
                            &evaluation_context,
                            Some(value_node),
                        ))
                    } else {
                        None
                    }
                }
            });

        LazyValidationIterator::new(results)
    }
}

impl<S: QuerySRDF> SparqlConstraintComponent<S> for Class<S> {
    fn evaluate_sparql<'a>(
        &'a self,
        validation_context: &'a ValidationContext<'a, S>,
        evaluation_context: EvaluationContext<'a>,
        value_nodes: &'a ValueNodes<S>,
    ) -> LazyValidationIterator<'a, S> {
        let results = value_nodes
            .into_iter()
            .filter_map(move |(focus_node, value_node)| {
                let query = formatdoc! {"
                    PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
                    PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>
                    ASK {{ {} rdf:type/rdfs:subClassOf* {} }}
                ", value_node, self.class_rule,
                };

                let ask = match validation_context.store().query_ask(&query) {
                    Ok(ask) => ask,
                    Err(_) => return None,
                };

                if !ask {
                    Some(ValidationResult::new(
                        focus_node,
                        &evaluation_context,
                        Some(value_node),
                    ))
                } else {
                    None
                }
            });

        LazyValidationIterator::new(results)
    }
}
