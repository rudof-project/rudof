use crate::{
    Result, Rudof,
    api::shex::ShExOperations,
    formats::{ResultShExValidationFormat, ShExDetailsLayout, ShExValidationSortByMode},
};
use std::io;

/// Builder for `serialize_shex_validation_results` operation.
///
/// Provides a fluent interface for configuring and executing ShEx validation results
/// serialization operations with optional parameters.
pub struct SerializeShexValidationResultsBuilder<'a, W: io::Write> {
    rudof: &'a Rudof,
    writer: &'a mut W,
    shex_validation_sort_order_mode: Option<&'a ShExValidationSortByMode>,
    result_shex_validation_format: Option<&'a ResultShExValidationFormat>,
    details_layout: Option<&'a ShExDetailsLayout>,
}

impl<'a, W: io::Write> SerializeShexValidationResultsBuilder<'a, W> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::serialize_shex_validation_results()` and should not
    /// be constructed directly.
    pub(crate) fn new(rudof: &'a Rudof, writer: &'a mut W) -> Self {
        Self {
            rudof,
            writer,
            shex_validation_sort_order_mode: None,
            result_shex_validation_format: None,
            details_layout: None,
        }
    }

    /// Sets the sorting mode for validation results.
    ///
    /// # Arguments
    ///
    /// * `shex_validation_sort_order_mode` - The sorting mode to apply to validation results
    pub fn with_shex_validation_sort_order_mode(
        mut self,
        shex_validation_sort_order_mode: &'a ShExValidationSortByMode,
    ) -> Self {
        self.shex_validation_sort_order_mode = Some(shex_validation_sort_order_mode);
        self
    }

    /// Sets the output format for validation results.
    ///
    /// # Arguments
    ///
    /// * `result_shex_validation_format` - The format in which to serialize validation results
    pub fn with_result_shex_validation_format(
        mut self,
        result_shex_validation_format: &'a ResultShExValidationFormat,
    ) -> Self {
        self.result_shex_validation_format = Some(result_shex_validation_format);
        self
    }

    /// Sets where the `details` format writes the details of each result: below the table, with a
    /// summary in each row (the default), or all of them in the table.
    ///
    /// # Arguments
    ///
    /// * `details_layout` - Where the details go
    pub fn with_shex_details_layout(mut self, details_layout: &'a ShExDetailsLayout) -> Self {
        self.details_layout = Some(details_layout);
        self
    }

    /// Executes the validation results serialization operation with the configured parameters.
    pub fn execute(self) -> Result<()> {
        <Rudof as ShExOperations>::serialize_shex_validation_results(
            self.rudof,
            self.shex_validation_sort_order_mode,
            self.result_shex_validation_format,
            self.details_layout,
            self.writer,
        )
    }
}
