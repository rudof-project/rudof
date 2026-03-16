use crate::{Rudof, api::data::DataOperations};

/// Builder for `reset_data` operation.
///
/// Provides a fluent interface for configuring and executing data reset
/// operations.
pub struct ResetDataBuilder<'a> {
    rudof: &'a mut Rudof,
}

impl<'a> ResetDataBuilder<'a> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::reset_data()` and should not
    /// be constructed directly.
    pub(crate) fn new(rudof: &'a mut Rudof) -> Self {
        Self { rudof }
    }

    /// Executes the data reset operation.
    ///
    /// # Errors
    ///
    /// Returns an error if the data cannot be reset.
    pub fn execute(self) {
        <Rudof as DataOperations>::reset_data(self.rudof)
    }
}
