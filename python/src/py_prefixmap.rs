use crate::cnv_err;
use pyo3::{PyResult, pyclass, pymethods};
use rudof_lib::{PrefixMap, RudofError};

/// PrefixMap
#[pyclass(name = "PrefixMap")]
pub struct PyPrefixMap {
    inner: PrefixMap,
}

#[pymethods]
impl PyPrefixMap {
    /// Convert PrefixMap to a String
    pub fn show(&self) -> String {
        let result = &self.inner;
        format!("{result}")
    }

    pub fn add_prefix(&mut self, prefix: &str, iri: &str) -> PyResult<()> {
        self.inner
            .add_prefix(prefix.to_string(), iri.to_string())
            .map_err(|e| RudofError::PrefixMapError {
                error: e.to_string(),
            })
            .map_err(cnv_err)?;
        Ok(())
    }
}
