use crate::{
    Result, Rudof,
    api::shacl::ShaclOperations,
    formats::{ShaclFetchStrategy, ShaclValidationMode},
};

/// Builder for `validate_shacl` operation.
///
/// Provides a fluent interface for configuring and executing SHACL validation
/// operations with optional parameters.
pub struct ValidateShaclBuilder<'a> {
    rudof: &'a mut Rudof,
    mode: Option<&'a ShaclValidationMode>,
    fetch_strategy: Option<ShaclFetchStrategy>,
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
            fetch_strategy: None,
        }
    }

    /// Sets the validation mode.
    ///
    /// # Arguments
    ///
    /// * `mode` - The validation mode to use
    pub fn with_shacl_validation_mode(mut self, mode: &'a ShaclValidationMode) -> Self {
        self.mode = Some(mode);
        self
    }

    /// Sets how the data of a SPARQL endpoint is fetched, overriding the
    /// configured `[shacl] fetch_strategy` for this validation. No effect on
    /// data in memory.
    ///
    /// # Arguments
    ///
    /// * `strategy` - `OnDemand` fetches each focus node's data when the
    ///   validation needs it; `ByShape` fetches the data shape by shape
    ///   before validating, many nodes per query
    pub fn with_fetch_strategy(mut self, strategy: ShaclFetchStrategy) -> Self {
        self.fetch_strategy = Some(strategy);
        self
    }

    /// Executes the SHACL validation operation with the configured parameters.
    pub fn execute(self) -> Result<()> {
        match self.fetch_strategy {
            None => <Rudof as ShaclOperations>::validate_shacl(self.rudof, self.mode),
            Some(strategy) => {
                crate::api::shacl::implementations::validate_shacl_with(self.rudof, self.mode, Some(strategy))
            },
        }
    }
}
