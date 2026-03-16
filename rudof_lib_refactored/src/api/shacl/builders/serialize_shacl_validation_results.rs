use crate::{Rudof, Result, api::shacl::ShaclOperations, formats::ShaclValidationSortByMode};
use std::io;

/// Builder for `serialize_shacl_validation_results` operation.
///
/// Provides a fluent interface for configuring and executing SHACL validation results
/// serialization operations with optional parameters.
pub struct SerializeShaclValidationResultsBuilder<'a, W: io::Write> {
    rudof: &'a Rudof,
    writer: &'a mut W,
    sort_order: Option<&'a ShaclValidationSortByMode>,
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
            sort_order: None,
        }
    }

    /// Sets the sorting mode for validation results.
    ///
    /// # Arguments
    ///
    /// * `sort_order` - The sorting mode to apply to validation results
    pub fn with_sort_order(mut self, sort_order: &'a ShaclValidationSortByMode) -> Self {
        self.sort_order = Some(sort_order);
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
            self.sort_order,
            self.writer,
        )
    }
}
