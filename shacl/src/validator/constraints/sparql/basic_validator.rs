use crate::error::ValidationError;
use crate::ir::components::BasicSparql;
use crate::ir::{IRComponent, IRSchema, IRShape};
use crate::types::MessageMap;
use crate::validator::constraints::sparql::{inject_values_into_where, path_to_sparql};
use crate::validator::constraints::{BasicSparqlValidator, NativeValidator};
use crate::validator::engine::Engine;
use crate::validator::nodes::ValueNodes;
use crate::validator::report::ValidationResult;
use rudof_rdf::rdf_core::query::QueryRDF;
use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::term::literal::ConcreteLiteral;
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath};
use std::fmt::Debug;

impl<RDF: NeighsRDF + Debug + 'static> NativeValidator<RDF> for BasicSparql {
    fn validate_native(
        &self,
        _: &IRComponent,
        _: &IRShape,
        _: &RDF,
        _: &mut dyn Engine<RDF>,
        _: &ValueNodes<RDF>,
        _: Option<&IRShape>,
        _: Option<&SHACLPath>,
        _: &IRSchema,
    ) -> Result<Vec<ValidationResult>, ValidationError> {
        // Silently skip since sh:sparql requires a SPARQL engine
        Ok(Vec::new())
    }
}

#[cfg(feature = "sparql")]
impl<RDF: QueryRDF + NeighsRDF + Debug + 'static> BasicSparqlValidator<RDF> for BasicSparql {
    fn validate_sparql(
        &self,
        component: &IRComponent,
        shape: &IRShape,
        store: &RDF,
        _: &mut dyn Engine<RDF>,
        value_nodes: &ValueNodes<RDF>,
        _: Option<&IRShape>,
        maybe_path: Option<&SHACLPath>,
        _: &IRSchema,
    ) -> Result<Vec<ValidationResult>, ValidationError> {
        if self.deactivated() == Some(true) {
            return Ok(Vec::new());
        }

        let prefix_header = self
            .prefixes()
            .map(|p| {
                p.iter()
                    .map(|(prefix, iri)| format!("PREFIX {prefix}: <{iri}>\n"))
                    .collect::<String>()
            })
            .unwrap_or_default();

        // Substitute $PATH placeholder if this is a property shape
        let path_str = maybe_path.map(path_to_sparql).unwrap_or_default();
        let select_with_path = self.select().replace("$PATH", &path_str);

        let constraint_component = Object::Iri(component.into());
        let mut results = Vec::new();

        for (focus_node, _) in value_nodes.iter() {
            // Bind ?this via a VALUES clause and inject it in the WHERE block
            let values_clause = format!("VALUES ?this {{ {} }}", focus_node);
            let query_body = inject_values_into_where(&select_with_path, &values_clause);
            let full_query = format!("{}{}", prefix_header, query_body);

            let solutions = store
                .query_select(&full_query)
                .map_err(ValidationError::select_query_error::<RDF>)?;

            for sol in solutions.iter() {
                // A binding of ?failure = true signals a constraint failure
                if let Some(failure_term) = sol.find_solution("failure")
                    && let Ok(Object::Literal(ConcreteLiteral::BooleanLiteral(true))) =
                        RDF::term_as_object(failure_term)
                {
                    return Err(ValidationError::QueryError(
                        "SPARQL constraint produced a failure".to_string(),
                    ));
                }

                // Each non-failure solution is a violation
                let result_focus = if let Some(this_term) = sol.find_solution("this") {
                    RDF::term_as_object(this_term)?
                } else {
                    RDF::term_as_object(focus_node)?
                };

                // sh:resultPath: prefer ?path binding, then fall back to the shape's path
                let result_path = sol
                    .find_solution("path")
                    .and_then(|t| {
                        if let Ok(Object::Iri(iri)) = RDF::term_as_object(t) {
                            Some(SHACLPath::Predicate { pred: iri })
                        } else {
                            None
                        }
                    })
                    .or_else(|| maybe_path.cloned());

                // sh:value: use ?value binding, or fall back to the focus node
                let value = sol
                    .find_solution("value")
                    .and_then(|t| RDF::term_as_object(t).ok())
                    .or_else(|| RDF::term_as_object(focus_node).ok());

                // sh:resultMessage: prefer ?message binding, then self.message
                let message = if let Some(msg_term) = sol.find_solution("message") {
                    MessageMap::from(format!("{msg_term}"))
                } else {
                    self.message().cloned().unwrap_or_default()
                };

                results.push(
                    ValidationResult::new(result_focus, constraint_component.clone(), shape.severity().clone())
                        .with_source(Some(shape.id().clone()))
                        .with_path(result_path)
                        .with_value(value)
                        .with_message(message),
                );
            }
        }

        Ok(results)
    }
}
