use crate::{Result, Rudof, api::rdf_config::RdfConfigOperations, formats::ResultRdfConfigFormat};
use std::io;

/// Builder for `serialize_rdf_config` operation.
///
/// Provides a fluent interface for configuring and executing RDF-config serialization operations.
pub struct SerializeRdfConfigBuilder<'a, W: io::Write> {
    rudof: &'a Rudof,
    result_rdf_config_format: Option<&'a ResultRdfConfigFormat>,
    writer: &'a mut W,
}

impl<'a, W: io::Write> SerializeRdfConfigBuilder<'a, W> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::serialize_rdf_config()` and should not
    /// be constructed directly.
    pub(crate) fn new(rudof: &'a Rudof, writer: &'a mut W) -> Self {
        Self {
            rudof,
            result_rdf_config_format: None,
            writer,
        }
    }

    /// Sets the format of the RDF-config to serialize.
    ///
    /// # Arguments
    ///
    /// * `result_rdf_config_format` - The format of the RDF-config
    pub fn with_result_rdf_config_format(mut self, format: &'a ResultRdfConfigFormat) -> Self {
        self.result_rdf_config_format = Some(format);
        self
    }

    /// Executes the RDF-config serialization operation.
    pub fn execute(self) -> Result<()> {
        <Rudof as RdfConfigOperations>::serialize_rdf_config(self.rudof, self.result_rdf_config_format, self.writer)
    }
}
