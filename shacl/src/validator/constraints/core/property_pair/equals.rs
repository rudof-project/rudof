use crate::error::ValidationError;
use crate::ir::components::Equals;
use crate::ir::{IRComponent, IRSchema, IRShape};
use crate::validator::constraints::{BasicSparqlValidator, NativeValidator};
use crate::validator::engine::Engine;
use crate::validator::nodes::ValueNodes;
use crate::validator::report::ValidationResult;
use rudof_rdf::rdf_core::query::QueryRDF;
use rudof_rdf::rdf_core::term::{Object, Triple};
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath};
use std::collections::HashSet;
use std::fmt::Debug;

impl<S: NeighsRDF + Debug + 'static> NativeValidator<S> for Equals {
    fn validate_native(
        &self,
        component: &IRComponent,
        shape: &IRShape,
        store: &S,
        _: &mut dyn Engine<S>,
        value_nodes: &ValueNodes<S>,
        _: Option<&IRShape>,
        maybe_path: Option<&SHACLPath>,
        _: &IRSchema,
    ) -> Result<Vec<ValidationResult>, ValidationError> {
        let component_obj = Object::iri(component.into());
        let mut results = Vec::new();

        for (fnode, nodes) in value_nodes.iter() {
            let subject = match S::term_as_subject(fnode) {
                Ok(s) => s,
                Err(_) => continue,
            };

            let iri: S::IRI = self.iri().clone().into();

            let prop_values = store
                .triples_with_subject_predicate(&subject, &iri)
                .map_err(ValidationError::new_graph_error::<S>)?
                .map(|t| t.obj().clone())
                .collect::<HashSet<_>>();

            let nodes_set = nodes.iter().collect::<HashSet<_>>();

            let fnode_obj = S::term_as_object(fnode)?;

            for pv in &prop_values {
                if !nodes_set.contains(pv) {
                    let value = S::term_as_object(pv).ok();
                    let vr = ValidationResult::new(fnode_obj.clone(), component_obj.clone(), shape.severity().clone())
                        .with_source(Some(shape.id().clone()))
                        .with_path(maybe_path.cloned())
                        .with_value(value);
                    results.push(vr);
                }
            }

            for vn in nodes.iter() {
                if !prop_values.contains(vn) {
                    let value = S::term_as_object(vn).ok();
                    let vr = ValidationResult::new(fnode_obj.clone(), component_obj.clone(), shape.severity().clone())
                        .with_source(Some(shape.id().clone()))
                        .with_path(maybe_path.cloned())
                        .with_value(value);
                    results.push(vr);
                }
            }
        }

        Ok(results)
    }
}

#[cfg(feature = "sparql")]
impl<S: QueryRDF + NeighsRDF + Debug + 'static> BasicSparqlValidator<S> for Equals {
    fn validate_sparql(
        &self,
        _: &IRComponent,
        _: &IRShape,
        _: &S,
        _: &ValueNodes<S>,
        _: Option<&IRShape>,
        _: Option<&SHACLPath>,
        _: &IRSchema,
    ) -> Result<Vec<ValidationResult>, ValidationError> {
        unimplemented!()
    }
}
