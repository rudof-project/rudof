use crate::{Result, Rudof, api::data::DataOperations, formats::ResultServiceFormat};
use std::io;

/// Builder for `serialize_service_description` operation.
///
/// Provides a fluent interface for configuring and executing service description
/// serialization operations with optional parameters.
pub struct SerializeServiceDescriptionBuilder<'a, W: io::Write> {
    rudof: &'a Rudof,
    writer: &'a mut W,
    result_service_format: Option<&'a ResultServiceFormat>,
}

impl<'a, W: io::Write> SerializeServiceDescriptionBuilder<'a, W> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::serialize_service_description()` and should not
    /// be constructed directly.
    pub(crate) fn new(rudof: &'a Rudof, writer: &'a mut W) -> Self {
        Self {
            rudof,
            writer,
            result_service_format: None,
        }
    }

    /// Sets the output format for service description serialization.
    ///
    /// # Arguments
    ///
    /// * `result_service_format` - The result_service_format to use when serializing the service description
    pub fn with_result_service_format(mut self, result_service_format: &'a ResultServiceFormat) -> Self {
        self.result_service_format = Some(result_service_format);
        self
    }

    /// Executes the service description serialization operation with the configured parameters.
    pub fn execute(self) -> Result<()> {
        <Rudof as DataOperations>::serialize_service_description(self.rudof, self.result_service_format, self.writer)
    }
}
