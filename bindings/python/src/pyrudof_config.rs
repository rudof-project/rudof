//! Wrapper for the methods provided by `rudof_lib`
//!
//! Provides Python bindings for `RudofConfig`, allowing creation, loading from a file,
//! and safe usage in Python code.

use std::path::Path;

use pyo3::{PyErr, PyResult, Python, pyclass, pymethods};
use rudof_lib::{
    RudofConfig,
    errors::{ConfigError, RudofError},
};

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
            let rudof_config = RudofConfig::new();
            Ok(Self { inner: rudof_config })
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
        let rudof_config = RudofConfig::from_path(path).map_err(cnv_config_err)?;
        Ok(PyRudofConfig { inner: rudof_config })
    }

    pub fn __repr__(&self) -> String {
        "RudofConfig()".to_string()
    }
}

/// Convert a `ConfigError` into a Python exception by first turning it
/// into a `RudofError` and delegating to the shared `cnv_err` helper.
fn cnv_config_err(e: ConfigError) -> PyErr {
    let r: RudofError = e.into();
    crate::pyrudof_lib::cnv_err(r)
}
