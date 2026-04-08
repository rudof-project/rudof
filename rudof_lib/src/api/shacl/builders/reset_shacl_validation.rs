use crate::{Rudof, api::shacl::ShaclOperations};

/// Builder for `reset_shacl_validation` operation.
///
/// Provides a fluent interface for configuring and executing SHACL validation reset
/// operations.
pub struct ResetShaclBuilder<'a> {
    rudof: &'a mut Rudof,
}

impl<'a> ResetShaclBuilder<'a> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::reset_shacl_validation()` and should not
    /// be constructed directly.
    pub(crate) fn new(rudof: &'a mut Rudof) -> Self {
        Self { rudof }
    }

    /// Executes the SHACL validation reset operation.
    pub fn execute(self) {
        <Rudof as ShaclOperations>::reset_shacl_validation(self.rudof)
    }
}
