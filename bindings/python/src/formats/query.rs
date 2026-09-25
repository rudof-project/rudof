use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use rudof_lib::formats::{QueryType, ResultQueryFormat};

pyenum! {
    /// The kind of SPARQL query being run.
    "QueryType": PyQueryType => QueryType, default = Select {
        /// SELECT - returns a table of bindings.
        Select    => Select,
        /// CONSTRUCT - returns an RDF graph.
        Construct => Construct,
        /// ASK - returns a boolean.
        Ask       => Ask,
        /// DESCRIBE - returns an RDF graph describing the matched resources.
        Describe  => Describe,
    }
}

pyenum! {
    /// Output formats for SPARQL query results.
    "QueryResultFormat": PyQueryResultFormat => ResultQueryFormat, default = Internal, from_str {
        /// Internal representation used for processing.
        Internal => Internal,
        /// Turtle - compact, human-readable RDF format.
        Turtle   => Turtle,
        /// N-Triples - line-based RDF format with one triple per line.
        NTriples => NTriples,
        /// JSON-LD - JSON serialization for Linked Data.
        JsonLd   => JsonLd,
        /// JSON - machine-readable JSON serialization.
        Json     => Json,
        /// RDF/XML - XML-based RDF serialization.
        RdfXml   => RdfXml,
        /// CSV - comma-separated values for spreadsheet tools.
        Csv      => Csv,
        /// Markdown - a table suitable for documentation.
        Markdown => Markdown,
        /// TriG - Turtle extended with named graphs.
        TriG     => TriG,
        /// Notation3 - superset of Turtle.
        N3       => N3,
        /// N-Quads - N-Triples extended with named graphs.
        NQuads   => NQuads,
    }
}

/// The result of the most recent :meth:`Rudof.run_query` call.
#[cfg_attr(feature = "stub-gen", pyo3_stub_gen_derive::gen_stub_pyclass)]
#[pyclass(frozen, name = "QueryResults", module = "pyrudof._pyrudof")]
pub struct PyQueryResults {
    variables: Vec<String>,
    rows: Vec<Vec<Option<String>>>,
    boolean: Option<bool>,
    graph: Option<String>,
}

impl PyQueryResults {
    /// The result of a SELECT query.
    pub(crate) fn select(variables: Vec<String>, rows: Vec<Vec<Option<String>>>) -> Self {
        Self {
            variables,
            rows,
            boolean: None,
            graph: None,
        }
    }

    /// The answer to an ASK query.
    pub(crate) fn ask(answer: bool) -> Self {
        Self {
            variables: Vec::new(),
            rows: Vec::new(),
            boolean: Some(answer),
            graph: None,
        }
    }

    /// The serialized graph a CONSTRUCT or DESCRIBE query returned.
    pub(crate) fn from_graph(graph: String) -> Self {
        Self {
            variables: Vec::new(),
            rows: Vec::new(),
            boolean: None,
            graph: Some(graph),
        }
    }
}

#[cfg_attr(feature = "stub-gen", pyo3_stub_gen_derive::gen_stub_pymethods)]
#[pymethods]
impl PyQueryResults {
    /// The projected variable names, in SELECT order. Empty for ASK and CONSTRUCT.
    #[getter]
    fn variables(&self) -> Vec<String> {
        self.variables.clone()
    }

    /// One list of cells per solution, aligned with :attr:`variables`. Empty for ASK and
    /// CONSTRUCT.
    #[getter]
    fn rows(&self) -> Vec<Vec<Option<String>>> {
        self.rows.clone()
    }

    /// The answer to an ASK query, or ``None`` for every other query type.
    #[getter]
    fn boolean(&self) -> Option<bool> {
        self.boolean
    }

    /// The serialized graph of a CONSTRUCT or DESCRIBE query, or ``None`` otherwise.
    #[getter]
    fn graph(&self) -> Option<String> {
        self.graph.clone()
    }

    /// ``True`` when the result is the answer to an ASK query.
    #[getter]
    fn is_boolean(&self) -> bool {
        self.boolean.is_some()
    }

    /// ``True`` when the result is a graph, from a CONSTRUCT or DESCRIBE query.
    #[getter]
    fn is_graph(&self) -> bool {
        self.graph.is_some()
    }

    /// The result as plain Python: a list of dicts for SELECT, a ``bool`` for ASK, a ``str`` for CONSTRUCT and DESCRIBE.
    fn to_python(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        if let Some(answer) = self.boolean {
            return Ok(answer.into_pyobject(py)?.to_owned().into_any().unbind());
        }
        if let Some(graph) = &self.graph {
            return Ok(graph.into_pyobject(py)?.into_any().unbind());
        }
        let solutions = PyList::empty(py);
        for row in &self.rows {
            let solution = PyDict::new(py);
            for (variable, cell) in self.variables.iter().zip(row) {
                solution.set_item(variable, cell)?;
            }
            solutions.append(solution)?;
        }
        Ok(solutions.into_any().unbind())
    }

    /// The number of solutions. ``1`` for ASK, ``0`` for a graph result.
    fn __len__(&self) -> usize {
        if self.boolean.is_some() { 1 } else { self.rows.len() }
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyQueryRows {
        PyQueryRows {
            variables: slf.variables.clone(),
            inner: slf.rows.clone().into_iter(),
        }
    }

    fn __repr__(&self) -> String {
        if let Some(answer) = self.boolean {
            format!("QueryResults(boolean={})", if answer { "True" } else { "False" })
        } else if self.graph.is_some() {
            "QueryResults(graph=...)".to_string()
        } else {
            format!("QueryResults(variables={:?}, rows={})", self.variables, self.rows.len())
        }
    }
}

/// Iterator over a :class:`QueryResults`' solutions, one dict per solution.
#[cfg_attr(feature = "stub-gen", pyo3_stub_gen_derive::gen_stub_pyclass)]
#[pyclass(name = "QueryRowIterator", module = "pyrudof._pyrudof")]
pub struct PyQueryRows {
    variables: Vec<String>,
    inner: std::vec::IntoIter<Vec<Option<String>>>,
}

#[cfg_attr(feature = "stub-gen", pyo3_stub_gen_derive::gen_stub_pymethods)]
#[cfg_attr(not(feature = "stub-gen"), pyo3_stub_gen_derive::remove_gen_stub)]
#[pymethods]
impl PyQueryRows {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    // `None` is how pyo3 signals StopIteration; every yielded row is a dict mapping
    // each variable to its binding, so the stub names that instead of `Any | None`.
    #[gen_stub(override_return_type(
            type_repr = "builtins.dict[builtins.str, typing.Optional[builtins.str]]",
            imports = ("builtins", "typing")
        ))]
    fn __next__(&mut self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        let Some(row) = self.inner.next() else {
            return Ok(None);
        };
        let solution = PyDict::new(py);
        for (variable, cell) in self.variables.iter().zip(&row) {
            solution.set_item(variable, cell)?;
        }
        Ok(Some(solution.into_any().unbind()))
    }

    fn __length_hint__(&self) -> usize {
        self.inner.len()
    }
}
