use crate::errors::QueryError;
use rudof_rdf::rdf_core::{Rdf, query::QuerySolutions, term::Object};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sparql_service::RdfData;
use std::collections::BTreeMap;

/// The variable bindings of a SPARQL `SELECT` result, with every term qualified as an
/// [`Object`] rather than left in the backend's own term type.
///
/// Shaped like the `SELECT` half of the SPARQL 1.1 Query Results JSON Format: the
/// variables are listed in the order the query declares them, and a row binds only the
/// variables that have a value, so a variable missing from a row is unbound in it.
///
/// Built from the [`QueryResult::Select`](crate::types::QueryResult::Select) solutions:
///
/// ```ignore
/// if let Some(QueryResult::Select(solutions)) = rudof.query_results() {
///     let bindings = Bindings::try_from(solutions)?;
/// }
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Bindings {
    variables: Vec<String>,
    rows: Vec<BTreeMap<String, Object>>,
}

impl Bindings {
    /// The variable names, without their `?`, in the order the query declares them.
    pub fn variables(&self) -> &[String] {
        &self.variables
    }

    /// One row per solution, binding the variables of [`Self::variables`] that have a
    /// value in it.
    pub fn rows(&self) -> &[BTreeMap<String, Object>] {
        &self.rows
    }

    /// The number of solutions.
    pub fn count(&self) -> usize {
        self.rows.len()
    }
}

impl TryFrom<&QuerySolutions<RdfData>> for Bindings {
    type Error = QueryError;

    fn try_from(solutions: &QuerySolutions<RdfData>) -> std::result::Result<Self, Self::Error> {
        // Every solution of a SELECT carries the same variables, so the first one names the
        // columns; a result with no solutions has none to take them from.
        let variables: Vec<String> = solutions
            .iter()
            .next()
            .map(|solution| solution.variables_iter().map(|var| var.as_str().to_string()).collect())
            .unwrap_or_default();

        let mut rows = Vec::with_capacity(solutions.count());
        for solution in solutions.iter() {
            let mut row = BTreeMap::new();
            for (column, variable) in variables.iter().enumerate() {
                if let Some(term) = solution.find_solution(column) {
                    let object = RdfData::term_as_object(term)
                        .map_err(|e| QueryError::FailedQualifyingSolution { error: e.to_string() })?;
                    row.insert(variable.clone(), object);
                }
            }
            rows.push(row);
        }

        Ok(Bindings { variables, rows })
    }
}
