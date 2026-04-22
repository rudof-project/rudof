use crate::ir::components::Closed;
use crate::ir::{IRComponent, IRSchema, IRShape};
use crate::validator::constraints::{ConstraintError, Validator};
use crate::validator::engine::Engine;
use crate::validator::nodes::ValueNodes;
use crate::validator::report::ValidationResult;
use rudof_rdf::rdf_core::term::{Object, Triple};
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath};
use std::fmt::Debug;

impl<S: NeighsRDF + Debug> Validator<S> for Closed {
    fn validate(
        &self,
        component: &IRComponent,
        shape: &IRShape,
        store: &S,
        _: &mut dyn Engine<S>,
        value_nodes: &ValueNodes<S>,
        _: Option<&IRShape>,
        _: Option<&SHACLPath>,
        _: &IRSchema,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        if !self.is_closed() {
            return Ok(Vec::new());
        }

        let allowed_props = shape.allowed_properties();
        let component_obj = Object::iri(component.into());
        let mut results = Vec::new();

        for (fnode, _) in value_nodes.iter() {
            let subject = match S::term_as_subject(fnode) {
                Ok(subj) => subj,
                Err(_) => continue,
            };

            let triples = store
                .triples_with_subject(&subject)
                .map_err(|e| ConstraintError::Internal { err: e.to_string() })?;

            let focus_obj = S::term_as_object(fnode)?;

            for triple in triples {
                let (_, pred, obj) = triple.into_components();
                let pred_iri = pred.into();
                if !allowed_props.contains(&pred_iri) {
                    let value = S::term_as_object(&obj).ok();
                    let vr = ValidationResult::new(focus_obj.clone(), component_obj.clone(), shape.severity())
                        .with_source(Some(shape.id().clone()))
                        .with_path(Some(SHACLPath::iri(pred_iri)))
                        .with_value(value);
                    results.push(vr);
                }
            }
        }

        Ok(results)
    }
}
