use crate::{Rudof, Result, api::shex::ShExOperations, formats::ShapeMapFormat};
use std::io;

/// Builder for `serialize_shapemap` operation.
///
/// Provides a fluent interface for configuring and executing shape map serialization
/// operations with optional parameters.
pub struct SerializeShapemapBuilder<'a, W: io::Write> {
    rudof: &'a Rudof,
    writer: &'a mut W,
    shapemap_format: Option<&'a ShapeMapFormat>,
}

impl<'a, W: io::Write> SerializeShapemapBuilder<'a, W> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::serialize_shapemap()` and should not
    /// be constructed directly.
    pub(crate) fn new(rudof: &'a Rudof, writer: &'a mut W) -> Self {
        Self {
            rudof,
            writer,
            shapemap_format: None,
        }
    }

    /// Sets the output format for shape map serialization.
    ///
    /// # Arguments
    ///
    /// * `shapemap_format` - The format to use when serializing the shape map
    pub fn with_result_shapemap_format(mut self, shapemap_format: &'a ShapeMapFormat) -> Self {
        self.shapemap_format = Some(shapemap_format);
        self
    }

    /// Executes the shape map serialization operation with the configured parameters.
    ///
    /// # Errors
    ///
    /// Returns an error if no shape map is loaded or serialization fails.
    pub fn execute(self) -> Result<()> {
        <Rudof as ShExOperations>::serialize_shapemap(
            self.rudof,
            self.shapemap_format,
            self.writer,
        )
    }
}
