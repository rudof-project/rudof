use crate::{Result, Rudof, api::shacl::ShaclOperations, formats::ShaclFormat};
use std::io;

/// Builder for `serialize_shacl_schema` operation.
///
/// Provides a fluent interface for configuring and executing schema serialization
/// operations with optional parameters.
pub struct SerializeShaclShapesBuilder<'a, W: io::Write> {
    rudof: &'a Rudof,
    writer: &'a mut W,
    shacl_format: Option<&'a ShaclFormat>,
}

impl<'a, W: io::Write> SerializeShaclShapesBuilder<'a, W> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::serialize_shacl_schema()` and should not
    /// be constructed directly.
    pub(crate) fn new(rudof: &'a Rudof, writer: &'a mut W) -> Self {
        Self {
            rudof,
            writer,
            shacl_format: None,
        }
    }

    /// Sets the output format for schema serialization.
    ///
    /// # Arguments
    ///
    /// * `shacl_format` - The format to use when serializing the schema
    pub fn with_shacl_result_format(mut self, shacl_format: &'a ShaclFormat) -> Self {
        self.shacl_format = Some(shacl_format);
        self
    }

    /// Executes the schema serialization operation with the configured parameters.
    pub fn execute(self) -> Result<()> {
        <Rudof as ShaclOperations>::serialize_shacl_schema(self.rudof, self.shacl_format, self.writer)
    }
}
