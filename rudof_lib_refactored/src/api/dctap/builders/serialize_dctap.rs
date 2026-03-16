use crate::{Rudof, Result, api::dctap::DctapOperations, formats::ResultDCTapFormat};
use std::io;

/// Builder for `serialize_dctap` operation.
///
/// Provides a fluent interface for configuring and executing DC-TAP serialization
/// operations with optional parameters.
pub struct SerializeDctapBuilder<'a, W: io::Write> {
    rudof: &'a Rudof,
    writer: &'a mut W,
    format: Option<&'a ResultDCTapFormat>,
}

impl<'a, W: io::Write> SerializeDctapBuilder<'a, W> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::serialize_dctap()` and should not
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
    /// * `format` - The format to use when serializing the DC-TAP profile
    pub fn with_format(mut self, format: &'a ResultDCTapFormat) -> Self {
        self.format = Some(format);
        self
    }

    /// Executes the DC-TAP serialization operation with the configured parameters.
    ///
    /// # Errors
    ///
    /// Returns an error if the DC-TAP profile cannot be serialized or written.
    pub fn execute(self) -> Result<()> {
        <Rudof as DctapOperations>::serialize_dctap(self.rudof, self.format, self.writer)
    }
}
