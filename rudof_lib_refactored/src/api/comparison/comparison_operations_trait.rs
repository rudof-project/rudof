use crate::{
    Result,
    formats::{
        InputSpec, DataReaderMode, ComparisonFormat, ComparisonMode, 
        ResultComparisonFormat
    },
    api::comparison::implementations::show_schema_comparison,
};
use std::io;


/// Comparison operations
pub trait ComparisonOperations {
    /// Compares two schemas and returns their differences in the requested format.
    ///
    /// # Arguments
    ///
    /// * `schema1` - Input specification defining the first schema source
    /// * `schema2` - Input specification defining the second schema source
    /// * `base1` - Optional base IRI for resolving relative IRIs in the first schema (uses default if None)
    /// * `base2` - Optional base IRI for resolving relative IRIs in the second schema (uses default if None)
    /// * `reader_mode` - Optional parsing mode used to read both schemas (uses default if None)
    /// * `format1` - Format of the first schema
    /// * `format2` - Format of the second schema
    /// * `mode1` - Comparison mode applied to the first schema
    /// * `mode2` - Comparison mode applied to the second schema
    /// * `shape1` - Optional shape identifier to focus the comparison in the first schema
    /// * `shape2` - Optional shape identifier to focus the comparison in the second schema
    /// * `show_time` - Whether to include timing information in the comparison output (false by default)
    /// * `result_format` - Optional output format for comparison results (uses default if None)
    /// * `writer` - The destination to write the serialized results to
    ///
    /// # Errors
    ///
    /// Returns an error if schemas cannot be loaded, compared, or serialized.
    fn show_schema_comparison<W: io::Write>(
        &mut self,
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
    ) -> Result<()>;
}

impl ComparisonOperations for crate::Rudof {
    fn show_schema_comparison<W: io::Write>(
        &mut self,
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
        show_schema_comparison(self, schema1, schema2, base1, base2, reader_mode, 
            format1, format2, mode1, mode2, shape1, shape2, show_time, result_format, writer)
    }
}
