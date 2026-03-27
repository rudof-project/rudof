use crate::{Rudof, Result, api::pgschema::PgSchemaOperations, formats::InputSpec};

/// Builder for the `load_typemap` operation.
pub struct LoadTypemapBuilder<'a> {
    rudof: &'a mut Rudof,
    typemap: &'a InputSpec,
}

impl<'a> LoadTypemapBuilder<'a> {
    /// Create a new builder.
    ///
    /// Internal helper called by `Rudof::load_typemap()`; not intended for
    /// public construction by callers.
    pub(crate) fn new(rudof: &'a mut Rudof, typemap: &'a InputSpec) -> Self {
        Self { rudof, typemap }
    }

    /// Execute the `load_typemap` operation with the configured inputs.
    ///
    /// # Errors
    ///
    /// Returns an error if the typemap cannot be parsed or loaded into the runtime.
    pub fn execute(self) -> Result<()> {
        <Rudof as PgSchemaOperations>::load_typemap(self.rudof, self.typemap)
    }
}
