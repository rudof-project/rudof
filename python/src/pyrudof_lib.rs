//! This is a wrapper of the methods provided by `rudof_lib`
//!
use std::{ffi::OsStr, fs::File, io::BufReader, path::Path};

use pyo3::{exceptions::PyValueError, pyclass, pymethods, PyErr, PyResult, Python};
use rudof_lib::{
    QueryShapeMap, ReaderMode, ResultShapeMap, Rudof, RudofConfig, RudofError, ShExFormat,
    ShExFormatter, ShExSchema, ShaclFormat, ShaclSchema, ShaclValidationMode, ShapeMapFormat,
    ShapeMapFormatter, ShapesGraphSource, UmlGenerationMode, ValidationReport, ValidationStatus,
};

#[pyclass(unsendable, frozen, name = "RudofConfig")]
pub struct PyRudofConfig {
    inner: RudofConfig,
}

#[pymethods]
impl PyRudofConfig {
    #[new]
    fn __init__(py: Python<'_>) -> PyResult<Self> {
        py.allow_threads(|| {
            Ok(Self {
                inner: RudofConfig::default(),
            })
        })
    }

    #[staticmethod]
    fn from_path(path: &str) -> PyResult<Self> {
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
    fn __init__(config: &PyRudofConfig) -> PyResult<Self> {
        Ok(Self {
            inner: Rudof::new(&config.inner),
        })
    }

    /// Reset data
    fn reset_data(&mut self) {
        self.inner.reset_data();
    }

    fn reset_shex(&mut self) {
        self.inner.reset_shex();
    }

    fn reset_shapemap(&mut self) {
        self.inner.reset_shapemap();
    }

    fn get_shex(&self) -> Option<PyShExSchema> {
        let shex_schema = self.inner.get_shex();
        shex_schema.map(|s| PyShExSchema { _inner: s.clone() })
    }

    fn get_shapemap(&self) -> Option<PyQueryShapeMap> {
        let shapemap = self.inner.get_shapemap();
        shapemap.map(|s| PyQueryShapeMap { inner: s.clone() })
    }

    fn get_shacl(&self) -> Option<PyShaclSchema> {
        let shacl_schema = self.inner.get_shacl();
        shacl_schema.map(|s| PyShaclSchema { inner: s.clone() })
    }

    /// Reads a ShEx schema from a string
    fn read_shex_str(&mut self, input: &str) -> PyResult<()> {
        self.inner.reset_shex();
        self.inner
            .read_shex(input.as_bytes(), None, &rudof_lib::ShExFormat::ShExC)
            .map_err(cnv_err)?;
        Ok(())
    }

    /// Reads a ShEx schema from a path
    fn read_shex_path(&mut self, path_name: &str) -> PyResult<()> {
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
            .read_shex(reader, None, &rudof_lib::ShExFormat::ShExC)
            .map_err(cnv_err)?;
        Ok(())
    }

    /// Resets the current ShEx validation results
    fn reset_validation_results(&mut self) {
        self.inner.reset_validation_results();
    }

    fn read_data_str(&mut self, input: &str) -> PyResult<()> {
        self.inner
            .read_data(
                input.as_bytes(),
                &rudof_lib::RDFFormat::Turtle,
                None,
                &ReaderMode::Lax,
            )
            .map_err(cnv_err)?;
        Ok(())
    }

    fn read_shapemap_str(&mut self, input: &str) -> PyResult<()> {
        self.inner
            .read_shapemap(input.as_bytes(), &rudof_lib::ShapeMapFormat::Compact)
            .map_err(cnv_err)?;
        Ok(())
    }

    fn validate_shex(&mut self) -> PyResult<PyResultShapeMap> {
        let result = self.inner.validate_shex().map_err(cnv_err)?;
        Ok(PyResultShapeMap { inner: result })
    }

    fn validate_shacl(
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

    fn shex2plantuml(&self, uml_mode: &PyUmlGenerationMode) -> PyResult<String> {
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

    /// Serialize the current ShEx
    fn serialize_shex(
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

    /// Serialize the current SHACL
    fn serialize_shacl(&self, format: &PyShaclFormat) -> PyResult<String> {
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
    fn serialize_shapemap(
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
}

#[pyclass(frozen, name = "ShapeMapFormat")]
pub struct PyShapeMapFormat {
    inner: ShapeMapFormat,
}

#[pymethods]
impl PyShapeMapFormat {
    #[new]
    fn __init__(py: Python<'_>) -> Self {
        py.allow_threads(|| Self {
            inner: ShapeMapFormat::default(),
        })
    }

    /// Returns `Compact` format
    #[staticmethod]
    fn compact() -> Self {
        Self {
            inner: ShapeMapFormat::Compact,
        }
    }

    /// Returns `JSON` format
    #[staticmethod]
    fn json() -> Self {
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
    fn __init__(py: Python<'_>) -> Self {
        py.allow_threads(|| Self {
            inner: ShExFormat::default(),
        })
    }

    /// Returns `Turtle` format
    #[staticmethod]
    fn turtle() -> Self {
        Self {
            inner: ShExFormat::Turtle,
        }
    }

    /// Returns `ShExC` format
    #[staticmethod]
    fn shexc() -> Self {
        Self {
            inner: ShExFormat::ShExC,
        }
    }

    /// Returns `ShExJ` format
    #[staticmethod]
    fn shexj() -> Self {
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
    fn __init__(py: Python<'_>) -> Self {
        py.allow_threads(|| Self {
            inner: ShaclFormat::default(),
        })
    }

    /// Returns `Turtle` format
    #[staticmethod]
    fn turtle() -> Self {
        Self {
            inner: ShaclFormat::Turtle,
        }
    }

    /// Returns `N-Triples` format
    #[staticmethod]
    fn ntriples() -> Self {
        Self {
            inner: ShaclFormat::NTriples,
        }
    }

    /// Returns `RDFXML` format
    #[staticmethod]
    fn rdfxml() -> Self {
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
    fn __init__(py: Python<'_>) -> Self {
        py.allow_threads(|| Self {
            inner: ShExFormatter::default(),
        })
    }

    /// Returns a ShExFormatter that doesn't print terminal colors
    #[staticmethod]
    fn without_colors() -> Self {
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
    fn __init__(py: Python<'_>) -> Self {
        py.allow_threads(|| Self {
            inner: ShapeMapFormatter::default(),
        })
    }

    /// Returns a Shapemap formatter that doesn't print terminal colors
    #[staticmethod]
    fn without_colors() -> Self {
        Self {
            inner: ShapeMapFormatter::default().without_colors(),
        }
    }
}

#[pyclass(frozen, name = "PyUmlGenerationMode")]
pub struct PyUmlGenerationMode {
    inner: UmlGenerationMode,
}

#[pymethods]
impl PyUmlGenerationMode {
    #[new]
    fn __init__(py: Python<'_>) -> Self {
        py.allow_threads(|| Self {
            inner: UmlGenerationMode::all(),
        })
    }

    #[staticmethod]
    fn all() -> Self {
        Self {
            inner: UmlGenerationMode::all(),
        }
    }

    #[staticmethod]
    fn neighs(node: &str) -> Self {
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

#[pyclass(name = "PyQueryShapeMap")]
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
    fn __init__(py: Python<'_>) -> Self {
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
    fn __init__(py: Python<'_>) -> Self {
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