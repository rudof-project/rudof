use crate::{Rudof, api::data::DataOperations};

/// Builder for `reset_service_description` operation.
///
/// Provides a fluent interface for configuring and executing service description
/// reset operations.
pub struct ResetServiceDescriptionBuilder<'a> {
    rudof: &'a mut Rudof,
}

impl<'a> ResetServiceDescriptionBuilder<'a> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::reset_service_description()` and should not
    /// be constructed directly.
    pub(crate) fn new(rudof: &'a mut Rudof) -> Self {
        Self { rudof }
    }

    /// Executes the service description reset operation.
    pub fn execute(self) {
        <Rudof as DataOperations>::reset_service_description(self.rudof)
    }
}
