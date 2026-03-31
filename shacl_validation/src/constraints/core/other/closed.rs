use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::constraints::Validator;
use crate::constraints::constraint_error::ConstraintError;
use crate::shacl_engine::Engine;
use crate::shacl_engine::sparql::SparqlEngine;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;
use iri_s::IriS;
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath, query::QueryRDF, term::{Object, Triple}};
use shacl::ir::components::Closed;
use shacl::ir::{IRComponent, IRSchema, IRShape};
use std::collections::HashSet;
use std::fmt::Debug;

impl<S: NeighsRDF + Debug> Validator<S> for Closed {
    fn validate(
        &self,
        component: &IRComponent,
        shape: &IRShape,
        store: &S,
        _engine: &mut dyn Engine<S>,
        value_nodes: &ValueNodes<S>,
        _source_shape: Option<&IRShape>,
        _maybe_path: Option<&SHACLPath>,
        _shapes_graph: &IRSchema,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        if !self.is_closed() {
            return Ok(Vec::new());
        }

        let allowed_properties: HashSet<IriS> = shape.allowed_properties();
        let component_obj = Object::iri(component.into());
        let severity = shape.severity();
        let mut results = Vec::new();

        for (focus_node, _) in value_nodes.iter() {
            let subject = match S::term_as_subject(focus_node) {
                Ok(subj) => subj,
                Err(_) => continue,
            };

            let used_properties: HashSet<IriS> = store
                .triples_with_subject(&subject)
                .map_err(|e| ConstraintError::InternalError { msg: e.to_string() })?
                .map(|t| t.pred().clone().into())
                .collect();

            let focus_obj =
                S::term_as_object(focus_node).map_err(ConstraintError::RDFError)?;

            for extra in used_properties.difference(&allowed_properties) {
                let vr = ValidationResult::new(
                    focus_obj.clone(),
                    component_obj.clone(),
                    severity.clone(),
                )
                .with_source(Some(shape.id().clone()))
                .with_path(Some(SHACLPath::iri(extra.clone())));
                results.push(vr);
            }
        }

        Ok(results)
    }
}

impl<S: NeighsRDF + Debug + 'static> NativeValidator<S> for Closed {
    fn validate_native(
        &self,
        component: &IRComponent,
        shape: &IRShape,
        store: &S,
        engine: &mut dyn Engine<S>,
        value_nodes: &ValueNodes<S>,
        source_shape: Option<&IRShape>,
        maybe_path: Option<&SHACLPath>,
        shapes_graph: &IRSchema,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        self.validate(
            component,
            shape,
            store,
            engine,
            value_nodes,
            source_shape,
            maybe_path,
            shapes_graph,
        )
    }
}

impl<S: QueryRDF + NeighsRDF + Debug + 'static> SparqlValidator<S> for Closed {
    fn validate_sparql(
        &self,
        component: &IRComponent,
        shape: &IRShape,
        store: &S,
        value_nodes: &ValueNodes<S>,
        source_shape: Option<&IRShape>,
        maybe_path: Option<&SHACLPath>,
        shapes_graph: &IRSchema,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        self.validate(
            component,
            shape,
            store,
            &mut SparqlEngine::new(),
            value_nodes,
            source_shape,
            maybe_path,
            shapes_graph,
        )
    }
}
