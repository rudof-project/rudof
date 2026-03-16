use crate::{Rudof, Result, api::rdf_config::RdfConfigOperations, formats::{InputSpec, RdfConfigFormat}};

/// Builder for `load_rdf_config` operation.
///
/// Provides a fluent interface for configuring and executing RDF-config loading operations.
pub struct LoadRdfConfigBuilder<'a> {
    rudof: &'a mut Rudof,
    rdf_config: &'a InputSpec,
    format: Option<&'a RdfConfigFormat>,
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
            format: None,
        }
    }

    /// Sets the format of the RDF-config to load.
    ///
    /// # Arguments
    ///
    /// * `format` - The format of the RDF-config
    pub fn with_format(mut self, format: &'a RdfConfigFormat) -> Self {
        self.format = Some(format);
        self
    }

    /// Executes the RDF-config loading operation.
    ///
    /// # Errors
    ///
    /// Returns an error if the RDF-config cannot be parsed or loaded.
    pub fn execute(self) -> Result<()> {
        <Rudof as RdfConfigOperations>::load_rdf_config(self.rudof, self.rdf_config, self.format)
    }
}
