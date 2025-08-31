use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::helpers::constraint::validate_with;
use crate::helpers::constraint::validate_with_focus;
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
use srdf::Triple;
use std::fmt::Debug;
use tracing::debug;

impl<R: NeighsRDF + Debug + 'static> NativeValidator<R> for LessThan {
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
        debug!("LessThan.validate_native with path: {:?}", mp);
        let check = |focus: &R::Term, value_node: &R::Term| {
            debug!(
                "lessThan check: focus: {focus}, value node: {value_node} with values of property: {}. Path: {:?}",
                self.iri(),
                mp
            );
            let subject: R::Subject = <R as Rdf>::term_as_subject(focus).unwrap();
            let triples_to_compare = match store
                .triples_with_subject_predicate(subject.clone(), self.iri().clone().into())
            {
                Ok(iter) => iter,
                Err(e) => {
                    debug!(
                        "Error trying to find triples for subject {} and predicate {}: {e}",
                        subject,
                        self.iri()
                    );
                    return true;
                }
            };
            for triple in triples_to_compare {
                let value = triple.obj();
                let value1 = <R as Rdf>::term_as_object(&value_node).unwrap();
                let value2 = <R as Rdf>::term_as_object(&value).unwrap();
                debug!("Comparing {value1} < {value2}");
                if value1 >= value2 {
                    debug!("LessThan constraint violated: {value_node} is not less than {value}");
                    return true;
                }
            }
            false
        };
        let message = format!("Less than failed. Property {}", self.iri());

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
