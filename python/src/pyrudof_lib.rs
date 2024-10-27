//! This is a wrapper of the methods provided by `rudof_lib`
//!
use pyo3::{exceptions::PyValueError, pyclass, pymethods, PyErr, PyResult, Python};
use rudof_lib::{
    ReaderMode, ResultShapeMap, Rudof, RudofConfig, RudofError, ShaclValidationMode,
    ShapesGraphSource, ValidationReport, ValidationStatus,
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

    fn read_shex_str(&mut self, input: &str) -> PyResult<()> {
        self.inner.reset_shex();
        self.inner
            .read_shex(input.as_bytes(), None, &rudof_lib::ShExFormat::ShExC)
            .map_err(cnv_err)?;
        println!("ShEx loaded successfully");
        Ok(())
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
