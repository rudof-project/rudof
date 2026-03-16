use crate::{Rudof, Result, api::pgschema::PgSchemaOperations, formats::ResultPgSchemaValidationFormat};
use std::io;

/// Builder for the `serialize_pgschema_validation_results` operation.
pub struct SerializePgSchemaValidationResultsBuilder<'a, W: io::Write> {
    rudof: &'a Rudof,
    writer: &'a mut W,
    format: Option<&'a ResultPgSchemaValidationFormat>,
}

impl<'a, W: io::Write> SerializePgSchemaValidationResultsBuilder<'a, W> {
    /// Create a new serialization builder.
    ///
    /// Internal: created by `Rudof::serialize_pgschema_validation_results()`.
    pub(crate) fn new(rudof: &'a Rudof, writer: &'a mut W) -> Self {
        Self { rudof, writer, format: None }
    }

    /// Set the output format to use when serializing validation results.
    ///
    /// # Arguments
    ///
    /// * `format` - Desired result format (e.g. one of the supported
    ///   `ResultPgSchemaValidationFormat` variants)
    pub fn with_format(mut self, format: &'a ResultPgSchemaValidationFormat) -> Self {
        self.format = Some(format);
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
        <Rudof as PgSchemaOperations>::serialize_pgschema_validation_results(self.rudof, self.format, self.writer)
    }
}
