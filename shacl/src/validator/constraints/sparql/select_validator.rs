use std::fmt::Debug;
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath};
use rudof_rdf::rdf_core::query::QueryRDF;
use rudof_rdf::rdf_core::term::literal::ConcreteLiteral;
use rudof_rdf::rdf_core::term::Object;
use crate::error::ValidationError;
use crate::ir::components::Sparql;
use crate::ir::{IRComponent, IRSchema, IRShape};
use crate::types::MessageMap;
use crate::validator::constraints::{NativeValidator, SparqlValidator};
use crate::validator::engine::Engine;
use crate::validator::nodes::ValueNodes;
use crate::validator::report::ValidationResult;

impl<RDF: NeighsRDF + Debug + 'static> NativeValidator<RDF> for Sparql {
    fn validate_native(&self, _: &IRComponent, _: &IRShape, _: &RDF, _: &mut dyn Engine<RDF>, _: &ValueNodes<RDF>, _: Option<&IRShape>, _: Option<&SHACLPath>, _: &IRSchema) -> Result<Vec<ValidationResult>, ValidationError> {
        // Silently skip since sh:sparql requires a SPARQL engine
        Ok(Vec::new())
    }
}

#[cfg(feature = "sparql")]
impl<RDF: QueryRDF + NeighsRDF + Debug + 'static> SparqlValidator<RDF> for Sparql {
    fn validate_sparql(&self, component: &IRComponent, shape: &IRShape, store: &RDF, value_nodes: &ValueNodes<RDF>, _: Option<&IRShape>, maybe_path: Option<&SHACLPath>, _: &IRSchema) -> Result<Vec<ValidationResult>, ValidationError> {
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
                if let Some(failure_term) = sol.find_solution("failure") {
                    if let Ok(Object::Literal(ConcreteLiteral::BooleanLiteral(true))) =
                        RDF::term_as_object(failure_term)
                    {
                        return Err(ValidationError::QueryError(
                            "SPARQL constraint produced a failure".to_string(),
                        ));
                    }
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
                    ValidationResult::new(
                        result_focus,
                        constraint_component.clone(),
                        shape.severity().clone(),
                    )
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

/// Converts a [`SHACLPath`] to its SPARQL property path [String] representation.
///
/// IRIs are enclosed in angle brackets and path operators follow SPARQL 1.1 property path syntax.
fn path_to_sparql(path: &SHACLPath) -> String {
    match path {
        SHACLPath::Predicate { pred } => format!("<{pred}>"),
        SHACLPath::Alternative { paths } => {
            let parts: Vec<_> = paths.iter().map(path_to_sparql).collect();
            format!("({})", parts.join("|"))
        },
        SHACLPath::Sequence { paths } => {
            let parts: Vec<_> = paths.iter().map(path_to_sparql).collect();
            format!("({})", parts.join("/"))
        },
        SHACLPath::Inverse { path } => format!("^({})", path_to_sparql(path)),
        SHACLPath::ZeroOrMore { path } => format!("({})*", path_to_sparql(path)),
        SHACLPath::OneOrMore { path } => format!("({})+", path_to_sparql(path)),
        SHACLPath::ZeroOrOne { path } => format!("({})?", path_to_sparql(path)),
    }
}

fn inject_values_into_where(query: &str, values_clause: &str) -> String {
    let upper = query.to_uppercase();
    if let Some(where_pos) = upper.find("WHERE") {
        if let Some(brace_offset) = query[where_pos..].find('{') {
            let insert_at = where_pos + brace_offset + 1;
            let mut result = query[..insert_at].to_string();
            result.push(' ');
            result.push_str(values_clause);
            result.push_str(&query[insert_at..]);
            return result;
        }
    }

    format!("{values_clause} {query}")
}
