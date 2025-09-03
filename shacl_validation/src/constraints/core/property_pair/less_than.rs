use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;
use shacl_ir::compiled::component_ir::ComponentIR;
use shacl_ir::compiled::component_ir::LessThan;
use shacl_ir::compiled::shape::ShapeIR;
use srdf::NeighsRDF;
use srdf::Object;
use srdf::QueryRDF;
use srdf::Rdf;
use srdf::SHACLPath;
use srdf::Triple;
use std::fmt::Debug;

impl<R: NeighsRDF + Debug + 'static> NativeValidator<R> for LessThan {
    fn validate_native(
        &self,
        component: &ComponentIR,
        shape: &ShapeIR,
        store: &R,
        value_nodes: &ValueNodes<R>,
        _source_shape: Option<&ShapeIR>,
        maybe_path: Option<SHACLPath>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        let mut validation_results = Vec::new();
        let component = Object::iri(component.into());
        let severity = Object::iri(shape.severity().iri());

        for (focus_node, nodes) in value_nodes.iter() {
            let subject: R::Subject = <R as Rdf>::term_as_subject(focus_node).unwrap();
            match store.triples_with_subject_predicate(subject.clone(), self.iri().clone().into()) {
                Ok(triples_iter) => {
                    // Collect nodes to compare
                    for triple in triples_iter {
                        let value = triple.obj();
                        let node1 = <R as Rdf>::term_as_object(value).unwrap();
                        for value2 in nodes.iter() {
                            let node2 = <R as Rdf>::term_as_object(value2).unwrap();
                            let message = match node2.partial_cmp(&node1) {
                                None => {
                                    Some(format!("LessThan constraint violated: {node1} is not comparable to {node2}"))
                                }
                                Some(ord) if ord.is_ge() => {
                                    Some(format!(
                                        "LessThan constraint violated: {node1} is not less than {node2}"
                                    ))
                                }
                                _ => None
                            };
                            match message {
                                Some(msg) => {
                                    let validation_result = ValidationResult::new(
                                        shape.id().clone(),
                                        component.clone(),
                                        severity.clone(),
                                    )
                                    .with_message(msg.as_str())
                                    .with_path(maybe_path.clone());
                                    validation_results.push(validation_result);
                                }
                                None => {}
                            }
                        }
                    }
                }
                Err(e) => {
                    let message = format!(
                        "LessThan: Error trying to find triples for subject {} and predicate {}: {e}",
                        subject,
                        self.iri()
                    );
                    let validation_result = ValidationResult::new(
                        shape.id().clone(),
                        component.clone(),
                        severity.clone(),
                    )
                    .with_message(message.as_str())
                    .with_path(maybe_path.clone());
                    validation_results.push(validation_result);
                }
            };
        }
        Ok(validation_results)
        // TODO: We should change the logic of the validation because now we do the loop inside the check and
        // in this way, when there is a violation for a focus node, it returns that violation and so, it doesn't return other
        // violations
        /*let check = |focus: &R::Term, value_node: &R::Term| {
            let subject: R::Subject = <R as Rdf>::term_as_subject(focus).unwrap();
            let triples_to_compare = match store
                .triples_with_subject_predicate(subject.clone(), self.iri().clone().into())
            {
                Ok(iter) => iter,
                Err(e) => {
                    debug!(
                        "LessThan: Error trying to find triples for subject {} and predicate {}: {e}",
                        subject,
                        self.iri()
                    );
                    return true;
                }
            };
            // This loop should be refactored to collect all violations and return them...
            for triple in triples_to_compare {
                let value = triple.obj();
                let value1 = <R as Rdf>::term_as_object(value_node).unwrap();
                let value2 = <R as Rdf>::term_as_object(value).unwrap();
                debug!("Comparing {value1} less than {value2}?");
                match value1.partial_cmp(&value2) {
                    None => {
                        debug!("LessThan constraint violated: {value_node} is not comparable to {value}");
                        return true;
                    }
                    Some(ord) if ord.is_ge() => {
                        debug!(
                            "LessThan constraint violated: {value_node} is not less than {value}"
                        );
                        return true;
                    }
                    _ => {}
                }
            }
            false
        };

        // We should do the loop over all candidates here

        let message = format!("Less than failed. Property {}", self.iri());

        validate_with_focus(
            component,
            shape,
            value_nodes,
            ValueNodeIteration,
            check,
            &message,
            maybe_path,
        )*/
    }
}

impl<R: QueryRDF + Debug + 'static> SparqlValidator<R> for LessThan {
    fn validate_sparql(
        &self,
        _component: &ComponentIR,
        _shape: &ShapeIR,
        _store: &R,
        _value_nodes: &ValueNodes<R>,
        _source_shape: Option<&ShapeIR>,
        _maybe_path: Option<SHACLPath>,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        Err(ConstraintError::NotImplemented("LessThan".to_string()))
    }
}
