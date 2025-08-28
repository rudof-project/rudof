use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::helpers::constraint::validate_with;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodeIteration;
use crate::value_nodes::ValueNodes;
use shacl_ir::compiled::component::CompiledComponent;
use shacl_ir::compiled::component::LessThan;
use shacl_ir::compiled::shape::CompiledShape;
use srdf::NeighsRDF;
use srdf::QueryRDF;
use srdf::SHACLPath;
use std::fmt::Debug;
use tracing::debug;

impl<R: NeighsRDF + Debug + 'static> NativeValidator<R> for LessThan {
    fn validate_native(
        &self,
        component: &CompiledComponent,
        shape: &CompiledShape,
        _store: &R,
        value_nodes: &ValueNodes<R>,
        _source_shape: Option<&CompiledShape>,
        maybe_path: Option<SHACLPath>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        let datatype_check = |value_node: &R::Term| {
            debug!(
                "Less than check: value node: {value_node} with values of property: {}",
                self.iri()
            );
            true
        };

        let message = format!(
            "Less than constraint not satisfied. Expected less than: {}",
            self.iri()
        );
        validate_with(
            component,
            shape,
            value_nodes,
            ValueNodeIteration,
            datatype_check,
            &message,
            maybe_path,
        )
    }
}

impl<R: QueryRDF + Debug + 'static> SparqlValidator<R> for LessThan {
    fn validate_sparql(
        &self,
        _component: &CompiledComponent,
        _shape: &CompiledShape,
        _store: &R,
        _value_nodes: &ValueNodes<R>,
        _source_shape: Option<&CompiledShape>,
        _maybe_path: Option<SHACLPath>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        Err(ConstraintError::NotImplemented("LessThan".to_string()))
    }
}
