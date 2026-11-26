use std::path::PathBuf;

use crate::error::Result;
use pyo3::prelude::*;
use rudof_lib::{RudofConfig, TomlConfig};

#[cfg_attr(feature = "stub-gen", pyo3_stub_gen_derive::gen_stub_pyclass)]
#[pyclass(frozen, name = "RudofConfig", module = "pyrudof._pyrudof")]
pub struct PyRudofConfig {
    pub(crate) inner: RudofConfig,
}

#[cfg_attr(feature = "stub-gen", pyo3_stub_gen_derive::gen_stub_pymethods)]
#[pymethods]
impl PyRudofConfig {
    #[new]
    fn __init__() -> Self {
        PyRudofConfig {
            inner: RudofConfig::new(),
        }
    }

    /// Loads a RudofConfig from a file path.
    #[staticmethod]
    fn from_path(path: PathBuf) -> Result<Self> {
        Ok(Self {
            inner: RudofConfig::from_path(&path)?,
        })
    }

    fn __repr__(&self) -> String {
        "RudofConfig()".to_string()
    }
}
