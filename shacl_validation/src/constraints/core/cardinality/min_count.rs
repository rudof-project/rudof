use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::constraints::Validator;
use crate::constraints::constraint_error::ConstraintError;
use crate::focus_nodes::FocusNodes;
use crate::helpers::constraint::validate_with;
use crate::iteration_strategy::FocusNodeIteration;
use crate::shacl_engine::Engine;
use crate::shacl_engine::sparql::SparqlEngine;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath, query::QueryRDF};
use shacl::ir::components::MinCount;
use shacl::ir::{IRComponent, IRSchema, IRShape};
use std::fmt::Debug;

impl<S: NeighsRDF + Debug> Validator<S> for MinCount {
    fn validate(
        &self,
        component: &IRComponent,
        shape: &IRShape,
        _: &S,
        _: &mut dyn Engine<S>,
        value_nodes: &ValueNodes<S>,
        _source_shape: Option<&IRShape>,
        maybe_path: Option<&SHACLPath>,
        _shapes_graph: &IRSchema,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        tracing::debug!("Validating minCount with shape {}", shape.id());
        if self.min_count() == 0 {
            // If min_count is 0, then it always passes
            return Ok(Default::default());
        }
        let min_count = |targets: &FocusNodes<S>| targets.len() < self.min_count();
        let message = format!("MinCount({}) not satisfied", self.min_count());
        validate_with(
            component,
            shape,
            value_nodes,
            FocusNodeIteration,
            min_count,
            message.as_str(),
            maybe_path,
        )
    }
}

impl<S: NeighsRDF + Debug + 'static> NativeValidator<S> for MinCount {
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
        tracing::debug!("Validate native minCount with shape: {}", shape.id());
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

impl<S: QueryRDF + NeighsRDF + Debug + 'static> SparqlValidator<S> for MinCount {
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
