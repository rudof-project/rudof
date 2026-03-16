use crate::{Rudof, api::rdf_config::RdfConfigOperations};

/// Builder for `reset_rdf_config` operation.
///
/// Provides a fluent interface for configuring and executing RDF-config resetting operations.
pub struct ResetRdfConfigBuilder<'a> {
    rudof: &'a mut Rudof,
}

impl<'a> ResetRdfConfigBuilder<'a> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::reset_rdf_config()` and should not
    /// be constructed directly.
    pub(crate) fn new(rudof: &'a mut Rudof) -> Self {
        Self {
            rudof,
        }
    }

    /// Executes the RDF-config resetting operation.
    ///
    /// # Errors
    ///
    /// Returns an error if the RDF-config cannot be parsed or loaded.
    pub fn execute(self) {
        <Rudof as RdfConfigOperations>::reset_rdf_config(self.rudof)
    }
}
