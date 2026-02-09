//! Wrapper for the methods provided by `rudof_lib`
//!
//! Provides Python bindings for `RudofConfig`, allowing creation, loading from a file,
//! and safe usage in Python code.

use std::path::Path;

use pyo3::{PyErr, PyResult, Python, pyclass, pymethods};
use rudof_lib::{RudofConfig, RudofError};

use crate::PyRudofError;

/// Contains the configuration parameters for Rudof.
/// 
/// It can be:
/// * Created with default values.
/// * Loaded from a configuration file.
/// * Used to create a new `Rudof` instance.
/// * Used to update the configuration of an existing `Rudof` instance.
#[pyclass(frozen, name = "RudofConfig")]
pub struct PyRudofConfig {
    /// The internal Rust `RudofConfig` object.
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

     /// Loads a RudofConfig from a file path.
    ///
    /// Args:
    ///     path (str): Path to the configuration file.
    ///
    /// Returns:
    ///     PyRudofConfig: A configuration object initialized from the file.
    ///
    /// Raises:
    ///     RudofError: If the file cannot be read or parsed.
    #[staticmethod]
    pub fn from_path(path: &str) -> PyResult<Self> {
        let path = Path::new(path);
        let rudof_config = RudofConfig::from_path(path).map_err(cnv_err)?;
        Ok(PyRudofConfig {
            inner: rudof_config,
        })
    }
}

/// Converts a `RudofError` into a Python exception.
///
/// Args:
///     e (RudofError): The Rust error to convert.
///
/// Returns:
///     PyErr: A Python exception corresponding to the Rust error.
fn cnv_err(e: RudofError) -> PyErr {
    let e: PyRudofError = e.into();
    let e: PyErr = e.into();
    e
}
