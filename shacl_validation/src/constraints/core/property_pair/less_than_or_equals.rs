use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::constraints::constraint_error::ConstraintError;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;
use shacl_ir::compiled::component_ir::ComponentIR;
use shacl_ir::compiled::component_ir::LessThanOrEquals;
use shacl_ir::compiled::shape::ShapeIR;
use shacl_ir::schema_ir::SchemaIR;
use srdf::NeighsRDF;
use srdf::Object;
use srdf::QueryRDF;
use srdf::Rdf;
use srdf::SHACLPath;
use srdf::Triple;
use std::fmt::Debug;

impl<R: NeighsRDF + Debug + 'static> NativeValidator<R> for LessThanOrEquals {
    fn validate_native(
        &self,
        component: &ComponentIR,
        shape: &ShapeIR,
        store: &R,
        value_nodes: &ValueNodes<R>,
        _source_shape: Option<&ShapeIR>,
        maybe_path: Option<SHACLPath>,
        _shapes_graph: &SchemaIR,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        let mut validation_results = Vec::new();
        let component = Object::iri(component.into());

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
                                None => Some(format!(
                                    "LessThanOrEquals constraint violated: {node1} is not comparable to {node2}"
                                )),
                                Some(ord) if ord.is_gt() => Some(format!(
                                    "LessThanOrEquals constraint violated: {node1} is not less or equals than {node2}"
                                )),
                                _ => None,
                            };
                            if let Some(msg) = message {
                                let validation_result = ValidationResult::new(
                                    shape.id().clone(),
                                    component.clone(),
                                    shape.severity(),
                                )
                                .with_message(msg.as_str())
                                .with_path(maybe_path.clone());
                                validation_results.push(validation_result);
                            }
                        }
                    }
                }
                Err(e) => {
                    let message = format!(
                        "LessThanOrEquals: Error trying to find triples for subject {} and predicate {}: {e}",
                        subject,
                        self.iri()
                    );
                    let validation_result = ValidationResult::new(
                        shape.id().clone(),
                        component.clone(),
                        shape.severity(),
                    )
                    .with_message(message.as_str())
                    .with_path(maybe_path.clone());
                    validation_results.push(validation_result);
                }
            };
        }
        Ok(validation_results)
    }
}

impl<S: QueryRDF + Debug + 'static> SparqlValidator<S> for LessThanOrEquals {
    fn validate_sparql(
        &self,
        _component: &ComponentIR,
        _shape: &ShapeIR,
        _store: &S,
        _value_nodes: &ValueNodes<S>,
        _source_shape: Option<&ShapeIR>,
        _maybe_path: Option<SHACLPath>,
        _shapes_graph: &SchemaIR,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        Err(ConstraintError::NotImplemented(
            "LessThanOrEquals".to_string(),
        ))
    }
}
