use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::helpers::constraint::validate_ask_with;
use crate::helpers::constraint::validate_with;
use crate::helpers::srdf::get_objects_for;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodeIteration;
use crate::value_nodes::ValueNodes;
use indoc::formatdoc;
use shacl_ast::compiled::component::Class;
use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::shape::CompiledShape;
use srdf::Query;
use srdf::Sparql;
use srdf::Term;
use srdf::RDFS_SUBCLASS_OF;
use srdf::RDF_TYPE;
use std::fmt::Debug;

impl<S: Query + 'static> NativeValidator<S> for Class<S> {
    fn validate_native(
        &self,
        component: &CompiledComponent<S>,
        shape: &CompiledShape<S>,
        store: &S,
        value_nodes: &ValueNodes<S>,
        _source_shape: Option<&CompiledShape<S>>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        let class = |value_node: &S::Term| {
            if value_node.is_literal() {
                return true;
            }

            let is_class_valid = get_objects_for(store, value_node, &RDF_TYPE.clone().into())
                .unwrap_or_default()
                .iter()
                .any(|ctype| {
                    ctype == self.class_rule()
                        || get_objects_for(store, ctype, &RDFS_SUBCLASS_OF.clone().into())
                            .unwrap_or_default()
                            .contains(self.class_rule())
                });

            !is_class_valid
        };

        let message = format!(
            "Class constraint not satisfied for class {}",
            self.class_rule()
        );
        validate_with(
            component,
            shape,
            value_nodes,
            ValueNodeIteration,
            class,
            &message,
        )
    }
}

impl<S: Sparql + Debug + 'static> SparqlValidator<S> for Class<S> {
    fn validate_sparql(
        &self,
        component: &CompiledComponent<S>,
        shape: &CompiledShape<S>,
        store: &S,
        value_nodes: &ValueNodes<S>,
        _source_shape: Option<&CompiledShape<S>>,
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

        let message = format!(
            "Class constraint not satisfied for class {}",
            self.class_rule()
        );
        validate_ask_with(component, shape, store, value_nodes, query, &message)
    }
}
