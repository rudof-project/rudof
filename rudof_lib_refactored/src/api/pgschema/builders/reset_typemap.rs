use crate::{Rudof, api::pgschema::PgSchemaOperations};

/// Builder for the `reset_typemap` operation.
pub struct ResetTypemapBuilder<'a> {
    rudof: &'a mut Rudof,
}

impl<'a> ResetTypemapBuilder<'a> {
    /// Create a new reset builder.
    ///
    /// Internal: called by `Rudof::reset_typemap()`.
    pub(crate) fn new(rudof: &'a mut Rudof) -> Self {
        Self { rudof }
    }

    /// Execute the reset operation.
    ///
    /// This method does not return a `Result` because the underlying trait
    /// method is defined to return `()`; it will perform the reset in-place.
    pub fn execute(self) {
        <Rudof as PgSchemaOperations>::reset_typemap(self.rudof)
    }
}
