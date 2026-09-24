use pyo3::prelude::*;
use rudof_lib::formats::{
    ResultShaclValidationFormat, ShaclFormat, ShaclValidationMode, ShaclValidationSortByMode,
};

pyenum! {
    /// SHACL shapes graph serialization formats.
    "ShaclFormat": PyShaclFormat => ShaclFormat, default = Turtle, from_str {
        /// Internal representation used for processing.
        Internal => Internal,
        /// Turtle - compact, human-readable RDF format.
        Turtle   => Turtle,
        /// N-Triples - line-based RDF format with one triple per line.
        NTriples => NTriples,
        /// RDF/XML - XML-based RDF serialization.
        RdfXml   => RdfXml,
        /// TriG - Turtle extended with named graphs.
        TriG     => TriG,
        /// Notation3 - superset of Turtle.
        N3       => N3,
        /// N-Quads - N-Triples extended with named graphs.
        NQuads   => NQuads,
        /// JSON-LD - JSON serialization for Linked Data.
        JsonLd   => JsonLd,
        /// JSON - machine-readable JSON serialization.
        Json     => Json,
    }
}

pyenum! {
    /// SHACL validation engine.
    "ShaclValidationMode": PyShaclValidationMode => ShaclValidationMode, default = Native {
        /// Rust-native engine.
        Native => Native,
        /// SPARQL-based engine, validating through SPARQL queries.
        Sparql => Sparql,
    }
}

pyenum! {
    /// Sort mode for displaying a SHACL validation report as a table.
    "ShaclValidationSortMode": PyShaclValidationSortMode => ShaclValidationSortByMode,
    default = Severity {
        /// Group results by violation severity.
        Severity    => Severity,
        /// Group results by the focus node being validated.
        Node        => Node,
        /// Group results by SHACL constraint component.
        Component   => Component,
        /// Group results by the value that caused the violation.
        Value       => Value,
        /// Group results by the property path where violations occur.
        Path        => Path,
        /// Group results by the shape that was violated.
        SourceShape => SourceShape,
        /// Group results by detailed information.
        Details     => Details,
    }
}

pyenum! {
    /// Output formats for SHACL validation results.
    "ResultShaclValidationFormat": PyResultShaclValidationFormat => ResultShaclValidationFormat,
    default = Details, from_str {
        /// Verbose output including validation details.
        Details  => Details,
        /// Turtle - compact, human-readable RDF format.
        Turtle   => Turtle,
        /// N-Triples - line-based RDF format with one triple per line.
        NTriples => NTriples,
        /// RDF/XML - XML-based RDF serialization.
        RdfXml   => RdfXml,
        /// TriG - Turtle extended with named graphs.
        TriG     => TriG,
        /// Notation3 - superset of Turtle.
        N3       => N3,
        /// N-Quads - N-Triples extended with named graphs.
        NQuads   => NQuads,
        /// Minimal - the shortest possible validation report.
        Minimal  => Minimal,
        /// Compact - concise validation output.
        Compact  => Compact,
        /// JSON - machine-readable JSON serialization.
        Json     => Json,
        /// CSV - comma-separated values for spreadsheet tools.
        Csv      => Csv,
    }
}

// `ShapesGraphSource` has no counterpart in `rudof_lib`: it is a binding-only enum with no
// conversion and no call site today (the plan's claim that it has a `cnv_*` function does
// not match the tree). It therefore cannot go through `pyenum!`, which needs a target type
// to convert to. It is kept here, hand-written, so the Python API surface is unchanged;
// fold it into `pyenum!` as soon as `rudof_lib` grows the type it should map to.
/// Source of the SHACL shapes graph used during validation.
///
/// Shapes can come from the current SHACL schema or be extracted from the current RDF
/// data graph.
#[cfg_attr(feature = "stub-gen", pyo3_stub_gen_derive::gen_stub_pyclass_enum)]
#[pyclass(eq, eq_int, hash, frozen, from_py_object, name = "ShapesGraphSource", module = "pyrudof._pyrudof")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PyShapesGraphSource {
    /// Shapes come from the current RDF data graph.
    CurrentData,
    /// Shapes come from the current SHACL schema.
    CurrentSchema,
}

#[cfg_attr(feature = "stub-gen", pyo3_stub_gen_derive::gen_stub_pymethods)]
#[pymethods]
impl PyShapesGraphSource {
    #[new]
    fn __init__() -> Self {
        PyShapesGraphSource::CurrentSchema
    }

    fn __str__(&self) -> String {
        match self {
            PyShapesGraphSource::CurrentData => "CurrentData".to_string(),
            PyShapesGraphSource::CurrentSchema => "CurrentSchema".to_string(),
        }
    }

    fn __repr__(&self) -> String {
        format!("ShapesGraphSource.{}", self.__str__())
    }

    /// Every variant of this enum, in declaration order.
    #[classmethod]
    fn all(_cls: &Bound<'_, pyo3::types::PyType>) -> Vec<PyShapesGraphSource> {
        vec![
            PyShapesGraphSource::CurrentData,
            PyShapesGraphSource::CurrentSchema,
        ]
    }
}

use rudof_lib::types::{ShaclValidationReport, ShaclValidationResult};

/// One violation of a SHACL validation report.
#[cfg_attr(feature = "stub-gen", pyo3_stub_gen_derive::gen_stub_pyclass)]
#[pyclass(frozen, get_all, from_py_object, name = "ShaclValidationEntry", module = "pyrudof._pyrudof")]
#[derive(Clone)]
pub struct PyShaclValidationEntry {
    /// The node that failed validation, as a string.
    pub focus_node: String,
    /// The property path the violation was found at, if any.
    pub path: Option<String>,
    /// The value that caused the violation, if any.
    pub value: Option<String>,
    /// The shape that was violated, if any.
    pub source_shape: Option<String>,
    /// The SHACL constraint component that reported the violation.
    pub constraint_component: String,
    /// ``"Violation"``, ``"Warning"``, ``"Info"``, ``"Debug"``, ``"Trace"``, or the IRI
    /// of a custom severity.
    pub severity: String,
    /// The messages attached to the violation, joined by newlines. Empty when the shape
    /// declares no ``sh:message``.
    pub message: String,
}

#[cfg_attr(feature = "stub-gen", pyo3_stub_gen_derive::gen_stub_pymethods)]
#[pymethods]
impl PyShaclValidationEntry {
    fn __repr__(&self) -> String {
        format!(
            "ShaclValidationEntry(focus_node='{}', component='{}', severity='{}')",
            self.focus_node, self.constraint_component, self.severity
        )
    }
}

impl From<&ShaclValidationResult> for PyShaclValidationEntry {
    fn from(result: &ShaclValidationResult) -> Self {
        PyShaclValidationEntry {
            focus_node: result.focus_node().to_string(),
            path: result.path().map(|p| p.to_string()),
            value: result.value().map(|v| v.to_string()),
            source_shape: result.source().map(|s| s.to_string()),
            constraint_component: result.constraint_component().to_string(),
            severity: result.severity().to_string(),
            message: result.message().to_string(),
        }
    }
}

/// The result of the most recent :meth:`Rudof.validate_shacl` call.
///
/// A snapshot: it owns its data and is unaffected by later calls on the session.
#[cfg_attr(feature = "stub-gen", pyo3_stub_gen_derive::gen_stub_pyclass)]
#[pyclass(frozen, name = "ShaclValidationReport", module = "pyrudof._pyrudof")]
pub struct PyShaclValidationReport {
    // Kept so the report can serialize itself once `rudof_lib` grows the standalone
    // serializers; iteration goes through `entries`, which a `#[pyclass]` iterator can own.
    #[allow(dead_code)]
    pub(crate) inner: ShaclValidationReport,
    pub(crate) conforms: bool,
    pub(crate) entries: Vec<PyShaclValidationEntry>,
}

impl PyShaclValidationReport {
    pub(crate) fn new(inner: ShaclValidationReport) -> Self {
        let conforms = inner.conforms();
        let entries = inner.results().iter().map(Into::into).collect();
        Self {
            inner,
            conforms,
            entries,
        }
    }
}

#[cfg_attr(feature = "stub-gen", pyo3_stub_gen_derive::gen_stub_pymethods)]
#[pymethods]
impl PyShaclValidationReport {
    /// ``True`` if the data conforms to the shapes graph.
    ///
    /// Reported by the validator, not derived from the entries.
    #[getter]
    fn conforms(&self) -> bool {
        self.conforms
    }

    /// Every violation in the report. Alias of iterating the report.
    #[getter]
    fn violations(&self) -> Vec<PyShaclValidationEntry> {
        self.entries.clone()
    }

    fn __len__(&self) -> usize {
        self.entries.len()
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyShaclValidationEntries {
        PyShaclValidationEntries {
            inner: slf.entries.clone().into_iter(),
        }
    }

    fn __bool__(&self) -> bool {
        self.conforms
    }

    fn __repr__(&self) -> String {
        format!(
            "ShaclValidationReport(conforms={}, entries={})",
            if self.conforms { "True" } else { "False" },
            self.entries.len()
        )
    }
}

/// Iterator over a :class:`ShaclValidationReport`'s entries.
#[cfg_attr(feature = "stub-gen", pyo3_stub_gen_derive::gen_stub_pyclass)]
#[pyclass(name = "ShaclValidationEntryIterator", module = "pyrudof._pyrudof")]
pub struct PyShaclValidationEntries {
    inner: std::vec::IntoIter<PyShaclValidationEntry>,
}

#[cfg_attr(feature = "stub-gen", pyo3_stub_gen_derive::gen_stub_pymethods)]
#[cfg_attr(not(feature = "stub-gen"), pyo3_stub_gen_derive::remove_gen_stub)]
#[pymethods]
impl PyShaclValidationEntries {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    // `None` is how pyo3 signals StopIteration; it is never yielded to Python, so
    // the stub must not widen the element type to `Entry | None`.
    #[gen_stub(override_return_type(type_repr = "ShaclValidationEntry"))]
    fn __next__(&mut self) -> Option<PyShaclValidationEntry> {
        self.inner.next()
    }

    fn __length_hint__(&self) -> usize {
        self.inner.len()
    }
}
