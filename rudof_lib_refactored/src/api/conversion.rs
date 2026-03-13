use crate::{
    ConversionOperations, Result,
    formats::{
        InputSpec, DataReaderMode, ConversionMode, ResultConversionMode, ConversionFormat, 
        ResultConversionFormat
    }
};
use std::io;

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
        todo!()
    }
}
