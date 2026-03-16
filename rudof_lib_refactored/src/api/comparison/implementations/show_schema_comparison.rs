use crate::{
    Rudof, Result, 
    formats::{
        InputSpec, DataReaderMode, ComparisonFormat, ComparisonMode, ResultComparisonFormat
    }
};
use std::io;

pub fn show_schema_comparison<W: io::Write>(
    rudof: &Rudof,
    schema1: &InputSpec,
    schema2: &InputSpec,
    base1: Option<&str>,
    base2: Option<&str>,
    reader_mode: Option<&DataReaderMode>,
    format1: &ComparisonFormat,
    format2: &ComparisonFormat,
    mode1: &ComparisonMode,
    mode2: &ComparisonMode,
    shape1: Option<&str>,
    shape2: Option<&str>,
    show_time: Option<bool>,
    result_format: Option<&ResultComparisonFormat>,
    writer: &mut W,
) -> Result<()> {
    todo!()
}
