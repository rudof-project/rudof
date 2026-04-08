use crate::{Result, Rudof, api::shex::ShExOperations};

/// Builder for `validate_shex` operation.
///
/// Provides a fluent interface for configuring and executing ShEx validation
/// operations.
pub struct ValidateShexBuilder<'a> {
    rudof: &'a mut Rudof,
}

impl<'a> ValidateShexBuilder<'a> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::validate_shex()` and should not
    /// be constructed directly.
    pub(crate) fn new(rudof: &'a mut Rudof) -> Self {
        Self { rudof }
    }

    /// Executes the ShEx validation operation.
    pub fn execute(self) -> Result<()> {
        <Rudof as ShExOperations>::validate_shex(self.rudof)
    }
}
