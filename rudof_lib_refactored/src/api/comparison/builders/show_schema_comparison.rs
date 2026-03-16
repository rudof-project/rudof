use crate::{Rudof, Result, api::comparison::ComparisonOperations, formats::{InputSpec, DataReaderMode, ComparisonFormat, ComparisonMode, ResultComparisonFormat}};
use std::io;

/// Builder for `show_schema_comparison` operation.
///
/// Provides a fluent interface for configuring and executing schema comparison
/// operations with optional parameters.
pub struct ShowSchemaComparisonBuilder<'a, W: io::Write> {
    rudof: &'a Rudof,
    schema1: &'a InputSpec,
    schema2: &'a InputSpec,
    format1: &'a ComparisonFormat,
    format2: &'a ComparisonFormat,
    mode1: &'a ComparisonMode,
    mode2: &'a ComparisonMode,
    writer: &'a mut W,
    base1: Option<&'a str>,
    base2: Option<&'a str>,
    reader_mode: Option<&'a DataReaderMode>,
    shape1: Option<&'a str>,
    shape2: Option<&'a str>,
    show_time: Option<bool>,
    result_format: Option<&'a ResultComparisonFormat>,
}

impl<'a, W: io::Write> ShowSchemaComparisonBuilder<'a, W> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::show_schema_comparison()` and should not
    /// be constructed directly.
    pub(crate) fn new(
        rudof: &'a Rudof,
        schema1: &'a InputSpec,
        schema2: &'a InputSpec,
        format1: &'a ComparisonFormat,
        format2: &'a ComparisonFormat,
        mode1: &'a ComparisonMode,
        mode2: &'a ComparisonMode,
        writer: &'a mut W,
    ) -> Self {
        Self {
            rudof,
            schema1,
            schema2,
            format1,
            format2,
            mode1,
            mode2,
            writer,
            base1: None,
            base2: None,
            reader_mode: None,
            shape1: None,
            shape2: None,
            show_time: None,
            result_format: None,
        }
    }

    /// Sets the base IRI for the first schema.
    ///
    /// # Arguments
    ///
    /// * `base1` - The base IRI for resolving relative IRIs in the first schema
    pub fn with_base1(mut self, base1: &'a str) -> Self {
        self.base1 = Some(base1);
        self
    }

    /// Sets the base IRI for the second schema.
    ///
    /// # Arguments
    ///
    /// * `base2` - The base IRI for resolving relative IRIs in the second schema
    pub fn with_base2(mut self, base2: &'a str) -> Self {
        self.base2 = Some(base2);
        self
    }

    /// Sets the reader mode for parsing both schemas.
    ///
    /// # Arguments
    ///
    /// * `reader_mode` - The reading mode to apply during schema parsing
    pub fn with_reader_mode(mut self, reader_mode: &'a DataReaderMode) -> Self {
        self.reader_mode = Some(reader_mode);
        self
    }

    /// Sets the shape identifier to focus comparison in the first schema.
    ///
    /// # Arguments
    ///
    /// * `shape1` - The shape identifier for the first schema
    pub fn with_shape1(mut self, shape1: &'a str) -> Self {
        self.shape1 = Some(shape1);
        self
    }

    /// Sets the shape identifier to focus comparison in the second schema.
    ///
    /// # Arguments
    ///
    /// * `shape2` - The shape identifier for the second schema
    pub fn with_shape2(mut self, shape2: &'a str) -> Self {
        self.shape2 = Some(shape2);
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

    /// Sets the output format for comparison results.
    ///
    /// # Arguments
    ///
    /// * `result_format` - The format to use when serializing the comparison results
    pub fn with_result_format(mut self, result_format: &'a ResultComparisonFormat) -> Self {
        self.result_format = Some(result_format);
        self
    }

    /// Executes the schema comparison operation with the configured parameters.
    ///
    /// # Errors
    ///
    /// Returns an error if schemas cannot be loaded, compared, or serialized.
    pub fn execute(self) -> Result<()> {
        <Rudof as ComparisonOperations>::show_schema_comparison(
            self.rudof,
            self.schema1,
            self.schema2,
            self.base1,
            self.base2,
            self.reader_mode,
            self.format1,
            self.format2,
            self.mode1,
            self.mode2,
            self.shape1,
            self.shape2,
            self.show_time,
            self.result_format,
            self.writer,
        )
    }
}
