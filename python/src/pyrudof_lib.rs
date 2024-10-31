//! This is a wrapper of the methods provided by `rudof_lib`
//!
use pyo3::{exceptions::PyValueError, pyclass, pymethods, PyErr, PyResult, Python};
use rudof_lib::{
    iri, DCTAPFormat, QueryShapeMap, RDFFormat, ReaderMode, ResultShapeMap, Rudof, RudofConfig,
    RudofError, ShExFormat, ShExFormatter, ShExSchema, ShaclFormat, ShaclSchema,
    ShaclValidationMode, ShapeMapFormat, ShapeMapFormatter, ShapesGraphSource, UmlGenerationMode,
    ValidationReport, ValidationStatus, DCTAP,
};
use std::{ffi::OsStr, fs::File, io::BufReader, path::Path};

#[pyclass(frozen, name = "RudofConfig")]
pub struct PyRudofConfig {
    inner: RudofConfig,
}

#[pymethods]
impl PyRudofConfig {
    #[new]
    pub fn __init__(py: Python<'_>) -> PyResult<Self> {
        py.allow_threads(|| {
            Ok(Self {
                inner: RudofConfig::default(),
            })
        })
    }

    /// Read an `RudofConfig` from a file path
    #[staticmethod]
    #[pyo3(signature = (path))]
    pub fn from_path(path: &str) -> PyResult<Self> {
        let path = Path::new(path);
        let rudof_config = RudofConfig::from_path(path).map_err(cnv_err)?;
        Ok(PyRudofConfig {
            inner: rudof_config,
        })
    }
}

/// Main class to handle `rudof` features.
/// It is currently `unsendable` and doesn't support multiple threads.
/// There should  be only one instance of `rudof` per program.
///
// TODO: review the unsendable constraint and check if we can remove it in the future
#[pyclass(unsendable, name = "Rudof")]
pub struct PyRudof {
    inner: Rudof,
}

#[pymethods]
impl PyRudof {
    #[new]
    pub fn __init__(config: &PyRudofConfig) -> PyResult<Self> {
        Ok(Self {
            inner: Rudof::new(&config.inner),
        })
    }

    pub fn update_config(&mut self, config: &PyRudofConfig) {
        self.inner.update_config(&config.inner)
    }

    /// Obtain the version of the Rudof library
    #[pyo3(signature = ())]
    pub fn version(&self) -> PyResult<String> {
        let str = env!("CARGO_PKG_VERSION").to_string();
        Ok(str)
    }

    /// Resets the current RDF data
    #[pyo3(signature = ())]
    pub fn reset_data(&mut self) {
        self.inner.reset_data();
    }

    /// Resets the current ShEx schema
    #[pyo3(signature = ())]
    pub fn reset_shex(&mut self) {
        self.inner.reset_shex();
    }

    /// Resets the current shapemap
    #[pyo3(signature = ())]
    pub fn reset_shapemap(&mut self) {
        self.inner.reset_shapemap();
    }

    /// Resets the current SHACL shapes graph
    #[pyo3(signature = ())]
    pub fn reset_shacl(&mut self) {
        self.inner.reset_shacl();
    }

    /// Resets all current values
    #[pyo3(signature = ())]
    pub fn reset_all(&mut self) {
        self.inner.reset_all()
    }

    /// Obtains the current DCTAP
    #[pyo3(signature = ())]
    pub fn get_dctap(&self) -> Option<PyDCTAP> {
        let dctap = self.inner.get_dctap();
        dctap.map(|s| PyDCTAP { inner: s.clone() })
    }

    /// Obtains the current ShEx Schema
    #[pyo3(signature = ())]
    pub fn get_shex(&self) -> Option<PyShExSchema> {
        let shex_schema = self.inner.get_shex();
        shex_schema.map(|s| PyShExSchema { inner: s.clone() })
    }

    /// Obtains the current Shapemap
    #[pyo3(signature = ())]
    pub fn get_shapemap(&self) -> Option<PyQueryShapeMap> {
        let shapemap = self.inner.get_shapemap();
        shapemap.map(|s| PyQueryShapeMap { inner: s.clone() })
    }

    /// Obtains the current SHACL schema
    #[pyo3(signature = ())]
    pub fn get_shacl(&self) -> Option<PyShaclSchema> {
        let shacl_schema = self.inner.get_shacl();
        shacl_schema.map(|s| PyShaclSchema { inner: s.clone() })
    }

    /// Reads DCTAP from a String
    #[pyo3(signature = (input, format = &PyDCTapFormat::CSV))]
    pub fn read_dctap_str(&mut self, input: &str, format: &PyDCTapFormat) -> PyResult<()> {
        self.inner.reset_dctap();
        let format = cnv_dctap_format(format);
        self.inner
            .read_dctap(input.as_bytes(), &format)
            .map_err(cnv_err)?;
        Ok(())
    }

    /// Reads DCTAP from a path
    #[pyo3(signature = (path_name, format = &PyDCTapFormat::CSV))]
    pub fn read_dctap_path(&mut self, path_name: &str, format: &PyDCTapFormat) -> PyResult<()> {
        let path = Path::new(path_name);
        let file = File::open::<&OsStr>(path.as_ref())
            .map_err(|e| RudofError::ReadingDCTAPPath {
                path: path_name.to_string(),
                error: format!("{e}"),
            })
            .map_err(cnv_err)?;
        let reader = BufReader::new(file);
        self.inner.reset_dctap();
        let format = cnv_dctap_format(format);
        self.inner.read_dctap(reader, &format).map_err(cnv_err)?;
        Ok(())
    }

    /// Reads a ShEx schema from a string
    #[pyo3(signature = (input, format = &PyShExFormat::ShExC, base = None))]
    pub fn read_shex_str(
        &mut self,
        input: &str,
        format: &PyShExFormat,
        base: Option<&str>,
    ) -> PyResult<()> {
        let format = cnv_shex_format(format);
        self.inner.reset_shex();
        self.inner
            .read_shex(input.as_bytes(), &format, base)
            .map_err(cnv_err)?;
        Ok(())
    }

    /// Reads a SHACL shapes graph from a string
    #[pyo3(signature = (input, format = &PyShaclFormat::Turtle, base = None, reader_mode = &PyReaderMode::Lax))]
    pub fn read_shacl_str(
        &mut self,
        input: &str,
        format: &PyShaclFormat,
        base: Option<&str>,
        reader_mode: &PyReaderMode,
    ) -> PyResult<()> {
        let format = cnv_shacl_format(format);
        let reader_mode = cnv_reader_mode(reader_mode);
        self.inner.reset_shacl();
        self.inner
            .read_shacl(input.as_bytes(), &format, base, &reader_mode)
            .map_err(cnv_err)?;
        Ok(())
    }

    /// Reads a ShEx schema from a path
    #[pyo3(signature = (path_name, format = &PyShExFormat::ShExC, base = None))]
    pub fn read_shex_path(
        &mut self,
        path_name: &str,
        format: &PyShExFormat,
        base: Option<&str>,
    ) -> PyResult<()> {
        let path = Path::new(path_name);
        let file = File::open::<&OsStr>(path.as_ref())
            .map_err(|e| RudofError::ReadingShExPath {
                path: path_name.to_string(),
                error: format!("{e}"),
            })
            .map_err(cnv_err)?;
        let reader = BufReader::new(file);
        self.inner.reset_shex();
        let format = cnv_shex_format(format);
        self.inner
            .read_shex(reader, &format, base)
            .map_err(cnv_err)?;
        Ok(())
    }

    /// Reads a ShEx schema from a path
    #[pyo3(signature = (path_name, format = &PyShaclFormat::Turtle, base = None, reader_mode = &PyReaderMode::Lax))]
    pub fn read_shacl_path(
        &mut self,
        path_name: &str,
        format: &PyShaclFormat,
        base: Option<&str>,
        reader_mode: &PyReaderMode,
    ) -> PyResult<()> {
        let path = Path::new(path_name);
        let file = File::open::<&OsStr>(path.as_ref())
            .map_err(|e| RudofError::ReadingShExPath {
                path: path_name.to_string(),
                error: format!("{e}"),
            })
            .map_err(cnv_err)?;
        let reader = BufReader::new(file);
        self.inner.reset_shex();
        let format = cnv_shacl_format(format);
        let reader_mode = cnv_reader_mode(reader_mode);
        self.inner
            .read_shacl(reader, &format, base, &reader_mode)
            .map_err(cnv_err)?;
        Ok(())
    }

    /// Resets the current ShEx validation results
    #[pyo3(signature = ())]
    pub fn reset_validation_results(&mut self) {
        self.inner.reset_validation_results();
    }

    /// Adds RDF data read from a Path
    #[pyo3(signature = (path_name, format = &PyRDFFormat::Turtle, base = None, reader_mode = &PyReaderMode::Lax))]
    pub fn read_data_path(
        &mut self,
        path_name: &str,
        format: &PyRDFFormat,
        base: Option<&str>,
        reader_mode: &PyReaderMode,
    ) -> PyResult<()> {
        let reader_mode = cnv_reader_mode(reader_mode);
        let format = cnv_rdf_format(format);
        let path = Path::new(path_name);
        let file = File::open::<&OsStr>(path.as_ref())
            .map_err(|e| RudofError::ReadingDCTAPPath {
                path: path_name.to_string(),
                error: format!("{e}"),
            })
            .map_err(cnv_err)?;
        let reader = BufReader::new(file);
        self.inner
            .read_data(reader, &format, base, &reader_mode)
            .map_err(cnv_err)?;
        Ok(())
    }

    /// Adds RDF data read from a String to the current RDF Data
    #[pyo3(signature = (input, format = &PyRDFFormat::Turtle, base = None, reader_mode = &PyReaderMode::Lax))]
    pub fn read_data_str(
        &mut self,
        input: &str,
        format: &PyRDFFormat,
        base: Option<&str>,
        reader_mode: &PyReaderMode,
    ) -> PyResult<()> {
        let reader_mode = cnv_reader_mode(reader_mode);
        let format = cnv_rdf_format(format);
        self.inner
            .read_data(input.as_bytes(), &format, base, &reader_mode)
            .map_err(cnv_err)?;
        Ok(())
    }

    /// Reads the current Shapemap from a String
    #[pyo3(signature = (input,format = &PyShapeMapFormat::Compact))]
    pub fn read_shapemap_str(&mut self, input: &str, format: &PyShapeMapFormat) -> PyResult<()> {
        let format = cnv_shapemap_format(format);
        self.inner
            .read_shapemap(input.as_bytes(), &format)
            .map_err(cnv_err)?;
        Ok(())
    }

    /// Validate the current RDF Data with the current ShEx schema and the current Shapemap
    ///
    /// In order to validate, a ShEx Schema and a ShapeMap has to be read
    #[pyo3(signature = ())]
    pub fn validate_shex(&mut self) -> PyResult<PyResultShapeMap> {
        let result = self.inner.validate_shex().map_err(cnv_err)?;
        Ok(PyResultShapeMap { inner: result })
    }

    /// Validates the current RDF Data
    ///
    /// mode can be native to use Native implementation or SPARQL to use the SPARQL based implementation
    /// shapes_graph_source: Indicates the source of the shapes graph,
    /// which can be extracted from the current RDF data,
    /// or from the current SHACL schema.
    /// If there is no current SHACL schema, it tries to get it from the current RDF data
    #[pyo3(signature = (mode = &PyShaclValidationMode::Native, shapes_graph_source = &PyShapesGraphSource::CurrentSchema ))]
    pub fn validate_shacl(
        &mut self,
        mode: &PyShaclValidationMode,
        shapes_graph_source: &PyShapesGraphSource,
    ) -> PyResult<PyValidationReport> {
        let mode = cnv_shacl_validation_mode(mode);
        let shapes_graph_source = cnv_shapes_graph_source(shapes_graph_source);
        let result = self
            .inner
            .validate_shacl(&mode, &shapes_graph_source)
            .map_err(cnv_err)?;
        Ok(PyValidationReport { inner: result })
    }

    /// Converts the current DCTAP to ShEx and replaces the current ShEx by the resulting ShEx
    pub fn dctap2shex(&mut self) -> PyResult<()> {
        self.inner.dctap2shex().map_err(cnv_err)
    }

    /// Converts the current ShEx to a Class-like diagram using PlantUML syntax
    #[pyo3(signature = (uml_mode))]
    pub fn shex2plantuml(&self, uml_mode: &PyUmlGenerationMode) -> PyResult<String> {
        let mut v = Vec::new();
        self.inner
            .shex2plant_uml(&uml_mode.into(), &mut v)
            .map_err(|e| RudofError::ShEx2PlantUmlError {
                error: format!("Error generating UML: {e}"),
            })
            .map_err(cnv_err)?;
        let str = String::from_utf8(v)
            .map_err(|e| RudofError::ShEx2PlantUmlError {
                error: format!("ShEx2PlantUML: Error converting generated vector to UML: {e}"),
            })
            .map_err(cnv_err)?;
        Ok(str)
    }

    /// Serialize the current ShEx schema
    #[pyo3(signature = (formatter, format = &PyShExFormat::ShExC))]
    pub fn serialize_shex(
        &self,
        formatter: &PyShExFormatter,
        format: &PyShExFormat,
    ) -> PyResult<String> {
        let mut v = Vec::new();
        let format = cnv_shex_format(format);
        self.inner
            .serialize_shex(&format, &formatter.inner, &mut v)
            .map_err(|e| RudofError::SerializingShEx {
                error: format!("{e}"),
            })
            .map_err(cnv_err)?;
        let str = String::from_utf8(v)
            .map_err(|e| RudofError::SerializingShEx {
                error: format!("{e}"),
            })
            .map_err(cnv_err)?;
        Ok(str)
    }

    /// Serialize the current SHACL shapes graph
    #[pyo3(signature = (format = &PyShaclFormat::Turtle))]
    pub fn serialize_shacl(&self, format: &PyShaclFormat) -> PyResult<String> {
        let mut v = Vec::new();
        let format = cnv_shacl_format(format);
        self.inner
            .serialize_shacl(&format, &mut v)
            .map_err(|e| RudofError::SerializingShacl {
                error: format!("{e}"),
            })
            .map_err(cnv_err)?;
        let str = String::from_utf8(v)
            .map_err(|e| RudofError::SerializingShacl {
                error: format!("{e}"),
            })
            .map_err(cnv_err)?;
        Ok(str)
    }

    /// Serialize the current Query Shape Map
    #[pyo3(signature = (formatter, format = &PyShapeMapFormat::Compact))]
    pub fn serialize_shapemap(
        &self,
        formatter: &PyShapeMapFormatter,
        format: &PyShapeMapFormat,
    ) -> PyResult<String> {
        let mut v = Vec::new();
        let format = cnv_shapemap_format(format);
        self.inner
            .serialize_shapemap(&format, &formatter.inner, &mut v)
            .map_err(|e| RudofError::SerializingShacl {
                error: format!("{e}"),
            })
            .map_err(cnv_err)?;
        let str = String::from_utf8(v)
            .map_err(|e| RudofError::SerializingShacl {
                error: format!("{e}"),
            })
            .map_err(cnv_err)?;
        Ok(str)
    }

    /// Adds an endpoint to the current RDF Data
    #[pyo3(signature = (endpoint))]
    pub fn add_endpoint(&mut self, endpoint: &str) -> PyResult<()> {
        let iri = iri!(endpoint);
        self.inner.add_endpoint(&iri).map_err(cnv_err)
    }
}

#[pyclass(eq, eq_int, name = "ReaderMode")]
#[derive(PartialEq)]
pub enum PyReaderMode {
    Lax,
    Strict,
}

#[pymethods]
impl PyReaderMode {
    #[new]
    pub fn __init__(py: Python<'_>) -> Self {
        py.allow_threads(|| PyReaderMode::Lax)
    }

    /// Returns `lax` reader mode
    #[staticmethod]
    pub fn lax() -> Self {
        PyReaderMode::Lax
    }

    /// Returns `strict` reader mode
    #[staticmethod]
    pub fn strict() -> Self {
        PyReaderMode::Strict
    }
}

#[allow(clippy::upper_case_acronyms)]
#[pyclass(eq, eq_int, name = "RDFFormat")]
#[derive(PartialEq)]
pub enum PyRDFFormat {
    Turtle,
    NTriples,
    RDFXML,
    TriG,
    N3,
    NQuads,
}

#[allow(clippy::upper_case_acronyms)]
#[pyclass(eq, eq_int, name = "DCTapFormat")]
#[derive(PartialEq)]
pub enum PyDCTapFormat {
    CSV,
    XLSX,
}

#[allow(clippy::upper_case_acronyms)]
#[pyclass(eq, eq_int, name = "ShapeMapFormat")]
#[derive(PartialEq)]
pub enum PyShapeMapFormat {
    Compact,
    JSON,
}

#[allow(clippy::upper_case_acronyms)]
#[pyclass(eq, eq_int, name = "ShExFormat")]
#[derive(PartialEq)]
pub enum PyShExFormat {
    ShExC,
    ShExJ,
    Turtle,
}

#[allow(clippy::upper_case_acronyms)]
#[pyclass(eq, eq_int, name = "ShaclFormat")]
#[derive(PartialEq)]
pub enum PyShaclFormat {
    Turtle,
    NTriples,
    RDFXML,
    TriG,
    N3,
    NQuads,
}

#[pyclass(frozen, name = "ShExFormatter")]
pub struct PyShExFormatter {
    inner: ShExFormatter,
}

#[pymethods]
impl PyShExFormatter {
    #[new]
    pub fn __init__(py: Python<'_>) -> Self {
        py.allow_threads(|| Self {
            inner: ShExFormatter::default(),
        })
    }

    /// Returns a ShExFormatter that doesn't print terminal colors
    #[staticmethod]
    pub fn without_colors() -> Self {
        Self {
            inner: ShExFormatter::default().without_colors(),
        }
    }
}

/// Defines how to format a ShapeMap
#[pyclass(frozen, name = "ShapeMapFormatter")]
pub struct PyShapeMapFormatter {
    inner: ShapeMapFormatter,
}

#[pymethods]
impl PyShapeMapFormatter {
    #[new]
    pub fn __init__(py: Python<'_>) -> Self {
        py.allow_threads(|| Self {
            inner: ShapeMapFormatter::default(),
        })
    }

    /// Returns a Shapemap formatter that doesn't print terminal colors
    #[staticmethod]
    pub fn without_colors() -> Self {
        Self {
            inner: ShapeMapFormatter::default().without_colors(),
        }
    }
}

#[pyclass(name = "UmlGenerationMode")]
pub enum PyUmlGenerationMode {
    /// Generate UML for all nodes
    #[pyo3(name = "AllNodes")]
    PyAllNodes {},

    /// Generate UML only for the neighbours of a shape
    #[pyo3(constructor = (node), name ="Neighs")]
    PyNeighs { node: String },
}

#[pymethods]
impl PyUmlGenerationMode {
    #[new]
    pub fn __init__(py: Python<'_>) -> Self {
        py.allow_threads(|| PyUmlGenerationMode::PyAllNodes {})
    }

    #[staticmethod]
    pub fn all() -> Self {
        PyUmlGenerationMode::PyAllNodes {}
    }

    #[staticmethod]
    pub fn neighs(node: &str) -> Self {
        PyUmlGenerationMode::PyNeighs {
            node: node.to_string(),
        }
    }
}

impl From<&PyUmlGenerationMode> for UmlGenerationMode {
    fn from(m: &PyUmlGenerationMode) -> UmlGenerationMode {
        match m {
            PyUmlGenerationMode::PyAllNodes {} => UmlGenerationMode::AllNodes,
            PyUmlGenerationMode::PyNeighs { node } => UmlGenerationMode::Neighs(node.to_string()),
        }
    }
}

impl From<UmlGenerationMode> for PyUmlGenerationMode {
    fn from(value: UmlGenerationMode) -> Self {
        match value {
            UmlGenerationMode::AllNodes => PyUmlGenerationMode::PyAllNodes {},
            UmlGenerationMode::Neighs(node) => PyUmlGenerationMode::PyNeighs { node },
        }
    }
}

#[pyclass(name = "ShExSchema")]
pub struct PyShExSchema {
    inner: ShExSchema,
}

#[pymethods]
impl PyShExSchema {
    pub fn __repr__(&self) -> String {
        format!("{}", self.inner)
    }
}

#[pyclass(name = "DCTAP")]
pub struct PyDCTAP {
    inner: DCTAP,
}

#[pymethods]
impl PyDCTAP {
    pub fn __repr__(&self) -> String {
        format!("{}", self.inner)
    }

    pub fn __str__(&self) -> String {
        format!("{}", self.inner)
    }
}

#[pyclass(name = "QueryShapeMap")]
pub struct PyQueryShapeMap {
    inner: QueryShapeMap,
}

#[pymethods]
impl PyQueryShapeMap {
    fn __repr__(&self) -> String {
        format!("{}", self.inner)
    }
}

#[pyclass(name = "ShaclSchema")]
pub struct PyShaclSchema {
    inner: ShaclSchema,
}

#[pymethods]
impl PyShaclSchema {
    pub fn __repr__(&self) -> String {
        format!("{}", self.inner)
    }
}

#[pyclass(eq, eq_int, name = "ShaclValidationMode")]
#[derive(PartialEq)]
pub enum PyShaclValidationMode {
    Native,
    Sparql,
}

#[pyclass(eq, eq_int, name = "ShapesGraphSource")]
#[derive(PartialEq)]
pub enum PyShapesGraphSource {
    CurrentData,
    CurrentSchema,
}

#[pyclass(frozen, name = "ResultShapeMap")]
pub struct PyResultShapeMap {
    inner: ResultShapeMap,
}

#[pymethods]
impl PyResultShapeMap {
    /*fn get_info(&self, node: &str, label: &str) -> Option<PyValidationStatus> {
        let node = Node::from_str(node);
        let label = ShapeLabel::from_str(label);
        let info = self.get_info(node, label);
        info.map(|status| PyValidationStatus { inner: status })
    }*/

    /// Convert a ResultShapeMap to a String
    pub fn show(&self) -> String {
        let result = &self.inner;
        format!("{result}")
    }
}

#[pyclass(frozen, name = "ValidationReport")]
pub struct PyValidationReport {
    inner: ValidationReport,
}

#[pymethods]
impl PyValidationReport {
    /// Convert ValidationReport to a String
    pub fn show(&self) -> String {
        let result = &self.inner;
        format!("{result}")
    }

    /// Returns true if there were no violation errors
    pub fn conforms(&self) -> bool {
        self.inner.conforms()
    }
}

#[pyclass(frozen, name = "ValidationStatus")]
pub struct PyValidationStatus {
    inner: ValidationStatus,
}

#[pymethods]
impl PyValidationStatus {
    /// Convert ValidationStatus to a String
    pub fn show(&self) -> String {
        let result = &self.inner;
        format!("{result}")
    }
}

#[pyclass(name = "RudofError")]
/// Wrapper for `RudofError`
pub struct PyRudofError {
    error: RudofError,
}

impl From<PyRudofError> for PyErr {
    fn from(e: PyRudofError) -> Self {
        PyValueError::new_err(format!("{}", e.error))
    }
}

impl From<RudofError> for PyRudofError {
    fn from(error: RudofError) -> Self {
        Self { error }
    }
}

fn cnv_err(e: RudofError) -> PyErr {
    let e: PyRudofError = e.into();
    let e: PyErr = e.into();
    e
}

fn cnv_dctap_format(format: &PyDCTapFormat) -> DCTAPFormat {
    match format {
        PyDCTapFormat::CSV => DCTAPFormat::CSV,
        PyDCTapFormat::XLSX => DCTAPFormat::XLSX,
    }
}

fn cnv_reader_mode(format: &PyReaderMode) -> ReaderMode {
    match format {
        PyReaderMode::Lax => ReaderMode::Lax,
        PyReaderMode::Strict => ReaderMode::Strict,
    }
}

fn cnv_rdf_format(format: &PyRDFFormat) -> RDFFormat {
    match format {
        PyRDFFormat::Turtle => RDFFormat::Turtle,
        PyRDFFormat::NTriples => RDFFormat::NTriples,
        PyRDFFormat::RDFXML => RDFFormat::RDFXML,
        PyRDFFormat::TriG => RDFFormat::TriG,
        PyRDFFormat::N3 => RDFFormat::N3,
        PyRDFFormat::NQuads => RDFFormat::NQuads,
    }
}

fn cnv_shapemap_format(format: &PyShapeMapFormat) -> ShapeMapFormat {
    match format {
        PyShapeMapFormat::Compact => ShapeMapFormat::Compact,
        PyShapeMapFormat::JSON => ShapeMapFormat::JSON,
    }
}

fn cnv_shex_format(format: &PyShExFormat) -> ShExFormat {
    match format {
        PyShExFormat::ShExC => ShExFormat::ShExC,
        PyShExFormat::ShExJ => ShExFormat::ShExJ,
        PyShExFormat::Turtle => ShExFormat::Turtle,
    }
}

fn cnv_shacl_format(format: &PyShaclFormat) -> ShaclFormat {
    match format {
        PyShaclFormat::Turtle => ShaclFormat::Turtle,
        PyShaclFormat::NTriples => ShaclFormat::NTriples,
        PyShaclFormat::RDFXML => ShaclFormat::RDFXML,
        PyShaclFormat::TriG => ShaclFormat::TriG,
        PyShaclFormat::N3 => ShaclFormat::N3,
        PyShaclFormat::NQuads => ShaclFormat::NQuads,
    }
}

fn cnv_shacl_validation_mode(mode: &PyShaclValidationMode) -> ShaclValidationMode {
    match mode {
        PyShaclValidationMode::Native => ShaclValidationMode::Native,
        PyShaclValidationMode::Sparql => ShaclValidationMode::Sparql,
    }
}

fn cnv_shapes_graph_source(sgs: &PyShapesGraphSource) -> ShapesGraphSource {
    match sgs {
        PyShapesGraphSource::CurrentData => ShapesGraphSource::CurrentData,
        PyShapesGraphSource::CurrentSchema => ShapesGraphSource::CurrentSchema,
    }
}
