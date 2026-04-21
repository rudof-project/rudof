use crate::{Rudof, api::pgschema::PgSchemaOperations};

/// Builder for the `reset_pg_schema_validation` operation.
pub struct ResetPgSchemaValidationBuilder<'a> {
    rudof: &'a mut Rudof,
}

impl<'a> ResetPgSchemaValidationBuilder<'a> {
    /// Create a new builder instance.
    ///
    /// Internal use by `Rudof::reset_pgschema_validation()`.
    pub(crate) fn new(rudof: &'a mut Rudof) -> Self {
        Self { rudof }
    }

    /// Execute the reset of pgschema validation state.
    pub fn execute(self) {
        <Rudof as PgSchemaOperations>::reset_pgschema_validation(self.rudof)
    }
}
