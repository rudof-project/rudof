use crate::error::Result;
use crate::formats::PyRDFFormat;
use pyo3::prelude::*;
use rudof_lib::errors::RudofError as CoreError;
use rudof_lib::formats::{DataFormat, ResultShExValidationFormat, ShExFormat, ShExValidationSortByMode};
use rudof_lib::types::{ResultShapeMap, ShExValidationStatus};
use rudof_rdf::rdf_core::{BuildRDF, RDFFormat};
use rudof_rdf::rdf_impl::OxigraphInMemory;

pyenum! {
    /// ShEx schema serialization formats.
    "ShExFormat": PyShExFormat => ShExFormat, default = ShExC, from_str {
        /// Internal representation used for processing.
        Internal => Internal,
        /// Simplified ShEx representation.
        Simple   => Simple,
        /// ShExC - compact ShEx syntax (human-readable).
        ShExC    => ShExC,
        /// ShExJ - JSON representation of a ShEx schema.
        ShExJ    => ShExJ,
        /// JSON - generic JSON format.
        Json     => Json,
        /// JSON-LD - JSON serialization for Linked Data.
        JsonLd   => JsonLd,
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
        /// PlantUML - text-based UML diagram source.
        PlantUML => PlantUML,
        /// SVG - Scalable Vector Graphics image.
        Svg      => Svg,
        /// PNG - Portable Network Graphics image.
        Png      => Png,
        /// Binary - precompiled schema cache.
        Binary   => Binary,
    }
}

pyenum! {
    /// Output formats for ShEx validation results.
    "ResultShexValidationFormat": PyResultShexValidationFormat => ResultShExValidationFormat,
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
        /// Compact - concise ShapeMap representation.
        Compact  => Compact,
        /// JSON - machine-readable JSON serialization.
        Json     => Json,
        /// CSV - comma-separated values for spreadsheet tools.
        Csv      => Csv,
    }
}

pyenum! {
    /// Sort mode for displaying a ShEx validation result as a table.
    "ShexValidationSortMode": PyShexValidationSortMode => ShExValidationSortByMode,
    default = Node {
        /// Sort rows by focus node.
        Node    => Node,
        /// Sort rows by shape label.
        Shape   => Shape,
        /// Sort rows by validation status.
        Status  => Status,
        /// Sort rows by detailed information.
        Details => Details,
    }
}

/// One ``(node, shape, status)`` row of a ShEx validation result.
#[cfg_attr(feature = "stub-gen", pyo3_stub_gen_derive::gen_stub_pyclass)]
#[pyclass(
    frozen,
    get_all,
    from_py_object,
    name = "ShExValidationEntry",
    module = "pyrudof._pyrudof"
)]
#[derive(Clone)]
pub struct PyShExValidationEntry {
    /// The focus node, as a string.
    pub node: String,
    /// The shape label it was checked against.
    pub shape: String,
    /// ``"conformant"``, ``"nonconformant"``, ``"pending"`` or ``"inconsistent"``.
    pub status: String,
    /// The reason or appinfo carried by the status, if any.
    pub details: Option<String>,
}

#[cfg_attr(feature = "stub-gen", pyo3_stub_gen_derive::gen_stub_pymethods)]
#[pymethods]
impl PyShExValidationEntry {
    fn __repr__(&self) -> String {
        format!(
            "ShExValidationEntry(node='{}', shape='{}', status='{}')",
            self.node, self.shape, self.status
        )
    }
}

/// The result of the most recent :meth:`Rudof.validate_shex` call.
#[cfg_attr(feature = "stub-gen", pyo3_stub_gen_derive::gen_stub_pyclass)]
#[pyclass(frozen, name = "ShExValidationReport", module = "pyrudof._pyrudof")]
pub struct PyShExValidationReport {
    #[allow(dead_code)]
    pub(crate) inner: ResultShapeMap,
    pub(crate) entries: Vec<PyShExValidationEntry>,
    pub(crate) read: Option<ReadTriples>,
}

/// The triples a validation read: how many reads, and the distinct triples.
pub(crate) struct ReadTriples {
    pub(crate) reads: u64,
    pub(crate) graph: OxigraphInMemory,
}

impl PyShExValidationReport {
    pub(crate) fn new(inner: ResultShapeMap, read: Option<ReadTriples>) -> Self {
        let entries = inner
            .iter()
            .map(|(node, shape, status)| PyShExValidationEntry {
                node: node.to_string(),
                shape: shape.to_string(),
                status: status.code(),
                details: status_details(status),
            })
            .collect();
        Self { inner, entries, read }
    }
}

fn status_details(status: &ShExValidationStatus) -> Option<String> {
    match status {
        ShExValidationStatus::Conformant(info) => Some(info.to_string()),
        ShExValidationStatus::NonConformant(info) => Some(info.to_string()),
        ShExValidationStatus::Pending => None,
        ShExValidationStatus::Inconsistent(conformant, non_conformant) => {
            Some(format!("Conformant: {conformant}, Non-conformant: {non_conformant}"))
        },
    }
}

#[cfg_attr(feature = "stub-gen", pyo3_stub_gen_derive::gen_stub_pymethods)]
#[pymethods]
impl PyShExValidationReport {
    /// ``True`` if every entry is conformant.
    ///
    /// An empty report is vacuously conformant; use ``len()`` to tell the two apart.
    #[getter]
    fn conforms(&self) -> bool {
        self.entries.iter().all(|e| e.status == "conformant")
    }

    /// The entries that are **not** conformant.
    #[getter]
    fn violations(&self) -> Vec<PyShExValidationEntry> {
        self.entries
            .iter()
            .filter(|e| e.status != "conformant")
            .cloned()
            .collect()
    }

    /// How many triples the validation read, duplicates included.
    ///
    /// ``None`` unless :meth:`Rudof.validate_shex` was called with ``read_triples=True``.
    #[getter]
    fn triples_read(&self) -> Option<u64> {
        self.read.as_ref().map(|r| r.reads)
    }

    /// How many distinct triples the validation read.
    ///
    /// ``None`` unless :meth:`Rudof.validate_shex` was called with ``read_triples=True``.
    #[getter]
    fn distinct_triples_read(&self) -> Option<usize> {
        self.read.as_ref().map(|r| r.graph.len())
    }

    /// Serializes the distinct triples the validation read.
    ///
    /// Args:
    ///     format (RDFFormat, optional): Output format. Defaults to ``RDFFormat.Turtle``.
    ///
    /// Returns:
    ///     str: The serialized triples.
    ///
    /// Raises:
    ///     RudofError: If :meth:`Rudof.validate_shex` was not called with ``read_triples=True``.
    #[pyo3(signature = (format = None))]
    fn serialize_read_triples(&self, format: Option<&PyRDFFormat>) -> Result<String> {
        let read = self.read.as_ref().ok_or(CoreError::Generic {
            error: "validate_shex was not called with read_triples=True".into(),
        })?;
        let format = RDFFormat::try_from(format.map(DataFormat::from).unwrap_or(DataFormat::Turtle))?;
        let mut buffer = Vec::new();
        read.graph
            .serialize(&format, &mut buffer)
            .map_err(|e| CoreError::Generic { error: e.to_string() })?;
        Ok(String::from_utf8_lossy(&buffer).into_owned())
    }

    fn __len__(&self) -> usize {
        self.entries.len()
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyShExValidationEntries {
        PyShExValidationEntries {
            inner: slf.entries.clone().into_iter(),
        }
    }

    fn __bool__(&self) -> bool {
        self.conforms()
    }

    fn __repr__(&self) -> String {
        format!(
            "ShExValidationReport(conforms={}, entries={})",
            if self.conforms() { "True" } else { "False" },
            self.entries.len()
        )
    }
}

/// Iterator over a :class:`ShExValidationReport`'s entries.
#[cfg_attr(feature = "stub-gen", pyo3_stub_gen_derive::gen_stub_pyclass)]
#[pyclass(name = "ShExValidationEntryIterator", module = "pyrudof._pyrudof")]
pub struct PyShExValidationEntries {
    inner: std::vec::IntoIter<PyShExValidationEntry>,
}

#[cfg_attr(feature = "stub-gen", pyo3_stub_gen_derive::gen_stub_pymethods)]
#[cfg_attr(not(feature = "stub-gen"), pyo3_stub_gen_derive::remove_gen_stub)]
#[pymethods]
impl PyShExValidationEntries {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    // `None` is how pyo3 signals StopIteration; it is never yielded to Python, so
    // the stub must not widen the element type to `Entry | None`.
    #[gen_stub(override_return_type(type_repr = "ShExValidationEntry"))]
    fn __next__(&mut self) -> Option<PyShExValidationEntry> {
        self.inner.next()
    }

    fn __length_hint__(&self) -> usize {
        self.inner.len()
    }
}
