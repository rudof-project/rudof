use crate::error::ValidationError;
use crate::ir::components::LessThan;
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

impl<S: NeighsRDF + Debug + 'static> NativeValidator<S> for LessThan {
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
        let component = Object::Iri(component.into());

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
                                    "LessThan constraint violated: {node1} is not comparable to {node2}"
                                )),
                                Some(ord) if ord.is_ge() => Some(format!(
                                    "LessThan constraint violated: {node1} is not less than {node2}"
                                )),
                                _ => None,
                            };

                            if let Some(msg) = msg {
                                let node_obj = S::term_as_object(value).ok();
                                let vr = ValidationResult::new(
                                    fnode_obj.clone(),
                                    component.clone(),
                                    shape.severity().clone(),
                                )
                                .with_message(MessageMap::from(msg))
                                .with_path(maybe_path.cloned())
                                .with_source(Some(shape.id().clone()))
                                .with_value(node_obj);
                                validation_results.push(vr);
                            }
                        }
                    }
                },
                Err(e) => {
                    let msg = format!(
                        "LessThan: Error trying to find triples for subject {subject} and predicate {}: {e}",
                        self.iri()
                    );
                    let vr = ValidationResult::new(fnode_obj, component.clone(), shape.severity().clone())
                        .with_path(maybe_path.cloned())
                        .with_message(MessageMap::from(msg))
                        .with_source(Some(shape.id().clone()));
                    validation_results.push(vr);
                },
            }
        }

        Ok(validation_results)
    }
}

#[cfg(feature = "sparql")]
impl<S: QueryRDF + NeighsRDF + Debug + 'static> BasicSparqlValidator<S> for LessThan {
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

            // emit one violation per (value_node, comparator) pair where
            // value_node is not strictly less than the comparator. COALESCE(?, false)
            // treats incomparable terms (e.g. cross-datatype) as violations.
            for vn in nodes.iter() {
                let query = formatdoc! {"
                    SELECT ?other WHERE {{
                        {} <{}> ?other .
                        BIND (COALESCE({} < ?other, false) AS ?lt)
                        FILTER (!?lt)
                    }}
                ", fnode, self.iri(), vn};

                let solutions = store
                    .query_select(&query)
                    .map_err(ValidationError::select_query_error::<S>)?;

                let value = S::term_as_object(vn).ok();
                for _ in solutions.iter() {
                    let msg = format!(
                        "LessThan constraint violated for property {}: value is not strictly less than every comparator",
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
