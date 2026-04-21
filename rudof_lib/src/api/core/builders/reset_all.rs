use crate::{Rudof, api::core::CoreOperations};

/// Builder for `reset_all` operation.
///
/// Provides a fluent interface for configuring and executing a full state reset.
pub struct ResetAllBuilder<'a> {
    rudof: &'a mut Rudof,
}

impl<'a> ResetAllBuilder<'a> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::reset_all()` and should not
    /// be constructed directly.
    pub(crate) fn new(rudof: &'a mut Rudof) -> Self {
        Self { rudof }
    }

    /// Executes the reset all operation.
    ///
    /// This clears all loaded data, schemas, queries, and validation results,
    /// returning the instance to a clean state.
    pub fn execute(self) {
        <Rudof as CoreOperations>::reset_all(self.rudof)
    }
}
