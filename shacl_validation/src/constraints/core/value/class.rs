use indoc::formatdoc;
use shacl_ast::compiled::component::Class;
use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::matcher::Any;
use srdf::Query;
use srdf::Sparql;
use srdf::Triple;
use srdf::RDFS_SUBCLASS_OF;
use srdf::RDF_TYPE;

use crate::constraints::SparqlValidator;
use crate::constraints::Validator;
use crate::engine::Engine;
use crate::helpers::constraint::validate_ask_with;
use crate::helpers::constraint::validate_with;
use crate::validate_error::ValidateError;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodeIteration;
use crate::value_nodes::ValueNodes;

impl<Q: Query, E: Engine<Q>> Validator<Q, E> for Class<Q> {
    fn validate(
        &self,
        component: &CompiledComponent<Q>,
        shape: &CompiledShape<Q>,
        store: &Q,
        value_nodes: &ValueNodes<Q>,
    ) -> Result<Vec<ValidationResult>, ValidateError> {
        let class = |value_node: &Q::Term| {
            let subject: Q::Subject = match value_node.clone().try_into() {
                Ok(subject) => subject,
                Err(_) => return Ok(true),
            };

            let is_instance_of = store
                .triples_matching(subject, RDF_TYPE.clone(), Any)
                .unwrap() // TODO: check this unwrap
                .map(Triple::into_object)
                .any(|ctype| {
                    let subject: Q::Subject = match ctype.clone().try_into() {
                        Ok(subject) => subject,
                        Err(_) => return false, // TODO: return an error here
                    };
                    &ctype == self.class_rule()
                        || store
                            .triples_matching(subject, RDFS_SUBCLASS_OF.clone(), Any)
                            .unwrap() // TODO: check this unwrap
                            .any(|triple| triple.obj() == self.class_rule())
                });

            Ok(!is_instance_of)
        };

        validate_with(component, shape, value_nodes, ValueNodeIteration, class)
    }
}

impl<S: Sparql + Query> SparqlValidator<S> for Class<S> {
    fn validate_sparql(
        &self,
        component: &CompiledComponent<S>,
        shape: &CompiledShape<S>,
        store: &S,
        value_nodes: &ValueNodes<S>,
    ) -> Result<Vec<ValidationResult>, ValidateError> {
        let class_value = self.class_rule().clone();

        let query = move |value_node: &S::Term| {
            formatdoc! {"
            PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
            PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>
            ASK {{ {} rdf:type/rdfs:subClassOf* {} }}
        ", value_node, class_value,
            }
        };

        validate_ask_with(component, shape, store, value_nodes, query)
    }
}
