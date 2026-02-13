use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::constraints::constraint_error::ConstraintError;
use crate::helpers::constraint::validate_ask_with;
use crate::helpers::constraint::validate_with;
use crate::iteration_strategy::ValueNodeIteration;
use crate::shacl_engine::engine;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;
use indoc::formatdoc;
use shacl_ir::compiled::component_ir::Class;
use shacl_ir::compiled::component_ir::ComponentIR;
use shacl_ir::compiled::shape::ShapeIR;
use shacl_ir::schema_ir::SchemaIR;
use rdf::rdf_core::{NeighsRDF, SHACLPath, query::QueryRDF, term::Term, vocab::{rdf_type, rdfs_subclass_of}};
use std::fmt::Debug;

impl<S: NeighsRDF + 'static> NativeValidator<S> for Class {
    fn validate_native(
        &self,
        component: &ComponentIR,
        shape: &ShapeIR,
        store: &S,
        _engine: &mut dyn engine::Engine<S>,
        value_nodes: &ValueNodes<S>,
        _source_shape: Option<&ShapeIR>,
        maybe_path: Option<SHACLPath>,
        _shapes_graph: &SchemaIR,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        let class = |value_node: &S::Term| {
            if value_node.is_literal() {
                return true;
            }
            let class_term = &S::object_as_term(self.class_rule());

            let is_class_valid = store
                .objects_for(value_node, &rdf_type().clone().into())
                .unwrap_or_default()
                .iter()
                .any(|ctype| {
                    ctype == class_term
                        || store
                            .objects_for(ctype, &rdfs_subclass_of().clone().into())
                            .unwrap_or_default()
                            .contains(class_term)
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
            maybe_path,
        )
    }
}

impl<S: QueryRDF + Debug + 'static> SparqlValidator<S> for Class {
    fn validate_sparql(
        &self,
        component: &ComponentIR,
        shape: &ShapeIR,
        store: &S,
        value_nodes: &ValueNodes<S>,
        _source_shape: Option<&ShapeIR>,
        maybe_path: Option<SHACLPath>,
        _shapes_graph: &SchemaIR,
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
        validate_ask_with(
            component,
            shape,
            store,
            value_nodes,
            query,
            &message,
            maybe_path,
        )
    }
}
