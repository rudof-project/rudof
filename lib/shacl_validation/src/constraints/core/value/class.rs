use indoc::formatdoc;
use shacl_ast::compiled::component::Class;
use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::model::matcher::Matcher;
use srdf::model::rdf::Object;
use srdf::model::rdf::Rdf;
use srdf::model::sparql::Sparql;
use srdf::model::Term;
use srdf::model::Triple;
use srdf::RDFS_SUBCLASS_OF;
use srdf::RDF_TYPE;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::engine::Engine;
use crate::helpers::constraint::validate_native_with_strategy;
use crate::helpers::constraint::validate_sparql_ask;
use crate::store::Store;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodeIteration;
use crate::value_nodes::ValueNodes;
use crate::Subsetting;

impl<R: Rdf, E: Engine<R>> NativeValidator<R, E> for Class<R> {
    fn validate_native(
        &self,
        component: &CompiledComponent<R>,
        shape: &CompiledShape<R>,
        store: &Store<R>,
        engine: E,
        value_nodes: &ValueNodes<R>,
        subsetting: &Subsetting,
    ) -> Result<Vec<ValidationResult<R>>, ConstraintError> {
        let class = |value_node: &Object<R>| {
            if value_node.is_literal() {
                return true;
            }

            let is_class_valid = store
                .inner_store()
                .triples_matching(value_node.clone(), RDF_TYPE.into(), Matcher::Any)?
                .map(Triple::into_object)
                .any(|ctype| {
                    ctype == self.class_rule()
                        || store
                            .inner_store()
                            .triples_matching(ctype, RDFS_SUBCLASS_OF.into(), Matcher::Any)
                            .unwrap()
                            .map(Triple::into_object)
                            .find(|object| object == self.class_rule())
                            .is_some()
                });

            !is_class_valid
        };

        validate_native_with_strategy(
            component,
            shape,
            value_nodes,
            ValueNodeIteration,
            class,
            subsetting,
        )
    }
}

impl<S: Rdf + Sparql> SparqlValidator<S> for Class<S> {
    fn validate_sparql(
        &self,
        component: &CompiledComponent<S>,
        shape: &CompiledShape<S>,
        store: &Store<S>,
        value_nodes: &ValueNodes<S>,
        subsetting: &Subsetting,
    ) -> Result<Vec<ValidationResult<S>>, ConstraintError> {
        let class_value = self.class_rule().clone();

        let query = move |value_node: &Object<S>| {
            formatdoc! {"
                PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
                PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>
                ASK {{ {} rdf:type/rdfs:subClassOf* {} }}
            ", value_node, class_value,
            }
        };

        validate_sparql_ask(component, shape, store, value_nodes, query, subsetting)
    }
}
