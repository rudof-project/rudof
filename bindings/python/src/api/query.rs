use crate::{
    api::PyRudof,
    error::Result,
    formats::{PyQueryResultFormat, PyQueryResults, PyQueryType},
    input::InputArg,
    output,
};
use pyo3::prelude::*;
use rudof_lib::errors::RudofError as CoreError;
use rudof_lib::formats::{QueryType, ResultQueryFormat};
use rudof_lib::types::QueryResult;

#[cfg_attr(feature = "stub-gen", pyo3_stub_gen_derive::gen_stub_pymethods)]
#[pymethods]
impl PyRudof {
    /// Loads a SPARQL query from a string, file path or URL.
    ///
    /// Args:
    ///     input (str | os.PathLike): Inline query, file path or URL.
    ///     query_type (QueryType, optional): Type of SPARQL query. Auto-detected when omitted.
    ///
    /// Raises:
    ///     InputError: If the input string, file or URL cannot be resolved.
    ///     QueryError: If the query is malformed.
    #[pyo3(signature = (input, query_type = None))]
    fn read_query(&mut self, py: Python<'_>, input: InputArg, query_type: Option<&PyQueryType>) -> Result<()> {
        let InputArg(input) = input;
        let query_type: Option<QueryType> = query_type.map(Into::into);

        py.detach(move || {
            let mut b = self.inner.load_sparql_query(&input);
            if let Some(t) = &query_type {
                b = b.with_query_type(t);
            }
            b.execute()
        })?;
        Ok(())
    }

    /// Executes the loaded query against the loaded data.
    ///
    /// Returns:
    ///     QueryResults: A snapshot of the result. A SELECT fills ``variables`` and
    ///     ``rows``, an ASK fills ``boolean``, and a CONSTRUCT or DESCRIBE fills ``graph``.
    ///
    /// Raises:
    ///     QueryError: If no query is loaded, or execution fails.
    #[pyo3(signature = ())]
    fn run_query(&mut self, py: Python<'_>) -> Result<PyQueryResults> {
        py.detach(|| self.inner.run_query().execute())?;

        let results = self.inner.query_results().ok_or(CoreError::Generic {
            error: "run_query produced no results".into(),
        })?;

        Ok(match results {
            QueryResult::Ask(answer) => PyQueryResults::ask(*answer),
            QueryResult::Construct(graph) => PyQueryResults::from_graph(graph.clone()),
            QueryResult::Select(solutions) => {
                // The variable list is per-solution in `QuerySolutions`; take it from the
                // first row, which is what `serialize_query_results` does for its header.
                let variables: Vec<String> = solutions
                    .iter()
                    .next()
                    .map(|s| s.variables().iter().map(|v| v.to_string()).collect())
                    .unwrap_or_default();
                let rows = solutions
                    .iter()
                    .map(|solution| {
                        (0..solution.variables().len())
                            .map(|i| solution.find_solution(i).map(|t| t.to_string()))
                            .collect()
                    })
                    .collect();
                PyQueryResults::select(variables, rows)
            },
        })
    }

    /// Serializes the results of the most recent :meth:`run_query` call to a string.
    ///
    /// Args:
    ///     format (QueryResultFormat, optional): Output format. Defaults to ``QueryResultFormat.Internal``.
    ///
    /// Returns:
    ///     str: Serialized query results.
    ///
    /// Raises:
    ///     QueryError: If there are no results, or serialization fails.
    #[pyo3(signature = (format = None))]
    fn serialize_query_results(&self, py: Python<'_>, format: Option<&PyQueryResultFormat>) -> Result<String> {
        let format: Option<ResultQueryFormat> = format.map(Into::into);
        output::capture_string_detached(py, move |w| {
            let mut s = self.inner.serialize_query_results(w);
            if let Some(f) = &format {
                s = s.with_result_query_format(f);
            }
            s.execute()
        })
    }

    /// Lists known SPARQL endpoints.
    ///
    /// Returns:
    ///     list[tuple[str, str]]: ``(name, url)`` tuples for known endpoints.
    ///
    /// Raises:
    ///     QueryError: If the endpoint registry cannot be read.
    fn list_endpoints(&mut self, py: Python<'_>) -> Result<Vec<(String, String)>> {
        let endpoints = py.detach(|| self.inner.list_endpoints().execute())?;
        Ok(endpoints)
    }
}
