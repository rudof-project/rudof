use crate::error::ValidationError;
use crate::ir::components::Closed;
use crate::ir::{IRComponent, IRSchema, IRShape};
use crate::validator::constraints::NativeValidator;
use crate::validator::engine::Engine;
use crate::validator::nodes::ValueNodes;
use crate::validator::report::ValidationResult;
use rudof_rdf::rdf_core::term::{Object, Term, Triple};
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath};
use std::fmt::Debug;

#[cfg(feature = "sparql")]
use crate::validator::constraints::BasicSparqlValidator;
#[cfg(feature = "sparql")]
use indoc::formatdoc;
#[cfg(feature = "sparql")]
use rudof_rdf::rdf_core::query::QueryRDF;

impl<S: NeighsRDF + Debug + 'static> NativeValidator<S> for Closed {
    fn validate_native(
        &self,
        component: &IRComponent,
        shape: &IRShape,
        store: &S,
        _: &mut dyn Engine<S>,
        value_nodes: &ValueNodes<S>,
        _: Option<&IRShape>,
        _: Option<&SHACLPath>,
        _: &IRSchema,
    ) -> Result<Vec<ValidationResult>, ValidationError> {
        if !self.is_closed() {
            return Ok(Vec::new());
        }

        let allowed_props = shape.allowed_properties();
        let component_obj = Object::iri(component.into());
        let mut results = Vec::new();

        for (fnode, _) in value_nodes.iter() {
            let subject = match S::term_as_subject(fnode) {
                Ok(subj) => subj,
                Err(_) => continue,
            };

            let triples = store
                .triples_with_subject(&subject)
                .map_err(ValidationError::new_graph_error::<S>)?;

            let focus_obj = S::term_as_object(fnode)?;

            for triple in triples {
                let (_, pred, obj) = triple.into_components();
                let pred_iri = pred.into();
                if !allowed_props.contains(&pred_iri) {
                    let value = S::term_as_object(&obj).ok();
                    let vr = ValidationResult::new(focus_obj.clone(), component_obj.clone(), shape.severity().clone())
                        .with_source(Some(shape.id().clone()))
                        .with_path(Some(SHACLPath::iri(pred_iri)))
                        .with_value(value);
                    results.push(vr);
                }
            }
        }

        Ok(results)
    }
}

#[cfg(feature = "sparql")]
impl<S: QueryRDF + NeighsRDF + Debug + 'static> BasicSparqlValidator<S> for Closed {
    fn validate_sparql(
        &self,
        component: &IRComponent,
        shape: &IRShape,
        store: &S,
        _: &mut dyn Engine<S>,
        value_nodes: &ValueNodes<S>,
        _: Option<&IRShape>,
        _: Option<&SHACLPath>,
        _: &IRSchema,
    ) -> Result<Vec<ValidationResult>, ValidationError> {
        if !self.is_closed() {
            return Ok(Vec::new());
        }

        let allowed_props = shape.allowed_properties();
        let component_obj = Object::iri(component.into());
        let mut results = Vec::new();

        let not_in_clause = if allowed_props.is_empty() {
            String::new()
        } else {
            let iris = allowed_props
                .iter()
                .map(|iri| format!("<{iri}>"))
                .collect::<Vec<_>>()
                .join(", ");
            format!(" FILTER (?p NOT IN ({iris}))")
        };

        for (fnode, _) in value_nodes.iter() {
            let focus_obj = S::term_as_object(fnode)?;

            let triples_iter: Box<dyn Iterator<Item = (Object, Option<Object>)>> = if fnode.is_blank_node() {
                let subject = match S::term_as_subject(fnode) {
                    Ok(s) => s,
                    Err(_) => continue,
                };
                let triples = store
                    .triples_with_subject(&subject)
                    .map_err(ValidationError::new_graph_error::<S>)?;
                Box::new(triples.into_iter().filter_map(|t| {
                    let (_, pred, obj) = t.into_components();
                    let pred_iri = pred.into();
                    if allowed_props.contains(&pred_iri) {
                        None
                    } else {
                        Some((Object::Iri(pred_iri), S::term_as_object(&obj).ok()))
                    }
                }))
            } else {
                let query = formatdoc! {"
                    SELECT ?p ?o WHERE {{ {} ?p ?o .{} }}
                ", fnode, not_in_clause};

                let solutions = store
                    .query_select(&query)
                    .map_err(ValidationError::select_query_error::<S>)?;

                let mut rows: Vec<(Object, Option<Object>)> = Vec::new();
                for sol in solutions.iter() {
                    let pred_term = match sol.find_solution("p") {
                        Some(t) => t,
                        None => continue,
                    };
                    let pred_obj = match S::term_as_object(pred_term) {
                        Ok(obj @ Object::Iri(_)) => obj,
                        _ => continue,
                    };
                    let value = sol.find_solution("o").and_then(|t| S::term_as_object(t).ok());
                    rows.push((pred_obj, value));
                }
                Box::new(rows.into_iter())
            };

            for (pred_obj, value) in triples_iter {
                let path = match &pred_obj {
                    Object::Iri(iri) => Some(SHACLPath::iri(iri.clone())),
                    _ => None,
                };
                let vr = ValidationResult::new(focus_obj.clone(), component_obj.clone(), shape.severity().clone())
                    .with_source(Some(shape.id().clone()))
                    .with_path(path)
                    .with_value(value);
                results.push(vr);
            }
        }

        Ok(results)
    }
}
