use crate::{Rudof, RudofConfig, api::core::CoreOperations};

/// Builder for `config` operation.
///
/// Provides a fluent interface for configuring and executing config checks.
pub struct ConfigBuilder<'a> {
    rudof: &'a Rudof,
}

impl<'a> ConfigBuilder<'a> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::config()` and should not
    /// be constructed directly.
    pub(crate) fn new(rudof: &'a Rudof) -> Self {
        Self { rudof }
    }

    /// Executes the config check operation.
    ///
    /// This retrieves the config information from the Rudof instance.
    pub fn execute(self) -> &'a RudofConfig {
        <Rudof as CoreOperations>::config(self.rudof)
    }
}
