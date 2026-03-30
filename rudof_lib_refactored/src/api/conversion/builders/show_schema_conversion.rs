use crate::{Rudof, Result, api::conversion::ConversionOperations, formats::{InputSpec, DataReaderMode, ConversionMode, ResultConversionMode, ConversionFormat, ResultConversionFormat}};
use std::io;

/// Builder for `show_schema_conversion` operation.
///
/// Provides a fluent interface for configuring and executing schema conversion
/// operations with optional parameters.
pub struct ShowSchemaConversionBuilder<'a, W: io::Write> {
    rudof: &'a mut Rudof,
    schema: &'a InputSpec,
    input_mode: &'a ConversionMode,
    output_mode: &'a ResultConversionMode,
    input_format: &'a ConversionFormat,
    output_format: &'a ResultConversionFormat,
    writer: &'a mut W,
    base: Option<&'a str>,
    reader_mode: Option<&'a DataReaderMode>,
    shape: Option<&'a str>,
    show_time: Option<bool>,
    templates_folder: Option<&'a std::path::Path>,
    output_folder: Option<&'a std::path::Path>,
}

impl<'a, W: io::Write> ShowSchemaConversionBuilder<'a, W> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::show_schema_conversion()` and should not
    /// be constructed directly.
    pub(crate) fn new(
        rudof: &'a mut Rudof,
        schema: &'a InputSpec,
        input_mode: &'a ConversionMode,
        output_mode: &'a ResultConversionMode,
        input_format: &'a ConversionFormat,
        output_format: &'a ResultConversionFormat,
        writer: &'a mut W,
    ) -> Self {
        Self {
            rudof,
            schema,
            input_mode,
            output_mode,
            input_format,
            output_format,
            writer,
            base: None,
            reader_mode: None,
            shape: None,
            show_time: None,
            templates_folder: None,
            output_folder: None,
        }
    }

    /// Sets the base IRI for resolving relative IRIs.
    ///
    /// # Arguments
    ///
    /// * `base` - The base IRI to use for resolution
    pub fn with_base(mut self, base: &'a str) -> Self {
        self.base = Some(base);
        self
    }

    /// Sets the reader mode for parsing the schema.
    ///
    /// # Arguments
    ///
    /// * `reader_mode` - The reading mode to apply during schema parsing
    pub fn with_reader_mode(mut self, reader_mode: &'a DataReaderMode) -> Self {
        self.reader_mode = Some(reader_mode);
        self
    }

    /// Sets the shape identifier to focus conversion on a specific shape.
    ///
    /// # Arguments
    ///
    /// * `shape` - The shape identifier for focused conversion
    pub fn with_shape(mut self, shape: &'a str) -> Self {
        self.shape = Some(shape);
        self
    }

    /// Sets whether to include timing information.
    ///
    /// # Arguments
    ///
    /// * `show_time` - Whether to include timing information in the output
    pub fn with_show_time(mut self, show_time: bool) -> Self {
        self.show_time = Some(show_time);
        self
    }

    /// Sets the templates folder for conversion (if applicable).
    ///
    /// # Arguments
    ///
    /// * `templates_folder` - Path to a folder containing templates for conversion
    pub fn with_templates_folder(mut self, templates_folder: &'a std::path::Path) -> Self {
        self.templates_folder = Some(templates_folder);
        self
    }

    /// Sets the output folder for writing conversion results.
    ///
    /// # Arguments
    ///
    /// * `output_folder` - Path to a folder where output files should be written
    pub fn with_output_folder(mut self, output_folder: &'a std::path::Path) -> Self {
        self.output_folder = Some(output_folder);
        self
    }

    /// Executes the schema conversion operation with the configured parameters.
    ///
    /// # Errors
    ///
    /// Returns an error if the schema cannot be loaded, converted, or serialized.
    pub fn execute(self) -> Result<()> {
        <Rudof as ConversionOperations>::show_schema_conversion(
            self.rudof,
            self.schema,
            self.base,
            self.reader_mode,
            self.input_mode,
            self.output_mode,
            self.input_format,
            self.output_format,
            self.shape,
            self.show_time,
            self.templates_folder,
            self.output_folder,
            self.writer,
        )
    }
}
