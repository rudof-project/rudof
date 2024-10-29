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

    /// Read an RudofConfig from a path
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
        dctap.map(|s| PyDCTAP { _inner: s.clone() })
    }

    /// Obtains the current ShEx Schema
    #[pyo3(signature = ())]
    pub fn get_shex(&self) -> Option<PyShExSchema> {
        let shex_schema = self.inner.get_shex();
        shex_schema.map(|s| PyShExSchema { _inner: s.clone() })
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
    #[pyo3(signature = (input, format))]
    pub fn read_dctap_str(&mut self, input: &str, format: &PyDCTapFormat) -> PyResult<()> {
        self.inner.reset_dctap();
        self.inner
            .read_dctap(input.as_bytes(), &format.inner)
            .map_err(cnv_err)?;
        Ok(())
    }

    /// Reads DCTAP from a path
    #[pyo3(signature = (path_name, format))]
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
        self.inner
            .read_dctap(reader, &format.inner)
            .map_err(cnv_err)?;
        Ok(())
    }

    /// Reads a ShEx schema from a string
    #[pyo3(signature = (input, format))]
    pub fn read_shex_str(&mut self, input: &str, format: &PyShExFormat) -> PyResult<()> {
        self.inner.reset_shex();
        self.inner
            .read_shex(input.as_bytes(), None, &format.inner)
            .map_err(cnv_err)?;
        Ok(())
    }

    /// Reads a ShEx schema from a path
    #[pyo3(signature = (path_name, format))]
    pub fn read_shex_path(&mut self, path_name: &str, format: &PyShExFormat) -> PyResult<()> {
        let path = Path::new(path_name);
        let file = File::open::<&OsStr>(path.as_ref())
            .map_err(|e| RudofError::ReadingShExPath {
                path: path_name.to_string(),
                error: format!("{e}"),
            })
            .map_err(cnv_err)?;
        let reader = BufReader::new(file);
        self.inner.reset_shex();
        self.inner
            .read_shex(reader, None, &format.inner)
            .map_err(cnv_err)?;
        Ok(())
    }

    /// Resets the current ShEx validation results
    #[pyo3(signature = ())]
    pub fn reset_validation_results(&mut self) {
        self.inner.reset_validation_results();
    }

    /// Adds RDF data read from a String to the current RDF Data
    #[pyo3(signature = (input, format, base, reader_mode))]
    pub fn read_data_str(
        &mut self,
        input: &str,
        format: &PyRDFFormat,
        base: Option<&str>,
        reader_mode: &PyReaderMode,
    ) -> PyResult<()> {
        self.inner
            .read_data(input.as_bytes(), &format.inner, base, &reader_mode.inner)
            .map_err(cnv_err)?;
        Ok(())
    }

    /// Reads the current Shapemap from a String
    #[pyo3(signature = (input,format))]
    pub fn read_shapemap_str(&mut self, input: &str, format: &PyShapeMapFormat) -> PyResult<()> {
        self.inner
            .read_shapemap(input.as_bytes(), &format.inner)
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
    #[pyo3(signature = (mode,shapes_graph_source))]
    pub fn validate_shacl(
        &mut self,
        mode: &PyShaclValidationMode,
        shapes_graph_source: &PyShapesGraphSource,
    ) -> PyResult<PyValidationReport> {
        let result = self
            .inner
            .validate_shacl(&mode.inner, &shapes_graph_source.inner)
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
            .shex2plant_uml(&uml_mode.inner, &mut v)
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
    #[pyo3(signature = (format, formatter))]
    pub fn serialize_shex(
        &self,
        format: &PyShExFormat,
        formatter: &PyShExFormatter,
    ) -> PyResult<String> {
        let mut v = Vec::new();
        self.inner
            .serialize_shex(&format.inner, &formatter.inner, &mut v)
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
    #[pyo3(signature = (format))]
    pub fn serialize_shacl(&self, format: &PyShaclFormat) -> PyResult<String> {
        let mut v = Vec::new();
        self.inner
            .serialize_shacl(&format.inner, &mut v)
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
    #[pyo3(signature = (format, formatter))]
    pub fn serialize_shapemap(
        &self,
        format: &PyShapeMapFormat,
        formatter: &PyShapeMapFormatter,
    ) -> PyResult<String> {
        let mut v = Vec::new();
        self.inner
            .serialize_shapemap(&format.inner, &formatter.inner, &mut v)
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

#[pyclass(frozen, name = "ReaderMode")]
pub struct PyReaderMode {
    inner: ReaderMode,
}

#[pymethods]
impl PyReaderMode {
    #[new]
    pub fn __init__(py: Python<'_>) -> Self {
        py.allow_threads(|| Self {
            inner: ReaderMode::default(),
        })
    }

    /// Returns `lax` reader mode
    #[staticmethod]
    pub fn lax() -> Self {
        Self {
            inner: ReaderMode::Lax,
        }
    }

    /// Returns `strict` reader mode
    #[staticmethod]
    pub fn strict() -> Self {
        Self {
            inner: ReaderMode::Strict,
        }
    }
}

#[pyclass(frozen, name = "RDFFormat")]
pub struct PyRDFFormat {
    inner: RDFFormat,
}

#[pymethods]
impl PyRDFFormat {
    #[new]
    pub fn __init__(py: Python<'_>) -> Self {
        py.allow_threads(|| Self {
            inner: RDFFormat::default(),
        })
    }

    /// Returns `Turtle` format
    #[staticmethod]
    pub fn csv() -> Self {
        Self {
            inner: RDFFormat::Turtle,
        }
    }

    /// Returns `RDFXML` format
    #[staticmethod]
    pub fn xlsx() -> Self {
        Self {
            inner: RDFFormat::RDFXML,
        }
    }

    /// Returns `NTriples` format
    #[staticmethod]
    pub fn xls() -> Self {
        Self {
            inner: RDFFormat::NTriples,
        }
    }

    /// Returns `N3` format
    #[staticmethod]
    pub fn n3() -> Self {
        Self {
            inner: RDFFormat::N3,
        }
    }

    /// Returns `NQuads` format
    #[staticmethod]
    pub fn nquads() -> Self {
        Self {
            inner: RDFFormat::NQuads,
        }
    }

    /// Returns `TriG` format
    #[staticmethod]
    pub fn trig() -> Self {
        Self {
            inner: RDFFormat::TriG,
        }
    }
}

#[pyclass(frozen, name = "DCTAPFormat")]
pub struct PyDCTapFormat {
    inner: DCTAPFormat,
}

#[pymethods]
impl PyDCTapFormat {
    #[new]
    pub fn __init__(py: Python<'_>) -> Self {
        py.allow_threads(|| Self {
            inner: DCTAPFormat::default(),
        })
    }

    /// Returns `CSV` format
    #[staticmethod]
    pub fn csv() -> Self {
        Self {
            inner: DCTAPFormat::CSV,
        }
    }

    /// Returns `XLSX` format
    #[staticmethod]
    pub fn xlsx() -> Self {
        Self {
            inner: DCTAPFormat::XLSX,
        }
    }

    /// Returns `XLS` format
    #[staticmethod]
    pub fn xls() -> Self {
        Self {
            inner: DCTAPFormat::XLS,
        }
    }
}

#[pyclass(frozen, name = "ShapeMapFormat")]
pub struct PyShapeMapFormat {
    inner: ShapeMapFormat,
}

#[pymethods]
impl PyShapeMapFormat {
    #[new]
    pub fn __init__(py: Python<'_>) -> Self {
        py.allow_threads(|| Self {
            inner: ShapeMapFormat::default(),
        })
    }

    /// Returns `Compact` format
    #[staticmethod]
    pub fn compact() -> Self {
        Self {
            inner: ShapeMapFormat::Compact,
        }
    }

    /// Returns `JSON` format
    #[staticmethod]
    pub fn json() -> Self {
        Self {
            inner: ShapeMapFormat::JSON,
        }
    }
}

#[pyclass(frozen, name = "ShExFormat")]
pub struct PyShExFormat {
    inner: ShExFormat,
}

#[pymethods]
impl PyShExFormat {
    #[new]
    pub fn __init__(py: Python<'_>) -> Self {
        py.allow_threads(|| Self {
            inner: ShExFormat::default(),
        })
    }

    /// Returns `Turtle` format
    #[staticmethod]
    pub fn turtle() -> Self {
        Self {
            inner: ShExFormat::Turtle,
        }
    }

    /// Returns `ShExC` format
    #[staticmethod]
    pub fn shexc() -> Self {
        Self {
            inner: ShExFormat::ShExC,
        }
    }

    /// Returns `ShExJ` format
    #[staticmethod]
    pub fn shexj() -> Self {
        Self {
            inner: ShExFormat::ShExJ,
        }
    }
}

#[pyclass(frozen, name = "ShaclFormat")]
pub struct PyShaclFormat {
    inner: ShaclFormat,
}

#[pymethods]
impl PyShaclFormat {
    #[new]
    pub fn __init__(py: Python<'_>) -> Self {
        py.allow_threads(|| Self {
            inner: ShaclFormat::default(),
        })
    }

    /// Returns `Turtle` format
    #[staticmethod]
    pub fn turtle() -> Self {
        Self {
            inner: ShaclFormat::Turtle,
        }
    }

    /// Returns `N-Triples` format
    #[staticmethod]
    pub fn ntriples() -> Self {
        Self {
            inner: ShaclFormat::NTriples,
        }
    }

    /// Returns `RDFXML` format
    #[staticmethod]
    pub fn rdfxml() -> Self {
        Self {
            inner: ShaclFormat::RDFXML,
        }
    }

    // TODO...add more constructors...
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

#[pyclass(frozen, name = "UmlGenerationMode")]
pub struct PyUmlGenerationMode {
    inner: UmlGenerationMode,
}

#[pymethods]
impl PyUmlGenerationMode {
    #[new]
    pub fn __init__(py: Python<'_>) -> Self {
        py.allow_threads(|| Self {
            inner: UmlGenerationMode::all(),
        })
    }

    #[staticmethod]
    pub fn all() -> Self {
        Self {
            inner: UmlGenerationMode::all(),
        }
    }

    #[staticmethod]
    pub fn neighs(node: &str) -> Self {
        Self {
            inner: UmlGenerationMode::neighs(node),
        }
    }
}

#[pyclass(name = "ShExSchema")]
pub struct PyShExSchema {
    _inner: ShExSchema,
}

impl PyShExSchema {}

#[pyclass(name = "DCTAP")]
pub struct PyDCTAP {
    _inner: DCTAP,
}

impl PyDCTAP {}

#[pyclass(name = "QueryShapeMap")]
pub struct PyQueryShapeMap {
    inner: QueryShapeMap,
}

impl PyQueryShapeMap {
    pub fn serialize(&self, _format: &ShExFormat) -> String {
        let result = &self.inner;
        format!("{result:?}")
    }
}

#[pyclass(name = "ShaclSchema")]
pub struct PyShaclSchema {
    inner: ShaclSchema,
}

impl PyShaclSchema {
    pub fn serialize(&self, _format: &ShaclFormat) -> String {
        let result = &self.inner;
        format!("{result:?}")
    }
}

#[pyclass(name = "ShaclValidationMode")]
pub struct PyShaclValidationMode {
    inner: ShaclValidationMode,
}

#[pymethods]
impl PyShaclValidationMode {
    #[new]
    pub fn __init__(py: Python<'_>) -> Self {
        py.allow_threads(|| Self {
            inner: ShaclValidationMode::default(),
        })
    }
}

#[pyclass(name = "ShapesGraphSource")]
pub struct PyShapesGraphSource {
    inner: ShapesGraphSource,
}

#[pymethods]
impl PyShapesGraphSource {
    #[new]
    pub fn __init__(py: Python<'_>) -> Self {
        py.allow_threads(|| Self {
            inner: ShapesGraphSource::default(),
        })
    }
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

#[pyclass]
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
