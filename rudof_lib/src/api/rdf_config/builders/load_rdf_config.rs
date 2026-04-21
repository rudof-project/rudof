use crate::{
    Result, Rudof,
    api::rdf_config::RdfConfigOperations,
    formats::{InputSpec, RdfConfigFormat},
};

/// Builder for `load_rdf_config` operation.
///
/// Provides a fluent interface for configuring and executing RDF-config loading operations.
pub struct LoadRdfConfigBuilder<'a> {
    rudof: &'a mut Rudof,
    rdf_config: &'a InputSpec,
    rdf_config_format: Option<&'a RdfConfigFormat>,
}

impl<'a> LoadRdfConfigBuilder<'a> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::load_rdf_config()` and should not
    /// be constructed directly.
    pub(crate) fn new(rudof: &'a mut Rudof, rdf_config: &'a InputSpec) -> Self {
        Self {
            rudof,
            rdf_config,
            rdf_config_format: None,
        }
    }

    /// Sets the format of the RDF-config to load.
    ///
    /// # Arguments
    ///
    /// * `rdf_config_format` - The format of the RDF-config
    pub fn with_rdf_config_format(mut self, rdf_config_format: &'a RdfConfigFormat) -> Self {
        self.rdf_config_format = Some(rdf_config_format);
        self
    }

    /// Executes the RDF-config loading operation.
    pub fn execute(self) -> Result<()> {
        <Rudof as RdfConfigOperations>::load_rdf_config(self.rudof, self.rdf_config, self.rdf_config_format)
    }
}
