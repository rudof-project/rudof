use crate::{Rudof, api::pgschema::PgSchemaOperations};

/// Builder for the `reset_pg_schema` operation.
pub struct ResetPgSchemaBuilder<'a> {
    rudof: &'a mut Rudof,
}

impl<'a> ResetPgSchemaBuilder<'a> {
    /// Create a new reset builder.
    ///
    /// Internal: called by `Rudof::reset_pg_schema()`.
    pub(crate) fn new(rudof: &'a mut Rudof) -> Self {
        Self { rudof }
    }

    /// Execute the reset of pgschema state.
    pub fn execute(self) {
        <Rudof as PgSchemaOperations>::reset_pgschema(self.rudof)
    }
}
