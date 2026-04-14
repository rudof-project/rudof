use std::cmp::Ordering;
use std::fmt::Debug;
use rudof_rdf::rdf_core::{NeighsRDF, Rdf, SHACLPath};
use rudof_rdf::rdf_core::query::QueryRDF;
use rudof_rdf::rdf_core::term::{Object, Triple};
use crate::ir::components::LessThan;
use crate::ir::{IRComponent, IRSchema, IRShape};
use crate::validation::constraints::{ConstraintError, NativeValidator, SparqlValidator};
use crate::validation::engine::Engine;
use crate::validation::report::ValidationResult;
use crate::validation::value_nodes::ValueNodes;

impl<S: NeighsRDF + Debug + 'static> NativeValidator<S> for LessThan {
    fn validate_native(&self, component: &IRComponent, shape: &IRShape, store: &S, engine: &mut dyn Engine<S>, value_nodes: &ValueNodes<S>, source_shape: Option<&IRShape>, maybe_path: Option<&SHACLPath>, shapes_graph: &IRSchema) -> Result<Vec<ValidationResult>, ConstraintError> {
        let mut validation_results = Vec::new();
        let component = Object::Iri(component.into());

        for (fnode, nodes) in value_nodes.iter() {
            let subject = S::term_as_subject(fnode)?;
            let iri: S::IRI = self.iri().clone().into();

            match store.triples_with_subject_predicate(&subject, &iri) {
                Ok(triples_iter) => {
                    // Collect nodes to compare
                    for triple in triples_iter {
                        let node1 = S::term_as_object(triple.obj())?;
                        for value in nodes.iter() {
                            let node2 = S::term_as_object(value)?;
                            let msg = match node2.partial_cmp(&node1) {
                                None => Some(format!("LessThan constraint violated: {node1} is not comparable to {node2}")),
                                Some(ord) if ord.is_ge() => Some(format!("LessThan constraint violated: {node1} is not less than {node2}")),
                                _ => None,
                            };

                            if let Some(msg) = msg {
                                let validation_result = ValidationResult::new(shape.id().clone(), component.clone(), shape.severity())
                                    .with_message(Some(msg))
                                    .with_path(maybe_path.cloned());
                                validation_results.push(validation_result);
                            }
                        }
                    }
                },
                Err(e) => {
                        let msg = format!("LessThan: Error trying to find triples for subject {subject} and predicate {}: {e}", self.iri());
                    let validation_result = ValidationResult::new(shape.id().clone(), component.clone(), shape.severity())
                        .with_path(maybe_path.cloned())
                        .with_message(Some(msg));
                    validation_results.push(validation_result);
                }
            }
        }

        Ok(validation_results)
    }
}

impl<S: QueryRDF + Debug + 'static> SparqlValidator<S> for LessThan {
    fn validate_sparql(&self, component: &IRComponent, shape: &IRShape, store: &S, value_nodes: &ValueNodes<S>, source_shape: Option<&IRShape>, maybe_path: Option<&SHACLPath>, shapes_graph: &IRSchema) -> Result<Vec<ValidationResult>, ConstraintError> {
        Err(ConstraintError::NotImplemented {
            err: "LessThan not implemented".to_string()
        })
    }
}