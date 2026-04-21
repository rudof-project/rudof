use crate::{Rudof, api::dctap::DctapOperations};

/// Builder for `reset_dctap` operation.
///
/// Provides a fluent interface for configuring and executing DC-TAP reset
/// operations.
pub struct ResetDctapBuilder<'a> {
    rudof: &'a mut Rudof,
}

impl<'a> ResetDctapBuilder<'a> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::reset_dctap()` and should not
    /// be constructed directly.
    pub(crate) fn new(rudof: &'a mut Rudof) -> Self {
        Self { rudof }
    }

    /// Executes the DC-TAP reset operation.
    pub fn execute(self) {
        <Rudof as DctapOperations>::reset_dctap(self.rudof)
    }
}
