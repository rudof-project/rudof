use crate::error::ValidationError;
use crate::ir::components::{Expression, VALUE_VAR};
use crate::ir::{IRComponent, IRSchema, IRShape, node_expr_is_true};
use crate::types::MessageMap;
use crate::validator::constraints::Validator;
use crate::validator::engine::Engine;
use crate::validator::nodes::ValueNodes;
use crate::validator::report::{Evidence, ValidationOutcome, ValidationResult};
use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath};
use std::collections::HashMap;
use std::fmt::Debug;

impl<S: NeighsRDF + Debug> Validator<S> for Expression {
    fn validate(
        &self,
        component: &IRComponent,
        shape: &IRShape,
        store: &S,
        engine: &mut dyn Engine<S>,
        value_nodes: &ValueNodes<S>,
        _: Option<&IRShape>,
        maybe_path: Option<&SHACLPath>,
        shapes_graph: &IRSchema,
    ) -> Result<ValidationOutcome, ValidationError> {
        let mut outcome = ValidationOutcome::new();
        let component_obj = Object::iri(component.into());

        // This shape may be validated while an outer expression is being evaluated, whose step
        // count is put back after the ones counted here
        let outer_steps = engine.node_expr_state().steps;
        let result = self.validate_value_nodes(
            &mut outcome,
            &component_obj,
            shape,
            store,
            engine,
            value_nodes,
            maybe_path,
            shapes_graph,
        );
        engine.node_expr_state().steps = outer_steps;
        result?;

        Ok(outcome)
    }
}

impl Expression {
    #[allow(clippy::too_many_arguments)]
    fn validate_value_nodes<S: NeighsRDF + Debug>(
        &self,
        outcome: &mut ValidationOutcome,
        component_obj: &Object,
        shape: &IRShape,
        store: &S,
        engine: &mut dyn Engine<S>,
        value_nodes: &ValueNodes<S>,
        maybe_path: Option<&SHACLPath>,
        shapes_graph: &IRSchema,
    ) -> Result<(), ValidationError> {
        for (fnode, nodes) in value_nodes.iter() {
            let fnode_obj = S::term_as_object(fnode)?;
            for node in nodes.iter() {
                let node_obj = S::term_as_object(node)?;
                // The focus node of the shape is the focus node, and the value node is bound
                // to the scope variable `value`
                let scope = HashMap::from([(VALUE_VAR.to_string(), node.clone())]);

                engine.node_expr_state().steps = 0;
                let values = self.expr().evaluate(store, fnode, &scope, engine, shapes_graph)?;

                if node_expr_is_true::<S>(&values) {
                    let ev = Evidence::new(fnode_obj.clone(), component_obj.clone())
                        .with_path(maybe_path.cloned())
                        .with_value(Some(node_obj))
                        .with_source(Some(shape.id().clone()));
                    outcome.push_evidence(ev);
                } else {
                    let msg = format!(
                        "Shape {}: the expression {} does not return true for value {node} (sh:expression)",
                        shape.id(),
                        self.expr()
                    );
                    let vr = ValidationResult::new(fnode_obj.clone(), component_obj.clone(), shape.severity().clone())
                        .with_path(maybe_path.cloned())
                        .with_message(MessageMap::from(msg))
                        .with_value(Some(node_obj))
                        .with_source(Some(shape.id().clone()))
                        .with_source_constraint(Some(self.source().clone()));
                    outcome.push_violation(vr);
                }
            }
        }

        Ok(())
    }
}
