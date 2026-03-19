use crate::{Rudof, Result, api::pgschema::PgSchemaOperations, formats::ResultPgSchemaValidationFormat};
use std::io;

/// Builder for the `serialize_pgschema_validation_results` operation.
pub struct SerializePgSchemaValidationResultsBuilder<'a, W: io::Write> {
    rudof: &'a Rudof,
    writer: &'a mut W,
    result_pg_schema_validation_format: Option<&'a ResultPgSchemaValidationFormat>,
}

impl<'a, W: io::Write> SerializePgSchemaValidationResultsBuilder<'a, W> {
    /// Create a new serialization builder.
    ///
    /// Internal: created by `Rudof::serialize_pgschema_validation_results()`.
    pub(crate) fn new(rudof: &'a Rudof, writer: &'a mut W) -> Self {
        Self { rudof, writer, result_pg_schema_validation_format: None }
    }

    /// Set the output format to use when serializing validation results.
    ///
    /// # Arguments
    ///
    /// * `result_pg_schema_validation_format` - Desired result format (e.g. one of the supported
    ///   `ResultPgSchemaValidationFormat` variants)
    pub fn with_result_pg_schema_validation_format(mut self, result_pg_schema_validation_format: &'a ResultPgSchemaValidationFormat) -> Self {
        self.result_pg_schema_validation_format = Some(result_pg_schema_validation_format);
        self
    }

    /// Execute the serialization with the configured writer and optional
    /// format.
    ///
    /// # Errors
    ///
    /// Returns an error if no validation results are available or if writing
    /// fails for any reason.
    pub fn execute(self) -> Result<()> {
        <Rudof as PgSchemaOperations>::serialize_pgschema_validation_results(self.rudof, self.result_pg_schema_validation_format, self.writer)
    }
}
