use crate::{Rudof, Result, api::shex::ShExOperations, formats::{ShExValidationSortByMode, ResultShExValidationFormat}};
use std::io;

/// Builder for `serialize_shex_validation_results` operation.
///
/// Provides a fluent interface for configuring and executing ShEx validation results
/// serialization operations with optional parameters.
pub struct SerializeShexValidationResultsBuilder<'a, W: io::Write> {
    rudof: &'a Rudof,
    writer: &'a mut W,
    sort_order: Option<&'a ShExValidationSortByMode>,
    result_format: Option<&'a ResultShExValidationFormat>,
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
            sort_order: None,
            result_format: None,
        }
    }

    /// Sets the sorting mode for validation results.
    ///
    /// # Arguments
    ///
    /// * `sort_order` - The sorting mode to apply to validation results
    pub fn with_sort_order(mut self, sort_order: &'a ShExValidationSortByMode) -> Self {
        self.sort_order = Some(sort_order);
        self
    }

    /// Sets the output format for validation results.
    /// 
    /// # Arguments
    /// 
    /// * `result_format` - The format in which to serialize validation results
    pub fn with_result_format(mut self, result_format: &'a ResultShExValidationFormat) -> Self {
        self.result_format = Some(result_format);
        self
    }

    /// Executes the validation results serialization operation with the configured parameters.
    ///
    /// # Errors
    ///
    /// Returns an error if no validation results are available.
    pub fn execute(self) -> Result<()> {
        <Rudof as ShExOperations>::serialize_shex_validation_results(
            self.rudof,
            self.sort_order,
            self.result_format,
            self.writer,
        )
    }
}
