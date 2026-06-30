use crate::error::ValidationError;
use crate::ir::components::LessThanOrEquals;
use crate::ir::{IRComponent, IRSchema, IRShape};
use crate::types::MessageMap;
use crate::validator::constraints::{BasicSparqlValidator, NativeValidator};
use crate::validator::engine::Engine;
use crate::validator::nodes::ValueNodes;
use crate::validator::report::ValidationResult;
use indoc::formatdoc;
use rudof_rdf::rdf_core::query::QueryRDF;
use rudof_rdf::rdf_core::term::{Object, Triple};
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath};
use std::fmt::Debug;

impl<S: NeighsRDF + Debug + 'static> NativeValidator<S> for LessThanOrEquals {
    fn validate_native(
        &self,
        component: &IRComponent,
        shape: &IRShape,
        store: &S,
        _: &mut dyn Engine<S>,
        value_nodes: &ValueNodes<S>,
        _: Option<&IRShape>,
        maybe_path: Option<&SHACLPath>,
        _: &IRSchema,
    ) -> Result<Vec<ValidationResult>, ValidationError> {
        let mut validation_results = Vec::new();
        let component = Object::iri(component.into());

        for (fnode, nodes) in value_nodes.iter() {
            let subject = S::term_as_subject(fnode)?;
            let iri: S::IRI = self.iri().clone().into();
            let fnode_obj = S::term_as_object(fnode)?;

            match store.triples_with_subject_predicate(&subject, &iri) {
                Ok(triples_iter) => {
                    // Collect nodes to compare
                    for triple in triples_iter {
                        let node1 = S::term_as_object(triple.obj())?;
                        for value in nodes.iter() {
                            let node2 = S::term_as_object(value)?;
                            let msg = match node2.partial_cmp(&node1) {
                                None => Some(format!(
                                    "LessThanOrEquals constraint violated: {node1} is not comparable to {node2}"
                                )),
                                Some(ord) if ord.is_gt() => Some(format!(
                                    "LessThanOrEquals constraint violated: {node1} is not less or equals than {node2}"
                                )),
                                _ => None,
                            };

                            if let Some(msg) = msg {
                                let node_obj = S::term_as_object(value).ok();
                                let validation_result = ValidationResult::new(
                                    fnode_obj.clone(),
                                    component.clone(),
                                    shape.severity().clone(),
                                )
                                .with_message(MessageMap::from(msg))
                                .with_path(maybe_path.cloned())
                                .with_value(node_obj)
                                .with_source(Some(shape.id().clone()));
                                validation_results.push(validation_result);
                            }
                        }
                    }
                },
                Err(e) => {
                    let msg = format!(
                        "LessThanOrEquals: Error trying to find triples for subject {subject} and predicate {}: {e}",
                        self.iri()
                    );
                    let validation_result =
                        ValidationResult::new(fnode_obj, component.clone(), shape.severity().clone())
                            .with_message(MessageMap::from(msg))
                            .with_path(maybe_path.cloned())
                            .with_source(Some(shape.id().clone()));
                    validation_results.push(validation_result);
                },
            }
        }
        Ok(validation_results)
    }
}

#[cfg(feature = "sparql")]
impl<S: QueryRDF + NeighsRDF + Debug + 'static> BasicSparqlValidator<S> for LessThanOrEquals {
    fn validate_sparql(
        &self,
        component: &IRComponent,
        shape: &IRShape,
        store: &S,
        _: &mut dyn Engine<S>,
        value_nodes: &ValueNodes<S>,
        _: Option<&IRShape>,
        maybe_path: Option<&SHACLPath>,
        _: &IRSchema,
    ) -> Result<Vec<ValidationResult>, ValidationError> {
        let component_obj = Object::iri(component.into());
        let mut results = Vec::new();

        for (fnode, nodes) in value_nodes.iter() {
            let fnode_obj = S::term_as_object(fnode)?;

            for vn in nodes.iter() {
                let query = formatdoc! {"
                    SELECT ?other WHERE {{
                        {} <{}> ?other .
                        BIND (COALESCE({} <= ?other, false) AS ?lte)
                        FILTER (!?lte)
                    }}
                ", fnode, self.iri(), vn};

                let solutions = store
                    .query_select(&query)
                    .map_err(ValidationError::select_query_error::<S>)?;

                let value = S::term_as_object(vn).ok();
                for _ in solutions.iter() {
                    let msg = format!(
                        "LessThanOrEquals constraint violated for property {}: value is not less than or equal to every comparator",
                        self.iri()
                    );
                    let vr = ValidationResult::new(fnode_obj.clone(), component_obj.clone(), shape.severity().clone())
                        .with_source(Some(shape.id().clone()))
                        .with_path(maybe_path.cloned())
                        .with_value(value.clone())
                        .with_message(MessageMap::from(msg));
                    results.push(vr);
                }
            }
        }

        Ok(results)
    }
}
