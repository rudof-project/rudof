use crate::{Rudof, Result, api::pgschema::PgSchemaOperations, formats::PgSchemaFormat};
use std::io;

/// Builder for the `serialize_pg_schema` operation.
pub struct SerializePgSchemaBuilder<'a, W: io::Write> {
    rudof: &'a Rudof,
    pg_schema_format: Option<&'a PgSchemaFormat>,
    writer: &'a mut W,
}

impl<'a, W: io::Write> SerializePgSchemaBuilder<'a, W> {
    /// Create a new serialize builder.
    ///
    /// Called by `Rudof::serialize_pg_schema()`; not meant for direct
    /// construction by callers.
    pub(crate) fn new(rudof: &'a Rudof, writer: &'a mut W) -> Self {
        Self { rudof, pg_schema_format: None, writer }
    }

    /// Set the output format to use when serializing the Property Graph schema.
    /// 
    /// # Arguments
    /// * `pg_schema_format` - Desired output format for the Property Graph schema (e.g. one of the supported `PgSchemaFormat` variants)
    pub fn with_result_pg_schema_format(mut self, pg_schema_format: &'a PgSchemaFormat) -> Self {
        self.pg_schema_format = Some(pg_schema_format);
        self
    }

    /// Execute the serialization using the configured writer.
    ///
    /// # Errors
    ///
    /// Returns an error if no Property Graph schema is loaded or if writing
    /// to the provided writer fails.
    pub fn execute(self) -> Result<()> {
        <Rudof as PgSchemaOperations>::serialize_pgschema(self.rudof, self.pg_schema_format, self.writer)
    }
}
