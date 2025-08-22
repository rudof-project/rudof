use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::constraints::Validator;
use crate::constraints::constraint_error::ConstraintError;
use crate::engine::Engine;
use crate::engine::native::NativeEngine;
use crate::engine::sparql::SparqlEngine;
use crate::focus_nodes::FocusNodes;
use crate::helpers::constraint::validate_with;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::FocusNodeIteration;
use crate::value_nodes::ValueNodes;

use shacl_ir::compiled::component::CompiledComponent;
use shacl_ir::compiled::component::MinCount;
use shacl_ir::compiled::shape::CompiledShape;
use srdf::NeighsRDF;
use srdf::QueryRDF;
use srdf::Rdf;
use srdf::SHACLPath;
use std::fmt::Debug;

impl<S: Rdf + Debug> Validator<S> for MinCount {
    fn validate(
        &self,
        component: &CompiledComponent,
        shape: &CompiledShape,
        _: &S,
        _: impl Engine<S>,
        value_nodes: &ValueNodes<S>,
        _source_shape: Option<&CompiledShape>,
        maybe_path: Option<SHACLPath>,
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
        component: &CompiledComponent,
        shape: &CompiledShape,
        store: &S,
        value_nodes: &ValueNodes<S>,
        source_shape: Option<&CompiledShape>,
        maybe_path: Option<SHACLPath>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        tracing::debug!("Validate native minCount with shape: {}", shape.id());
        self.validate(
            component,
            shape,
            store,
            NativeEngine,
            value_nodes,
            source_shape,
            maybe_path,
        )
    }
}

impl<S: QueryRDF + Debug + 'static> SparqlValidator<S> for MinCount {
    fn validate_sparql(
        &self,
        component: &CompiledComponent,
        shape: &CompiledShape,
        store: &S,
        value_nodes: &ValueNodes<S>,
        source_shape: Option<&CompiledShape>,
        maybe_path: Option<SHACLPath>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        self.validate(
            component,
            shape,
            store,
            SparqlEngine,
            value_nodes,
            source_shape,
            maybe_path,
        )
    }
}
