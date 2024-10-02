use indoc::formatdoc;
use shacl_ast::compiled::component::Class;
use srdf::QuerySRDF;
use srdf::RDFS_SUBCLASS_OF;
use srdf::RDF_TYPE;
use srdf::SRDF;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::helper::srdf::get_objects_for;
use crate::validation_report::result::{ValidationResult, ValidationResults};
use crate::ValueNodes;

impl<S: SRDF + 'static> NativeValidator<S> for Class<S> {
    fn validate_native(
        &self,
        store: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<ValidationResults<S>, ConstraintError> {
        let results = value_nodes
            .iter_value_nodes()
            .flat_map(move |(focus_node, value_node)| {
                if S::term_is_literal(value_node) {
                    let result = ValidationResult::new(focus_node, Some(value_node));
                    Some(result)
                } else {
                    let objects = get_objects_for(store, value_node, &S::iri_s2iri(&RDF_TYPE))
                        .unwrap_or_default();
                    let is_class_valid = objects.iter().any(|ctype| {
                        ctype == self.class_rule()
                            || get_objects_for(store, ctype, &S::iri_s2iri(&RDFS_SUBCLASS_OF))
                                .unwrap_or_default()
                                .contains(self.class_rule())
                    });

                    if !is_class_valid {
                        Some(ValidationResult::new(focus_node, Some(value_node)))
                    } else {
                        None
                    }
                }
            })
            .collect::<Vec<_>>();

        Ok(ValidationResults::new(results.into_iter()))
    }
}

impl<S: QuerySRDF + 'static> SparqlValidator<S> for Class<S> {
    fn validate_sparql(
        &self,
        store: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<ValidationResults<S>, ConstraintError> {
        let results = value_nodes
            .iter_value_nodes()
            .filter_map(move |(focus_node, value_node)| {
                let query = formatdoc! {"
                    PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
                    PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>
                    ASK {{ {} rdf:type/rdfs:subClassOf* {} }}
                ", value_node, self.class_rule(),
                };

                let ask = match store.query_ask(&query) {
                    Ok(ask) => ask,
                    Err(_) => return None,
                };

                if !ask {
                    Some(ValidationResult::new(focus_node, Some(value_node)))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        Ok(ValidationResults::new(results.into_iter()))
    }
}
