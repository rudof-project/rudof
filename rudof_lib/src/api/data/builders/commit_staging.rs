use crate::{Result, Rudof, api::data::DataOperations};

/// Builder for `commit_staging` operation.
pub struct CommitStagingBuilder<'a> {
    rudof: &'a mut Rudof,
}

impl<'a> CommitStagingBuilder<'a> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::commit_staging()` and should not be
    /// constructed directly.
    pub(crate) fn new(rudof: &'a mut Rudof) -> Self {
        Self { rudof }
    }

    /// Executes the operation, keeping what the innermost open frame staged.
    pub fn execute(self) -> Result<()> {
        <Rudof as DataOperations>::commit_staging(self.rudof)
    }
}
