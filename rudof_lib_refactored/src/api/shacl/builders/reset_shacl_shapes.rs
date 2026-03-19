use crate::{Rudof, api::shacl::ShaclOperations};

/// Builder for `reset_shacl_schema` operation.
///
/// Provides a fluent interface for configuring and executing schema reset
/// operations.
pub struct ResetShaclShapesBuilder<'a> {
    rudof: &'a mut Rudof,
}

impl<'a> ResetShaclShapesBuilder<'a> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::reset_shacl_shapes()` and should not
    /// be constructed directly.
    pub(crate) fn new(rudof: &'a mut Rudof) -> Self {
        Self { rudof }
    }

    /// Executes the schema reset operation.
    pub fn execute(self) {
        <Rudof as ShaclOperations>::reset_shacl_schema(self.rudof)
    }
}
