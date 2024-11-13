use std::fmt::Debug;

use indoc::formatdoc;
use shacl_ast::compiled::component::Class;
use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::QuerySRDF;
use srdf::RDFS_SUBCLASS_OF;
use srdf::RDF_TYPE;
use srdf::SRDF;

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::helpers::constraint::validate_native_with_strategy;
use crate::helpers::constraint::validate_sparql_ask;
use crate::helpers::srdf::get_objects_for;
use crate::store::Store;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodeIteration;
use crate::value_nodes::ValueNodes;
use crate::Subsetting;

impl<S: SRDF + 'static> NativeValidator<S> for Class<S> {
    fn validate_native(
        &self,
        component: &CompiledComponent<S>,
        shape: &CompiledShape<S>,
        store: &Store<S>,
        value_nodes: &ValueNodes<S>,
        subsetting: &Subsetting,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        let class = |value_node: &S::Term| {
            if S::term_is_literal(value_node) {
                return true;
            }

            let is_class_valid =
                get_objects_for(store.inner_store(), value_node, &S::iri_s2iri(&RDF_TYPE))
                    .unwrap_or_default()
                    .iter()
                    .any(|ctype| {
                        ctype == self.class_rule()
                            || get_objects_for(
                                store.inner_store(),
                                ctype,
                                &S::iri_s2iri(&RDFS_SUBCLASS_OF),
                            )
                            .unwrap_or_default()
                            .contains(self.class_rule())
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

impl<S: Sparql> SparqlValidator<S> for Class<S> {
    fn validate_sparql(
        &self,
        component: &CompiledComponent<S>,
        shape: &CompiledShape<S>,
        store: &Store<S>,
        value_nodes: &ValueNodes<S>,
        subsetting: &Subsetting,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        let class_value = self.class_rule().clone();

        let query = move |value_node: &S::Term| {
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
