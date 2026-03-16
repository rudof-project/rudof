use crate::{Rudof, api::shacl::ShaclOperations};

/// Builder for `reset_shapes` operation.
///
/// Provides a fluent interface for configuring and executing shapes reset
/// operations.
pub struct ResetShapesBuilder<'a> {
    rudof: &'a mut Rudof,
}

impl<'a> ResetShapesBuilder<'a> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::reset_shapes()` and should not
    /// be constructed directly.
    pub(crate) fn new(rudof: &'a mut Rudof) -> Self {
        Self { rudof }
    }

    /// Executes the shapes reset operation.
    pub fn execute(self) {
        <Rudof as ShaclOperations>::reset_shapes(self.rudof)
    }
}
