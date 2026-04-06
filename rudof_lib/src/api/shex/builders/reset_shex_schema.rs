use crate::{Rudof, api::shex::ShExOperations};

/// Builder for `reset_shex_schema` operation.
///
/// Provides a fluent interface for configuring and executing schema reset
/// operations.
pub struct ResetShexSchemaBuilder<'a> {
    rudof: &'a mut Rudof,
}

impl<'a> ResetShexSchemaBuilder<'a> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::reset_shex_schema()` and should not
    /// be constructed directly.
    pub(crate) fn new(rudof: &'a mut Rudof) -> Self {
        Self { rudof }
    }

    /// Executes the schema reset operation.
    pub fn execute(self) {
        <Rudof as ShExOperations>::reset_shex_schema(self.rudof)
    }
}
