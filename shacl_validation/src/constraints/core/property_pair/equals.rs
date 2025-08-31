use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::helpers::constraint::validate_with_focus;
use crate::iteration_strategy::ValueNodeIteration;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;
use shacl_ir::compiled::component::CompiledComponent;
use shacl_ir::compiled::component::Equals;
use shacl_ir::compiled::shape::CompiledShape;
use srdf::NeighsRDF;
use srdf::QueryRDF;
use srdf::Rdf;
use srdf::SHACLPath;
use srdf::Triple;
use std::fmt::Debug;
use tracing::debug;

impl<R: NeighsRDF + Debug + 'static> NativeValidator<R> for Equals {
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
                        "Equals: Error trying to find triples for subject {} and predicate {}: {e}",
                        subject,
                        self.iri()
                    );
                    return true;
                }
            };
            let mut triples_to_compare = triples_to_compare.peekable();
            if triples_to_compare.peek().is_none() {
                debug!(
                    "Equals: No triples found for subject {} and predicate {}",
                    subject,
                    self.iri()
                );
                return true;
            }
            for triple in triples_to_compare {
                let value = triple.obj();
                let value1 = <R as Rdf>::term_as_object(value_node).unwrap();
                let value2 = <R as Rdf>::term_as_object(value).unwrap();
                debug!("Comparing equals\nValue1:{value1}\nValue2:{value2}\nFocus:{focus}");
                if value1 != value2 {
                    debug!("Equals constraint violated: {value1} is not equal to {value2}");
                    return true;
                }
            }
            false
        };
        let message = format!("Equals failed. Property {}", self.iri());

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

impl<S: QueryRDF + NeighsRDF + Debug + 'static> SparqlValidator<S> for Equals {
    fn validate_sparql(
        &self,
        _component: &CompiledComponent,
        _shape: &CompiledShape,
        _store: &S,
        _value_nodes: &ValueNodes<S>,
        _source_shape: Option<&CompiledShape>,
        _maybe_path: Option<SHACLPath>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        Err(ConstraintError::NotImplemented("Equals".to_string()))
    }
}
