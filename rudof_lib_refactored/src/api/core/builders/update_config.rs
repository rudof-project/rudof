use crate::{Rudof, api::core::CoreOperations, RudofConfig};

/// Builder for `update_config` operation.
///
/// Provides a fluent interface for configuring and executing configuration updates.
pub struct UpdateConfigBuilder<'a> {
    rudof: &'a mut Rudof,
    config: &'a RudofConfig,
}

impl<'a> UpdateConfigBuilder<'a> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::update_config()` and should not
    /// be constructed directly.
    pub(crate) fn new(rudof: &'a mut Rudof, config: &'a RudofConfig) -> Self {
        Self { rudof, config }
    }

    /// Executes the configuration update operation.
    ///
    /// This applies the new configuration to the Rudof instance.
    pub fn execute(self) {
        <Rudof as CoreOperations>::update_config(self.rudof, self.config)
    }
}
