//! Python bindings for the Rudof prefix map utilities.
//!
//! This module provides Python wrappers for managing RDF prefix mappings
//! using the `PrefixMap` type from `rudof_lib`.

use crate::cnv_err;
use pyo3::{PyResult, pyclass, pymethods};
use rudof_lib::{IriS, PrefixMap, RudofError};

/// Python wrapper for `PrefixMap` from `rudof_lib`.
///
/// Provides methods for creating and manipulating prefix-to-IRI mappings.
#[pyclass(name = "PrefixMap")]
pub struct PyPrefixMap {
    /// The internal Rust `PrefixMap` object.
    inner: PrefixMap,
}

#[pymethods]
impl PyPrefixMap {
    /// Convert the prefix map to a string representation.
    ///
    /// Returns:
    ///     str: String representation of the prefix map.
    pub fn show(&self) -> String {
        let result = &self.inner;
        format!("{result}")
    }

    /// Add a prefix mapping.
    ///
    /// Args:
    ///     prefix (str): The prefix to add.
    ///     iri (str): The IRI associated with the prefix.
    ///
    /// Raises:
    ///     ValueError: If the IRI is invalid or the prefix cannot be added.
    pub fn add_prefix(&mut self, prefix: &str, iri: &str) -> PyResult<()> {
        let iri = IriS::new(iri)
            .map_err(|e| RudofError::PrefixMapError { error: e.to_string() })
            .map_err(cnv_err)?;

        self.inner
            .add_prefix(prefix, iri)
            .map_err(|e| RudofError::PrefixMapError { error: e.to_string() })
            .map_err(cnv_err)?;
        Ok(())
    }
}
