use crate::ast::error::ASTError;
use crate::error::{IRError, ValidationError};
use crate::ir::components::NodeByExpression;
use crate::ir::{IRComponent, IRSchema, IRShape};
use crate::types::MessageMap;
use crate::validator::constraints::Validator;
use crate::validator::engine::{Engine, Validate};
use crate::validator::nodes::{FocusNodes, ValueNodes};
use crate::validator::report::{Evidence, ValidationOutcome, ValidationResult};
use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath};
use std::collections::HashMap;
use std::fmt::Debug;

impl<S: NeighsRDF + Debug> Validator<S> for NodeByExpression {
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

impl NodeByExpression {
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
                // Each value node is the focus node of its own evaluation, with an empty scope
                let scope = HashMap::new();

                engine.node_expr_state().steps = 0;
                let shapes = self.expr().evaluate(store, node, &scope, engine, shapes_graph)?;

                for produced in shapes {
                    let produced_obj = S::term_as_object(&produced)?;
                    let idx = *shapes_graph
                        .get_idx(&produced_obj)
                        .ok_or_else(|| IRError::from(ASTError::from(produced_obj.clone())))?;
                    let node_shape = shapes_graph.get_shape_from_idx_e(&idx)?;
                    let focus_nodes = FocusNodes::single(node.clone());

                    let inner_outcome = if engine.has_validated(&node_obj, idx) {
                        engine.get_cached_outcome(&node_obj, idx).unwrap_or_default()
                    } else {
                        node_shape.validate(store, engine, Some(&focus_nodes), Some(shape), shapes_graph)?
                    };

                    if inner_outcome.conforms() {
                        let ev = Evidence::new(fnode_obj.clone(), component_obj.clone())
                            .with_path(maybe_path.cloned())
                            .with_value(Some(node_obj.clone()))
                            .with_source(Some(shape.id().clone()));
                        outcome.push_evidence(ev);
                    } else {
                        let msg = format!(
                            "Shape {}: value {node} does not conform to node shape {produced_obj} (sh:nodeByExpression)",
                            shape.id()
                        );
                        let vr =
                            ValidationResult::new(fnode_obj.clone(), component_obj.clone(), shape.severity().clone())
                                .with_path(maybe_path.cloned())
                                .with_message(MessageMap::from(msg))
                                .with_value(Some(node_obj.clone()))
                                .with_source(Some(shape.id().clone()))
                                .with_source_constraint(Some(produced_obj))
                                .with_details(
                                    inner_outcome
                                        .violations()
                                        .iter()
                                        .cloned()
                                        .map(ValidationResult::without_details)
                                        .collect(),
                                );
                        outcome.push_violation(vr);
                    }
                }
            }
        }

        Ok(())
    }
}
