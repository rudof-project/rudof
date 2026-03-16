use crate::{
    Result, RudofConfig,
    api::core::implementations::{reset_all, update_config, config, version},
};

/// Core operations for Rudof initialization and configuration.
pub trait CoreOperations: Sized {
    /// Creates a new Rudof instance with the given configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - The configuration to use for this instance
    ///
    /// # Errors
    ///
    /// Returns an error if the RDF data cannot be initialized with the given configuration.
    fn new(config: &RudofConfig) -> Result<Self>;

    /// Returns the version string of Rudof.
    ///
    /// # Returns
    ///
    /// A string containing the version number.
    fn version(&self) -> String;

    /// Returns the current configuration.
    ///
    /// # Returns
    ///
    /// The `RudofConfig` instance.
    fn config(&self) -> RudofConfig;

    /// Updates the configuration of this Rudof instance.
    ///
    /// # Arguments
    ///
    /// * `config` - The new configuration to apply
    fn update_config(&mut self, config: &RudofConfig);

    /// Resets all state in this Rudof instance.
    ///
    /// This clears all loaded data, schemas, queries, and validation results,
    /// returning the instance to a clean state.
    fn reset_all(&mut self);
}

impl CoreOperations for crate::Rudof {
    fn new(config: &RudofConfig) -> Result<Self> {
        todo!()
    }

    fn version(&self) -> String {
        version(self)
    }

    fn config(&self) -> RudofConfig {
        config(self)
    }

    fn update_config(&mut self, config: &RudofConfig) {
        update_config(self, config)
    }

    fn reset_all(&mut self) {
        reset_all(self)
    }
}