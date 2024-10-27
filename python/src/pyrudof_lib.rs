//! This is a wrapper of the methods provided by `rudof_lib`
//!
use std::{ffi::OsStr, fs::File, io::BufReader, path::Path};

use pyo3::{exceptions::PyValueError, pyclass, pymethods, PyErr, PyResult, Python};
use rudof_lib::{
    QueryShapeMap, ReaderMode, ResultShapeMap, Rudof, RudofConfig, RudofError, ShExFormat,
    ShExSchema, ShaclFormat, ShaclSchema, ShaclValidationMode, ShapesGraphSource,
    UmlGenerationMode, ValidationReport, ValidationStatus,
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
        shex_schema.map(|s| PyShExSchema { inner: s.clone() })
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
}

#[pyclass(name = "PyUmlGenerationMode")]
pub struct PyUmlGenerationMode {
    inner: UmlGenerationMode,
}

#[pyclass(name = "ShExSchema")]
pub struct PyShExSchema {
    inner: ShExSchema,
}

impl PyShExSchema {
    pub fn serialize(&self, _format: &ShExFormat) -> String {
        let result = &self.inner;
        format!("{result:?}")
    }
}

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
    fn __init__(py: Python<'_>) -> PyResult<Self> {
        py.allow_threads(|| {
            Ok(Self {
                inner: ShaclValidationMode::default(),
            })
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
    fn __init__(py: Python<'_>) -> PyResult<Self> {
        py.allow_threads(|| {
            Ok(Self {
                inner: ShapesGraphSource::default(),
            })
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
