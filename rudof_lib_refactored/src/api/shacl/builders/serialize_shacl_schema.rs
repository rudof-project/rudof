use crate::{Rudof, Result, api::shacl::ShaclOperations, formats::ShaclFormat};
use std::io;

/// Builder for `serialize_shacl_schema` operation.
///
/// Provides a fluent interface for configuring and executing schema serialization
/// operations with optional parameters.
pub struct SerializeShaclSchemaBuilder<'a, W: io::Write> {
    rudof: &'a Rudof,
    writer: &'a mut W,
    format: Option<&'a ShaclFormat>,
}

impl<'a, W: io::Write> SerializeShaclSchemaBuilder<'a, W> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::serialize_shacl_schema()` and should not
    /// be constructed directly.
    pub(crate) fn new(rudof: &'a Rudof, writer: &'a mut W) -> Self {
        Self {
            rudof,
            writer,
            format: None,
        }
    }

    /// Sets the output format for schema serialization.
    ///
    /// # Arguments
    ///
    /// * `format` - The format to use when serializing the schema
    pub fn with_schema_format(mut self, format: &'a ShaclFormat) -> Self {
        self.format = Some(format);
        self
    }

    /// Executes the schema serialization operation with the configured parameters.
    ///
    /// # Errors
    ///
    /// Returns an error if no schema is loaded or serialization fails.
    pub fn execute(self) -> Result<()> {
        <Rudof as ShaclOperations>::serialize_shacl_schema(
            self.rudof,
            self.format,
            self.writer,
        )
    }
}
