use crate::{Rudof, Result, api::pgschema::PgSchemaOperations, formats::InputSpec};

/// Builder for the `load_pg_schema` operation.
pub struct LoadPgSchemaBuilder<'a> {
    rudof: &'a mut Rudof,
    pg_schema: &'a InputSpec,
}

impl<'a> LoadPgSchemaBuilder<'a> {
    /// Create a new builder.
    ///
    /// Internal helper called by `Rudof::load_pg_schema()`; not intended for
    /// public construction by callers.
    pub(crate) fn new(rudof: &'a mut Rudof, pg_schema: &'a InputSpec) -> Self {
        Self { rudof, pg_schema }
    }

    /// Execute the `load_pg_schema` operation with the configured inputs.
    ///
    /// # Errors
    ///
    /// Returns an error if the Property Graph schema cannot be parsed or
    /// loaded into the runtime.
    pub fn execute(self) -> Result<()> {
        <Rudof as PgSchemaOperations>::load_pgschema(self.rudof, self.pg_schema)
    }
}
