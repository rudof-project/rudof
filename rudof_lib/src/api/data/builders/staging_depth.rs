use crate::{Rudof, api::data::DataOperations};

/// Builder for `staging_depth` operation.
pub struct StagingDepthBuilder<'a> {
    rudof: &'a Rudof,
}

impl<'a> StagingDepthBuilder<'a> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::staging_depth()` and should not be
    /// constructed directly.
    pub(crate) fn new(rudof: &'a Rudof) -> Self {
        Self { rudof }
    }

    /// Executes the operation, returning how many staging frames are open.
    pub fn execute(self) -> usize {
        <Rudof as DataOperations>::staging_depth(self.rudof)
    }
}
