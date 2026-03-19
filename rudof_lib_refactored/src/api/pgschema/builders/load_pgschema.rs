use crate::{Rudof, Result, api::pgschema::PgSchemaOperations, formats::{InputSpec, PgSchemaFormat}};

/// Builder for the `load_pg_schema` operation.
pub struct LoadPgSchemaBuilder<'a> {
    rudof: &'a mut Rudof,
    pg_schema: &'a InputSpec,
    pg_schema_format: Option<&'a PgSchemaFormat>,
}

impl<'a> LoadPgSchemaBuilder<'a> {
    /// Create a new builder.
    ///
    /// Internal helper called by `Rudof::load_pg_schema()`; not intended for
    /// public construction by callers.
    pub(crate) fn new(rudof: &'a mut Rudof, pg_schema: &'a InputSpec) -> Self {
        Self { rudof, pg_schema, pg_schema_format: None }
    }

    /// Set the format of the input Property Graph schema.
    /// 
    /// # Arguments
    /// * `pg_schema_format` - The format of the input Property Graph schema (e
    pub fn with_pg_schema_format(mut self, pg_schema_format: &'a PgSchemaFormat) -> Self {
        self.pg_schema_format = Some(pg_schema_format);
        self
    }

    /// Execute the `load_pg_schema` operation with the configured inputs.
    ///
    /// # Errors
    ///
    /// Returns an error if the Property Graph schema cannot be parsed or
    /// loaded into the runtime.
    pub fn execute(self) -> Result<()> {
        <Rudof as PgSchemaOperations>::load_pgschema(self.rudof, self.pg_schema, self.pg_schema_format)
    }
}
