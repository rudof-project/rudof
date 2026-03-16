use crate::{Rudof, Result, api::pgschema::PgSchemaOperations};

/// Builder for the `run_pgschema_validation` operation.
pub struct RunPgSchemaValidationBuilder<'a> {
    rudof: &'a mut Rudof,
}

impl<'a> RunPgSchemaValidationBuilder<'a> {
    /// Create a new run-validation builder.
    ///
    /// Internal: constructed by `Rudof::run_pgschema_validation()`.
    pub(crate) fn new(rudof: &'a mut Rudof) -> Self {
        Self { rudof }
    }

    /// Execute the validation run.
    ///
    /// # Errors
    ///
    /// Returns an error if validation cannot be run because required inputs
    /// (e.g. pgschema or shape map) are missing or malformed.
    pub fn execute(self) -> Result<()> {
        <Rudof as PgSchemaOperations>::run_pgschema_validation(self.rudof)
    }
}
