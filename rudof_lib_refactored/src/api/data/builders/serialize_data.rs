use crate::{Rudof, Result, api::data::DataOperations, formats::ResultDataFormat};
use std::io;

/// Builder for `serialize_data` operation.
///
/// Provides a fluent interface for configuring and executing data serialization
/// operations with optional parameters.
pub struct SerializeDataBuilder<'a, W: io::Write> {
    rudof: &'a Rudof,
    writer: &'a mut W,
    format: Option<&'a ResultDataFormat>,
}

impl<'a, W: io::Write> SerializeDataBuilder<'a, W> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::serialize_data()` and should not
    /// be constructed directly.
    pub(crate) fn new(rudof: &'a Rudof, writer: &'a mut W) -> Self {
        Self {
            rudof,
            writer,
            format: None,
        }
    }

    /// Sets the output format for serialization.
    ///
    /// # Arguments
    ///
    /// * `format` - The format to use when serializing the data
    pub fn with_format(mut self, format: &'a ResultDataFormat) -> Self {
        self.format = Some(format);
        self
    }

    /// Executes the data serialization operation with the configured parameters.
    ///
    /// # Errors
    ///
    /// Returns an error if the data cannot be serialized or written.
    pub fn execute(self) -> Result<()> {
        <Rudof as DataOperations>::serialize_data(self.rudof, self.format, self.writer)
    }
}
