use crate::config::PyRudofConfig;
use pyo3::prelude::*;
use rudof_lib::{Rudof, RudofConfig};

/// Main interface for working with Semantic Web operations.
///
/// A stateful session: loaded data, schemas and results persist across calls until a
/// ``reset_*`` method clears them.
#[cfg_attr(feature = "stub-gen", pyo3_stub_gen_derive::gen_stub_pyclass)]
#[pyclass(name = "Rudof", module = "pyrudof._pyrudof")]
pub struct PyRudof {
    pub(crate) inner: Rudof,
}

#[cfg_attr(feature = "stub-gen", pyo3_stub_gen_derive::gen_stub_pymethods)]
#[pymethods]
impl PyRudof {
    /// Creates a new Rudof instance.
    ///
    /// Args:
    ///     config (RudofConfig, optional): Configuration for the instance. Defaults to ``RudofConfig()``.
    #[new]
    #[pyo3(signature = (config = None))]
    fn __init__(config: Option<&PyRudofConfig>) -> Self {
        let config = config
            .map(|c| c.inner.clone())
            .unwrap_or_else(RudofConfig::new);
        Self {
            inner: Rudof::new(config),
        }
    }

    /// Updates the configuration of this Rudof instance.
    ///
    /// Args:
    ///     config (RudofConfig): New configuration to apply.
    ///
    /// Note:
    ///     This does not affect already-loaded data or schemas, only future operations.
    fn update_config(&mut self, config: &PyRudofConfig) {
        self.inner.update_config(config.inner.clone()).execute();
    }

    /// Returns the version of the underlying rudof library.
    fn get_version(&self) -> String {
        self.inner.version().execute().to_string()
    }

    fn __repr__(&self) -> String {
        format!("Rudof(version='{}')", self.inner.version().execute())
    }

    fn __enter__(slf: Py<Self>) -> Py<Self> {
        slf
    }

    #[pyo3(signature = (_exc_type = None, _exc_value = None, _traceback = None))]
    fn __exit__(
        &mut self,
        _exc_type: Option<&Bound<'_, PyAny>>,
        _exc_value: Option<&Bound<'_, PyAny>>,
        _traceback: Option<&Bound<'_, PyAny>>,
    ) -> bool {
        self.inner.reset_all().execute();
        false // never swallow the exception
    }

    /// Resets all current state (data, schemas, queries, validation results).
    ///
    /// Equivalent to calling every individual reset method.
    fn reset_all(&mut self) {
        self.inner.reset_all().execute();
    }

    /// Clears the current RDF data graph.
    ///
    /// Removes all RDF triples from memory. Does not affect loaded schemas or other state.
    fn reset_data(&mut self) {
        self.inner.reset_data().execute();
    }

    /// Resets ShEx validation.
    ///
    /// Unloads the ShEx schema, ShapeMap, compiled validator, and validation results.
    /// Does not affect RDF data or other state. Use :meth:`reset_shex_schema` instead to
    /// unload only the schema, keeping any loaded ShapeMap and validation results.
    fn reset_shex(&mut self) {
        self.inner.reset_shex().execute();
    }

    /// Clears the current ShEx schema only.
    ///
    /// Unlike :meth:`reset_shex`, this leaves any loaded ShapeMap and ShEx validation results untouched.
    fn reset_shex_schema(&mut self) {
        self.inner.reset_shex_schema().execute();
    }

    /// Clears the current SHACL shapes graph only.
    ///
    /// Unloads the SHACL shapes from memory, leaving any existing SHACL validation
    /// results untouched. Use :meth:`reset_shacl_validation` instead to also clear
    /// validation results.
    fn reset_shacl(&mut self) {
        self.inner.reset_shacl_shapes().execute();
    }

    /// Resets SHACL validation.
    ///
    /// Clears SHACL validation results and unloads the currently loaded SHACL shapes
    /// graph. Use :meth:`reset_shacl` instead to unload only the shapes graph, keeping
    /// any validation results.
    fn reset_shacl_validation(&mut self) {
        self.inner.reset_shacl().execute();
    }

    /// Clears the current ShapeMap.
    ///
    /// Removes the ShapeMap used for ShEx validation.
    fn reset_shapemap(&mut self) {
        self.inner.reset_shapemap().execute();
    }

    /// Clears the current SPARQL query.
    ///
    /// Removes the stored query from memory.
    fn reset_query(&mut self) {
        self.inner.reset_sparql_query().execute();
    }

    /// Clears the results of the most recent SPARQL query.
    ///
    /// Leaves the query itself loaded.
    fn reset_query_results(&mut self) {
        self.inner.reset_query_results().execute();
    }

    /// Clears the current DCTAP profile.
    fn reset_dctap(&mut self) {
        self.inner.reset_dctap().execute();
    }

    /// Clears the current rdf-config document.
    fn reset_rdf_config(&mut self) {
        self.inner.reset_rdf_config().execute();
    }

    /// Clears the current service description.
    fn reset_service_description(&mut self) {
        self.inner.reset_service_description().execute();
    }

    /// Clears the current property graph schema.
    fn reset_pgschema(&mut self) {
        self.inner.reset_pg_schema().execute();
    }

    /// Clears the current property graph type map.
    fn reset_typemap(&mut self) {
        self.inner.reset_typemap().execute();
    }

    /// Clears the results of the most recent property graph schema validation.
    fn reset_pgschema_validation(&mut self) {
        self.inner.reset_pg_schema_validation().execute();
    }

    /// Closes and forgets the current property graph database connection.
    fn reset_pg_db_connection(&mut self) {
        self.inner.reset_pg_db_connection().execute();
    }

    /// Resets validation state across all domains.
    ///
    /// Equivalent to calling :meth:`reset_shex`, :meth:`reset_shacl_validation` and
    /// :meth:`reset_pgschema_validation` together: clears ShEx, SHACL and Property Graph
    /// schema validation results, along with their associated loaded schemas and
    /// ShapeMaps.
    fn reset_validation_results(&mut self) {
        self.inner.reset_shex().execute();
        self.inner.reset_shacl().execute();
        self.inner.reset_pg_schema_validation().execute();
    }
}
