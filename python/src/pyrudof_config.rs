//! This is a wrapper of the methods provided by `rudof_lib`
//!
use std::path::Path;

use pyo3::{PyErr, PyResult, Python, pyclass, pymethods};
use rudof_lib::{RudofConfig, RudofError};

use crate::PyRudofError;

/// Contains the Rudof configuration parameters
/// It can be created with default values or read from a file
/// It can be used to create a `Rudof` instance
/// It is immutable
/// It can be used to update the configuration of an existing `Rudof` instance
/// It can be used to create a new `Rudof` instance with the same configuration
/// It is thread safe
#[pyclass(frozen, name = "RudofConfig")]
pub struct PyRudofConfig {
    pub inner: RudofConfig,
}

#[pymethods]
impl PyRudofConfig {
    #[new]
    pub fn __init__(py: Python<'_>) -> PyResult<Self> {
        py.detach(|| {
            let rudof_config = RudofConfig::new().map_err(cnv_err)?;
            Ok(Self {
                inner: rudof_config,
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

fn cnv_err(e: RudofError) -> PyErr {
    println!("RudofConfigError: {e}");
    let e: PyRudofError = e.into();
    let e: PyErr = e.into();
    e
}
