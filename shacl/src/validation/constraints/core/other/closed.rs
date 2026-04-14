use std::collections::HashSet;
use std::fmt::Debug;
use rudof_rdf::rdf_core::{NeighsRDF, RDFError, Rdf, SHACLPath};
use rudof_rdf::rdf_core::query::QueryRDF;
use rudof_rdf::rdf_core::term::{Object, Triple};
use crate::ir::components::Closed;
use crate::ir::{IRComponent, IRSchema, IRShape};
use crate::validation::constraints::{ConstraintError, NativeValidator, SparqlValidator, Validator};
use crate::validation::engine::{Engine, SparqlEngine};
use crate::validation::report::ValidationResult;
use crate::validation::utils::validate_with_focus;
use crate::validation::value_nodes::ValueNodes;

impl<S: NeighsRDF + Debug> Validator<S> for Closed {
    fn validate(&self, component: &IRComponent, shape: &IRShape, store: &S, engine: &mut dyn Engine<S>, value_nodes: &ValueNodes<S>, source_shape: Option<&IRShape>, maybe_path: Option<&SHACLPath>, shapes_graph: &IRSchema) -> Result<Vec<ValidationResult>, ConstraintError> {
        if !self.is_closed() { return Ok(Vec::new()); }

        let allowed_props = shape.allowed_properties();
        let component_obj = Object::iri(component.into());
        let mut results = Vec::new();

        for (fnode, _) in value_nodes.iter() {
            let subject = match S::term_as_subject(fnode) {
                Ok(subj) => subj,
                Err(_) => continue,
            };

            let used_props = store
                .triples_with_subject(&subject)
                .map_err(|e| ConstraintError::Internal { err: e.to_string() })?
                .map(|t| t.pred().clone().into())
                .collect::<HashSet<_>>();

            let focus_obj = S::term_as_object(fnode).map_err(ConstraintError::RDF)?;

            for extra in used_props.difference(&allowed_props) {
                let vr = ValidationResult::new(
                    focus_obj.clone(),
                    component_obj.clone(),
                    shape.severity(),
                )
                    .with_source(Some(shape.id().clone()))
                    .with_path(Some(SHACLPath::iri(extra.clone())));
                results.push(vr);
            }
        }

        Ok(results)
    }
}
