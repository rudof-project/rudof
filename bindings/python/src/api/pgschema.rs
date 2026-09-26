use crate::{
    api::PyRudof,
    error::Result,
    formats::{PyPgSchemaFormat, PyPgSchemaValidationReport, PyResultPgSchemaValidationFormat},
    guard,
    input::InputArg,
    output,
};
use pyo3::prelude::*;
use rudof_lib::errors::RudofError as CoreError;
use rudof_lib::formats::{PgSchemaFormat, ResultPgSchemaValidationFormat};

#[cfg_attr(feature = "stub-gen", pyo3_stub_gen_derive::gen_stub_pymethods)]
#[pymethods]
impl PyRudof {
    /// Loads a property graph schema from a string, file path or URL.
    ///
    /// Args:
    ///     input (str | os.PathLike): Inline schema, file path or URL.
    ///     format (PgSchemaFormat, optional): Schema format. Defaults to ``PgSchemaFormat.PgSchemaC``.
    ///
    /// Raises:
    ///     InputError: If the input string, file or URL cannot be resolved.
    ///     PgSchemaError: If the schema cannot be parsed.
    #[pyo3(signature = (input, format = None))]
    fn read_pgschema(&mut self, py: Python<'_>, input: InputArg, format: Option<&PyPgSchemaFormat>) -> Result<()> {
        let InputArg(input) = input;
        let format: Option<PgSchemaFormat> = format.map(Into::into);

        guard::detached(py, move || {
            let mut b = self.inner.load_pg_schema(&input);
            if let Some(f) = &format {
                b = b.with_pg_schema_format(f);
            }
            b.execute()
        })?;
        Ok(())
    }

    /// Serializes the current property graph schema to a string.
    ///
    /// Args:
    ///     format (PgSchemaFormat, optional): Output format. Defaults to ``PgSchemaFormat.PgSchemaC``.
    ///
    /// Returns:
    ///     str: Serialized property graph schema.
    ///
    /// Raises:
    ///     PgSchemaError: If no schema is loaded or serialization fails.
    #[pyo3(signature = (format = None))]
    fn serialize_pgschema(&self, py: Python<'_>, format: Option<&PyPgSchemaFormat>) -> Result<String> {
        let format: Option<PgSchemaFormat> = format.map(Into::into);
        output::capture_string_detached(py, move |w| {
            let mut s = self.inner.serialize_pg_schema(w);
            if let Some(f) = &format {
                s = s.with_result_pg_schema_format(f);
            }
            s.execute()
        })
    }

    /// Loads a typemap (node -> PG schema type bindings) from a string, file path or URL.
    ///
    /// Required before calling :meth:`validate_pgschema`.
    ///
    /// Args:
    ///     input (str | os.PathLike): Inline typemap, file path or URL.
    ///
    /// Raises:
    ///     InputError: If the input string, file or URL cannot be resolved.
    ///     PgSchemaError: If the typemap cannot be parsed.
    fn read_typemap(&mut self, py: Python<'_>, input: InputArg) -> Result<()> {
        let InputArg(input) = input;
        guard::detached(py, move || self.inner.load_typemap(&input).execute())?;
        Ok(())
    }

    /// Validates the current property graph data against the loaded PG schema and typemap.
    ///
    /// Returns:
    ///     PgSchemaValidationReport: A snapshot of the result — ``.conforms``,
    ///     ``.violations``, ``len()`` and iteration. Unaffected by later calls on the
    ///     session.
    ///
    /// Raises:
    ///     ValidationError: If no data, PG schema, or typemap is loaded.
    fn validate_pgschema(&mut self, py: Python<'_>) -> Result<PyPgSchemaValidationReport> {
        guard::detached(py, || self.inner.validate_pgschema().execute())?;

        let result = self.inner.pgschema_validation_results().ok_or(CoreError::Generic {
            error: "validate_pgschema produced no results".into(),
        })?;
        Ok(PyPgSchemaValidationReport::new(result))
    }

    /// Serializes the results of the most recent :meth:`validate_pgschema` call.
    ///
    /// Args:
    ///     format (ResultPgSchemaValidationFormat, optional): Output format. Defaults to
    ///         ``ResultPgSchemaValidationFormat.Compact``.
    ///
    /// Returns:
    ///     str: Serialized validation results.
    ///
    /// Raises:
    ///     ValidationError: If there are no results to serialize.
    #[pyo3(signature = (format = None))]
    fn serialize_pgschema_validation_results(
        &self,
        py: Python<'_>,
        format: Option<&PyResultPgSchemaValidationFormat>,
    ) -> Result<String> {
        let format: Option<ResultPgSchemaValidationFormat> = format.map(Into::into);
        output::capture_string_detached(py, move |w| {
            let mut s = self.inner.serialize_pgschema_validation_results(w);
            if let Some(f) = &format {
                s = s.with_result_pg_schema_validation_format(f);
            }
            s.execute()
        })
    }
}
