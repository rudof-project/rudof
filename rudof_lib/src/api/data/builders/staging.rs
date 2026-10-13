use crate::{Result, Rudof, api::data::DataOperations, types::StagingGuard};

/// Builder for `staging` operation.
pub struct StagingBuilder<'a> {
    rudof: &'a mut Rudof,
}

impl<'a> StagingBuilder<'a> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::staging()` and should not be constructed
    /// directly.
    pub(crate) fn new(rudof: &'a mut Rudof) -> Self {
        Self { rudof }
    }

    /// Executes the operation, opening a staging frame and returning the guard that
    /// closes it.
    pub fn execute(self) -> Result<StagingGuard<'a>> {
        <Rudof as DataOperations>::staging(self.rudof)
    }
}
