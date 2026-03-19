use crate::{Rudof, Result, api::dctap::DctapOperations, formats::{InputSpec, DCTapFormat}};

/// Builder for `load_dctap` operation.
///
/// Provides a fluent interface for configuring and executing DC-TAP loading
/// operations with optional parameters.
pub struct LoadDctapBuilder<'a> {
    rudof: &'a mut Rudof,
    dctap: &'a InputSpec,
    dctap_format: Option<&'a DCTapFormat>,
}

impl<'a> LoadDctapBuilder<'a> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::load_dctap()` and should not
    /// be constructed directly.
    pub(crate) fn new(rudof: &'a mut Rudof, dctap: &'a InputSpec) -> Self {
        Self {
            rudof,
            dctap,
            dctap_format: None,
        }
    }

    /// Sets the DC-TAP format for loading.
    ///
    /// # Arguments
    ///
    /// * `dctap_format` - The format to use when loading the DC-TAP profile
    pub fn with_dctap_format(mut self, dctap_format: &'a DCTapFormat) -> Self {
        self.dctap_format = Some(dctap_format);
        self
    }

    /// Executes the DC-TAP loading operation with the configured parameters.
    ///
    /// # Errors
    ///
    /// Returns an error if the DC-TAP profile cannot be loaded or parsed.
    pub fn execute(self) -> Result<()> {
        <Rudof as DctapOperations>::load_dctap(self.rudof, self.dctap, self.dctap_format)
    }
}
