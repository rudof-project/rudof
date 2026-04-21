use crate::{Result, Rudof, api::dctap::DctapOperations, formats::ResultDCTapFormat};
use std::io;

/// Builder for `serialize_dctap` operation.
///
/// Provides a fluent interface for configuring and executing DC-TAP serialization
/// operations with optional parameters.
pub struct SerializeDctapBuilder<'a, W: io::Write> {
    rudof: &'a Rudof,
    writer: &'a mut W,
    result_dctap_format: Option<&'a ResultDCTapFormat>,
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
            result_dctap_format: None,
        }
    }

    /// Sets the output format for serialization.
    ///
    /// # Arguments
    ///
    /// * `result_dctap_format` - The format to use when serializing the DC-TAP profile
    pub fn with_result_dctap_format(mut self, result_dctap_format: &'a ResultDCTapFormat) -> Self {
        self.result_dctap_format = Some(result_dctap_format);
        self
    }

    /// Executes the DC-TAP serialization operation with the configured parameters.
    pub fn execute(self) -> Result<()> {
        <Rudof as DctapOperations>::serialize_dctap(self.rudof, self.result_dctap_format, self.writer)
    }
}
