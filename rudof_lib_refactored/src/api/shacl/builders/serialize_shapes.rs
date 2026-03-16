use crate::{Rudof, Result, api::shacl::ShaclOperations, formats::ShaclFormat};
use std::io;

/// Builder for `serialize_shapes` operation.
///
/// Provides a fluent interface for configuring and executing shapes serialization
/// operations with optional parameters.
pub struct SerializeShapesBuilder<'a, W: io::Write> {
    rudof: &'a Rudof,
    writer: &'a mut W,
    format: Option<&'a ShaclFormat>,
}

impl<'a, W: io::Write> SerializeShapesBuilder<'a, W> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::serialize_shapes()` and should not
    /// be constructed directly.
    pub(crate) fn new(rudof: &'a Rudof, writer: &'a mut W) -> Self {
        Self {
            rudof,
            writer,
            format: None,
        }
    }

    /// Sets the output format for shapes serialization.
    ///
    /// # Arguments
    ///
    /// * `format` - The format to use when serializing the shapes
    pub fn with_format(mut self, format: &'a ShaclFormat) -> Self {
        self.format = Some(format);
        self
    }

    /// Executes the shapes serialization operation with the configured parameters.
    ///
    /// # Errors
    ///
    /// Returns an error if no shapes are loaded or serialization fails.
    pub fn execute(self) -> Result<()> {
        <Rudof as ShaclOperations>::serialize_shapes(
            self.rudof,
            self.format,
            self.writer,
        )
    }
}
