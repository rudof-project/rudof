use crate::{Result, Rudof, api::data::DataOperations};

/// Builder for `rollback_staging` operation.
pub struct RollbackStagingBuilder<'a> {
    rudof: &'a mut Rudof,
}

impl<'a> RollbackStagingBuilder<'a> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::rollback_staging()` and should not be
    /// constructed directly.
    pub(crate) fn new(rudof: &'a mut Rudof) -> Self {
        Self { rudof }
    }

    /// Executes the operation, discarding what the innermost open frame staged.
    pub fn execute(self) -> Result<()> {
        <Rudof as DataOperations>::rollback_staging(self.rudof)
    }
}
