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
use srdf::subject;
use srdf::NeighsRDF;
use srdf::QueryRDF;
use srdf::Rdf;
use srdf::SHACLPath;
use std::fmt::Debug;
use tracing::debug;

impl<R: NeighsRDF + Debug + 'static> NativeValidator<R> for LessThan
where
    <R as Rdf>::Err: Debug,
{
    fn validate_native(
        &self,
        component: &CompiledComponent,
        shape: &CompiledShape,
        store: &R,
        value_nodes: &ValueNodes<R>,
        _source_shape: Option<&CompiledShape>,
        maybe_path: Option<SHACLPath>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        let mp = maybe_path.clone();
        let check = |value_node: &R::Term| {
            debug!(
                "lessThan check: value node: {value_node} with values of property: {}. Path: {:?}",
                self.iri(),
                mp
            );
            let subject: R::Subject = <R as Rdf>::term_as_subject(value_node).unwrap();
            let triples_to_compare = store
                .triples_with_subject_predicate(subject, self.iri().clone().into())
                .unwrap();
            debug!(
                "Triples to compare: {:?}",
                triples_to_compare
                    .map(|n| format!("{:?}", n))
                    .collect::<Vec<_>>()
            );
            true
        };
        let message = format!("Less than failed. Property {}", self.iri());

        validate_with(
            component,
            shape,
            value_nodes,
            ValueNodeIteration,
            check,
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
