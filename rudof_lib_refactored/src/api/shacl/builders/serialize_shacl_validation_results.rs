use crate::{Rudof, Result, api::shacl::ShaclOperations, formats::{ShaclValidationSortByMode, ResultShaclValidationFormat}};
use std::io;

/// Builder for `serialize_shacl_validation_results` operation.
///
/// Provides a fluent interface for configuring and executing SHACL validation results
/// serialization operations with optional parameters.
pub struct SerializeShaclValidationResultsBuilder<'a, W: io::Write> {
    rudof: &'a Rudof,
    writer: &'a mut W,
    shacl_validation_sort_order_mode: Option<&'a ShaclValidationSortByMode>,
    result_shacl_validation_format: Option<&'a ResultShaclValidationFormat>,
}

impl<'a, W: io::Write> SerializeShaclValidationResultsBuilder<'a, W> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::serialize_shacl_validation_results()` and should not
    /// be constructed directly.
    pub(crate) fn new(rudof: &'a Rudof, writer: &'a mut W) -> Self {
        Self {
            rudof,
            writer,
            shacl_validation_sort_order_mode: None,
            result_shacl_validation_format: None,
        }
    }

    /// Sets the sorting mode for validation results.
    ///
    /// # Arguments
    ///
    /// * `shacl_validation_sort_order_mode` - The sorting mode to apply to validation results
    pub fn with_shacl_validation_sort_order_mode(mut self, shacl_validation_sort_order_mode: &'a ShaclValidationSortByMode) -> Self {
        self.shacl_validation_sort_order_mode = Some(shacl_validation_sort_order_mode);
        self
    }

    /// Sets the output format for validation results.
    ///
    /// # Arguments
    /// 
    /// * `result_shacl_validation_format` - The format in which to serialize validation results
    pub fn with_result_shacl_validation_format(mut self, result_shacl_validation_format: &'a ResultShaclValidationFormat) -> Self {
        self.result_shacl_validation_format = Some(result_shacl_validation_format);
        self
    }

    /// Executes the validation results serialization operation with the configured parameters.
    ///
    /// # Errors
    ///
    /// Returns an error if no validation results are available.
    pub fn execute(self) -> Result<()> {
        <Rudof as ShaclOperations>::serialize_shacl_validation_results(
            self.rudof,
            self.shacl_validation_sort_order_mode,
            self.result_shacl_validation_format,
            self.writer,
        )
    }
}
