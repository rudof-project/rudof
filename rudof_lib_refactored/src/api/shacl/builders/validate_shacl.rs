use crate::{Rudof, Result, api::shacl::ShaclOperations, formats::ShaclValidationMode};

/// Builder for `validate_shacl` operation.
///
/// Provides a fluent interface for configuring and executing SHACL validation
/// operations with optional parameters.
pub struct ValidateShaclBuilder<'a> {
    rudof: &'a mut Rudof,
    mode: Option<&'a ShaclValidationMode>,
}

impl<'a> ValidateShaclBuilder<'a> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::validate_shacl()` and should not
    /// be constructed directly.
    pub(crate) fn new(rudof: &'a mut Rudof) -> Self {
        Self {
            rudof,
            mode: None,
        }
    }

    /// Sets the validation mode.
    ///
    /// # Arguments
    ///
    /// * `mode` - The validation mode to use
    pub fn with_mode(mut self, mode: &'a ShaclValidationMode) -> Self {
        self.mode = Some(mode);
        self
    }

    /// Executes the SHACL validation operation with the configured parameters.
    ///
    /// # Errors
    ///
    /// Returns an error if no SHACL schema or shapes is loaded.
    pub fn execute(self) -> Result<()> {
        <Rudof as ShaclOperations>::validate_shacl(self.rudof, self.mode)
    }
}
