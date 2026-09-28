use crate::{
    api::PyRudof,
    error::Result,
    formats::{
        PyReaderMode, PyResultShaclValidationFormat, PyShaclFormat, PyShaclValidationMode, PyShaclValidationReport,
        PyShaclValidationSortMode,
    },
    guard,
    input::InputArg,
    output,
};
use pyo3::prelude::*;
use rudof_lib::errors::RudofError as CoreError;
use rudof_lib::formats::{
    DataReaderMode, ResultShaclValidationFormat, ShaclFormat, ShaclValidationMode, ShaclValidationSortByMode,
};

#[cfg_attr(feature = "stub-gen", pyo3_stub_gen_derive::gen_stub_pymethods)]
#[pymethods]
impl PyRudof {
    /// Loads a SHACL shapes graph from a string, file path or URL.
    ///
    /// Args:
    ///     input (str | os.PathLike, optional): Inline shapes, file path or URL. If not provided, the shapes are
    ///         extracted from the currently loaded data.
    ///     format (ShaclFormat, optional): RDF format. Defaults to ``ShaclFormat.Turtle``.
    ///     base (str, optional): Base IRI for resolving relative IRIs.
    ///     reader_mode (ReaderMode, optional): Error handling. Defaults to ``ReaderMode.Lax``.
    ///
    /// Raises:
    ///     InputError: If the input string, file or URL cannot be resolved.
    ///     ShaclError: If the shapes graph is malformed.
    #[pyo3(signature = (input = None, format = None, base = None, reader_mode = None))]
    fn read_shacl(
        &mut self,
        py: Python<'_>,
        input: Option<InputArg>,
        format: Option<&PyShaclFormat>,
        base: Option<&str>,
        reader_mode: Option<&PyReaderMode>,
    ) -> Result<()> {
        let input = input.map(|InputArg(spec)| spec);
        let format: Option<ShaclFormat> = format.map(Into::into);
        let reader_mode: Option<DataReaderMode> = reader_mode.map(Into::into);
        let base = base.map(str::to_owned);

        guard::detached(py, move || {
            let mut b = self.inner.load_shacl_shapes();
            if let Some(i) = &input {
                b = b.with_shacl_schema(i);
            }
            if let Some(f) = &format {
                b = b.with_shacl_schema_format(f);
            }
            if let Some(base) = &base {
                b = b.with_base(base);
            }
            if let Some(m) = &reader_mode {
                b = b.with_reader_mode(m);
            }
            b.execute()
        })?;
        Ok(())
    }

    /// Serializes the current SHACL shapes graph to a string.
    ///
    /// Args:
    ///     format (ShaclFormat, optional): Output format. Defaults to ``ShaclFormat.Turtle``.
    ///
    /// Returns:
    ///     str: Serialized shapes graph.
    ///
    /// Raises:
    ///     ShaclError: If no shapes are loaded or serialization fails.
    #[pyo3(signature = (format = None))]
    fn serialize_shacl(&self, py: Python<'_>, format: Option<&PyShaclFormat>) -> Result<String> {
        let format: Option<ShaclFormat> = format.map(Into::into);
        output::capture_string_detached(py, move |w| {
            let mut s = self.inner.serialize_shacl_shapes(w);
            if let Some(f) = &format {
                s = s.with_shacl_result_format(f);
            }
            s.execute()
        })
    }

    /// Validates the current RDF data against the loaded SHACL shapes.
    ///
    /// Args:
    ///     mode (ShaclValidationMode, optional): Validation engine. Defaults to ``ShaclValidationMode.Native``.
    ///         - ``Native``: Fast built-in engine (recommended for production)
    ///         - ``Sparql``: SPARQL-based engine
    ///
    /// Returns:
    ///     ShaclValidationReport: A snapshot of the result — ``.conforms``,
    ///     ``.violations``, ``len()`` and iteration. Unaffected by later calls on the
    ///     session.
    ///
    /// Raises:
    ///     ValidationError: If no data or schema is loaded, or validation fails.
    #[pyo3(signature = (mode = None))]
    fn validate_shacl(
        &mut self,
        py: Python<'_>,
        mode: Option<&PyShaclValidationMode>,
    ) -> Result<PyShaclValidationReport> {
        let mode: Option<ShaclValidationMode> = mode.map(Into::into);

        guard::detached(py, || {
            let mut b = self.inner.validate_shacl();
            if let Some(m) = &mode {
                b = b.with_shacl_validation_mode(m);
            }
            b.execute()
        })?;

        let report = self
            .inner
            .shacl_validation_results()
            .cloned()
            .ok_or(CoreError::Generic {
                error: "validate_shacl produced no results".into(),
            })?;
        // Guarded because building the report renders every focus node and value to a
        // string, which panics on a term `rudof_rdf` cannot format yet.
        guard::catch_value(|| PyShaclValidationReport::new(report))
    }

    /// Serializes the results of the most recent :meth:`validate_shacl` call.
    ///
    /// Args:
    ///     format (ResultShaclValidationFormat, optional): Output format. Defaults to
    ///         ``ResultShaclValidationFormat.Details``.
    ///     sort_mode (ShaclValidationSortMode, optional): Sorting mode. Defaults to
    ///         ``ShaclValidationSortMode.Severity``.
    ///
    /// Returns:
    ///     str: Serialized validation results.
    ///
    /// Raises:
    ///     ValidationError: If there are no results to serialize.
    #[pyo3(signature = (format = None, sort_mode = None))]
    fn serialize_shacl_validation_results(
        &self,
        py: Python<'_>,
        format: Option<&PyResultShaclValidationFormat>,
        sort_mode: Option<&PyShaclValidationSortMode>,
    ) -> Result<String> {
        let format: Option<ResultShaclValidationFormat> = format.map(Into::into);
        let sort_mode: Option<ShaclValidationSortByMode> = sort_mode.map(Into::into);

        output::capture_string_detached(py, move |w| {
            let mut s = self.inner.serialize_shacl_validation_results(w);
            if let Some(f) = &format {
                s = s.with_result_shacl_validation_format(f);
            }
            if let Some(m) = &sort_mode {
                s = s.with_shacl_validation_sort_order_mode(m);
            }
            s.execute()
        })
    }
}
