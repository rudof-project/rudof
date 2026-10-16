use crate::{Result, Rudof, api::data::DataOperations};

/// Builder for `begin_staging` operation.
pub struct BeginStagingBuilder<'a> {
    rudof: &'a mut Rudof,
}

impl<'a> BeginStagingBuilder<'a> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::begin_staging()` and should not be
    /// constructed directly.
    pub(crate) fn new(rudof: &'a mut Rudof) -> Self {
        Self { rudof }
    }

    /// Executes the operation, opening a staging frame over the loaded data.
    pub fn execute(self) -> Result<usize> {
        <Rudof as DataOperations>::begin_staging(self.rudof)
    }
}
