use crate::{Rudof, Result, api::pgschema::PgSchemaOperations};
use std::io;

/// Builder for the `serialize_pg_schema` operation.
pub struct SerializePgSchemaBuilder<'a, W: io::Write> {
    rudof: &'a Rudof,
    writer: &'a mut W,
}

impl<'a, W: io::Write> SerializePgSchemaBuilder<'a, W> {
    /// Create a new serialize builder.
    ///
    /// Called by `Rudof::serialize_pg_schema()`; not meant for direct
    /// construction by callers.
    pub(crate) fn new(rudof: &'a Rudof, writer: &'a mut W) -> Self {
        Self { rudof, writer }
    }

    /// Execute the serialization using the configured writer.
    ///
    /// # Errors
    ///
    /// Returns an error if no Property Graph schema is loaded or if writing
    /// to the provided writer fails.
    pub fn execute(self) -> Result<()> {
        <Rudof as PgSchemaOperations>::serialize_pgschema(self.rudof, self.writer)
    }
}
