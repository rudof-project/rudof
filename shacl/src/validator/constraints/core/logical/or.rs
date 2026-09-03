use crate::error::ValidationError;
use crate::ir::components::Or;
use crate::ir::{IRComponent, IRSchema, IRShape};
use crate::types::MessageMap;
use crate::validator::constraints::Validator;
use crate::validator::engine::{Engine, Validate};
use crate::validator::nodes::FocusNodes;
use crate::validator::nodes::ValueNodes;
use crate::validator::report::{Evidence, ValidationOutcome, ValidationResult};
use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath};
use std::fmt::Debug;

impl<S: NeighsRDF + Debug> Validator<S> for Or {
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
        let component = Object::iri(component.into());

        for (fnode, nodes) in value_nodes.iter() {
            let fnode_obj = S::term_as_object(fnode)?;
            for node in nodes.iter() {
                let focus_nodes = FocusNodes::single(node.clone());
                let mut conforms = false;
                for idx in self.shapes().iter() {
                    let or_shape = shapes_graph.get_shape_from_idx_e(idx)?;
                    let inner_results = or_shape.validate(store, engine, Some(&focus_nodes), Some(shape), shapes_graph);
                    if inner_results?.conforms() {
                        conforms = true;
                        break;
                    }
                }
                let node_obj = S::term_as_object(node).ok();
                if conforms {
                    let ev = Evidence::new(fnode_obj.clone(), component.clone())
                        .with_path(maybe_path.cloned())
                        .with_value(node_obj)
                        .with_source(Some(shape.id().clone()));
                    outcome.push_evidence(ev);
                } else {
                    let msg = "OR not satisfied".to_string();
                    let vr = ValidationResult::new(fnode_obj.clone(), component.clone(), shape.severity().clone())
                        .with_message(MessageMap::from(msg))
                        .with_path(maybe_path.cloned())
                        .with_value(node_obj)
                        .with_source(Some(shape.id().clone()));
                    outcome.push_violation(vr);
                }
            }
        }

        Ok(outcome)
    }
}
