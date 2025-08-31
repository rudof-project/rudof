use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::helpers::constraint::validate_with_focus;
use crate::iteration_strategy::ValueNodeIteration;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;
use shacl_ir::compiled::component::CompiledComponent;
use shacl_ir::compiled::component::LessThanOrEquals;
use shacl_ir::compiled::shape::CompiledShape;
use srdf::NeighsRDF;
use srdf::QueryRDF;
use srdf::Rdf;
use srdf::SHACLPath;
use srdf::Triple;
use std::fmt::Debug;
use tracing::debug;

impl<R: NeighsRDF + Debug + 'static> NativeValidator<R> for LessThanOrEquals {
    fn validate_native(
        &self,
        component: &CompiledComponent,
        shape: &CompiledShape,
        store: &R,
        value_nodes: &ValueNodes<R>,
        _source_shape: Option<&CompiledShape>,
        maybe_path: Option<SHACLPath>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        let check = |focus: &R::Term, value_node: &R::Term| {
            let subject: R::Subject = <R as Rdf>::term_as_subject(focus).unwrap();
            let triples_to_compare = match store
                .triples_with_subject_predicate(subject.clone(), self.iri().clone().into())
            {
                Ok(iter) => iter,
                Err(e) => {
                    debug!(
                        "LessThanOrEquals: Error trying to find triples for subject {} and predicate {}: {e}",
                        subject,
                        self.iri()
                    );
                    return true;
                }
            };
            for triple in triples_to_compare {
                let value = triple.obj();
                let value1 = <R as Rdf>::term_as_object(value_node).unwrap();
                let value2 = <R as Rdf>::term_as_object(value).unwrap();
                debug!("Comparing {value1} less than or equals {value2}");
                match value1.partial_cmp(&value2) {
                    None => {
                        debug!("LessThanOrEquals constraint violated: {value_node} is not comparable to {value}");
                        return true;
                    }
                    Some(ord) if ord.is_gt() => {
                        debug!(
                            "LessThanOrEquals constraint violated: {value_node} is not less than or equals {value}"
                        );
                        return true;
                    }
                    _ => {}
                }
            }
            false
        };
        let message = format!("Less than or equals failed. Property {}", self.iri());

        validate_with_focus(
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

impl<S: QueryRDF + Debug + 'static> SparqlValidator<S> for LessThanOrEquals {
    fn validate_sparql(
        &self,
        _component: &CompiledComponent,
        _shape: &CompiledShape,
        _store: &S,
        _value_nodes: &ValueNodes<S>,
        _source_shape: Option<&CompiledShape>,
        _maybe_path: Option<SHACLPath>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        Err(ConstraintError::NotImplemented(
            "LessThanOrEquals".to_string(),
        ))
    }
}
