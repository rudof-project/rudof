use crate::{
    Result,
    formats::{
        InputSpec, DataReaderMode, ConversionMode, ResultConversionMode, ConversionFormat, 
        ResultConversionFormat
    },
    api::conversion::implementations::show_schema_conversion
};
use std::io;

/// Conversion operations
pub trait ConversionOperations {
    /// Converts a schema from one format to another.
    ///
    /// # Arguments
    ///
    /// * `schema` - Input specification defining the schema source
    /// * `base` - Optional base IRI for resolving relative IRIs (uses default if None)
    /// * `reader_mode` - Optional parsing mode used to read the schema (uses default if None)
    /// * `input_mode` - The conversion mode for interpreting the input schema
    /// * `output_mode` - The conversion mode for generating the output schema
    /// * `input_format` - Format of the input schema
    /// * `output_format` - Format of the output schema
    /// * `shape` - Optional shape identifier to focus the conversion on a specific shape
    /// * `show_time` - Whether to include timing information in the conversion output (false by default)
    /// * `writer` - The destination to write the converted schema to
    ///
    /// # Errors
    ///
    /// Returns an error if the schema cannot be loaded, converted, or serialized.
    fn show_schema_conversion<W: io::Write>(
        &self,
        schema: &InputSpec,
        base: Option<&str>,
        reader_mode: Option<&DataReaderMode>,
        input_mode: &ConversionMode,
        output_mode: &ResultConversionMode,
        input_format: &ConversionFormat,
        output_format: &ResultConversionFormat,
        shape: Option<&str>,
        show_time: Option<bool>,
        writer: &mut W,
    ) -> Result<()>;
}

impl ConversionOperations for crate::Rudof {
    fn show_schema_conversion<W: io::Write>(
        &self,
        schema: &InputSpec,
        base: Option<&str>,
        reader_mode: Option<&DataReaderMode>,
        input_mode: &ConversionMode,
        output_mode: &ResultConversionMode,
        input_format: &ConversionFormat,
        output_format: &ResultConversionFormat,
        shape: Option<&str>,
        show_time: Option<bool>,
        writer: &mut W,
    ) -> Result<()> {
        show_schema_conversion(self, schema, base, reader_mode, input_mode, output_mode,
            input_format, output_format, shape, show_time, writer)
    }
}
