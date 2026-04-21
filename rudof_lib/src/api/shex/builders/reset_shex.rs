use crate::{Rudof, api::shex::ShExOperations};

/// Builder for `reset_shex` operation.
///
/// Provides a fluent interface for configuring and executing ShEx validation reset
/// operations.
pub struct ResetShexBuilder<'a> {
    rudof: &'a mut Rudof,
}

impl<'a> ResetShexBuilder<'a> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::reset_shex()` and should not
    /// be constructed directly.
    pub(crate) fn new(rudof: &'a mut Rudof) -> Self {
        Self { rudof }
    }

    /// Executes the ShEx validation reset operation.
    pub fn execute(self) {
        <Rudof as ShExOperations>::reset_shex(self.rudof)
    }
}
