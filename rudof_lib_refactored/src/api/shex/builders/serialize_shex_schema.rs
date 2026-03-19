use crate::{Rudof, Result, api::shex::ShExOperations, formats::ShExFormat};
use std::io;

/// Builder for `serialize_shex_schema` operation.
///
/// Provides a fluent interface for configuring and executing schema serialization
/// operations with optional parameters.
pub struct SerializeShexSchemaBuilder<'a, W: io::Write> {
    rudof: &'a Rudof,
    writer: &'a mut W,
    shape_label: Option<&'a str>,
    show_schema: Option<bool>,
    show_statistics: Option<bool>,
    show_dependencies: Option<bool>,
    show_time: Option<bool>,
    shex_format: Option<&'a ShExFormat>,
}

impl<'a, W: io::Write> SerializeShexSchemaBuilder<'a, W> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::serialize_shex_schema()` and should not
    /// be constructed directly.
    pub(crate) fn new(rudof: &'a Rudof, writer: &'a mut W) -> Self {
        Self {
            rudof,
            writer,
            shape_label: None,
            show_schema: None,
            show_statistics: None,
            show_dependencies: None,
            show_time: None,
            shex_format: None,
        }
    }

    /// Sets a specific shape label to serialize.
    ///
    /// # Arguments
    ///
    /// * `shape_label` - The shape label to serialize (serializes entire schema if None)
    pub fn with_shape(mut self, shape_label: &'a str) -> Self {
        self.shape_label = Some(shape_label);
        self
    }

    /// Sets whether to include the schema in the output.
    ///
    /// # Arguments
    ///
    /// * `show_schema` - Whether to include the schema (true by default)
    pub fn with_show_schema(mut self, show_schema: bool) -> Self {
        self.show_schema = Some(show_schema);
        self
    }

    /// Sets whether to include statistics in the output.
    ///
    /// # Arguments
    ///
    /// * `show_statistics` - Whether to include statistics (false by default)
    pub fn with_show_statistics(mut self, show_statistics: bool) -> Self {
        self.show_statistics = Some(show_statistics);
        self
    }

    /// Sets whether to show shape dependencies.
    ///
    /// # Arguments
    ///
    /// * `show_dependencies` - Whether to show dependencies (false by default)
    pub fn with_show_dependencies(mut self, show_dependencies: bool) -> Self {
        self.show_dependencies = Some(show_dependencies);
        self
    }

    /// Sets whether to include timing information.
    ///
    /// # Arguments
    ///
    /// * `show_time` - Whether to include timing information (false by default)
    pub fn with_show_time(mut self, show_time: bool) -> Self {
        self.show_time = Some(show_time);
        self
    }

    /// Sets the format for the result schema.
    /// 
    /// # Arguments
    ///
    /// * `shex_format` - The format to serialize the result schema
    pub fn with_result_shex_format(mut self, shex_format: &'a ShExFormat) -> Self {
        self.shex_format = Some(shex_format);
        self
    }

    /// Executes the schema serialization operation with the configured parameters.
    ///
    /// # Errors
    ///
    /// Returns an error if no schema is loaded or serialization fails.
    pub fn execute(self) -> Result<()> {
        <Rudof as ShExOperations>::serialize_shex_schema(
            self.rudof,
            self.shape_label,
            self.show_schema,
            self.show_statistics,
            self.show_dependencies,
            self.show_time,
            self.shex_format,
            self.writer,
        )
    }
}
