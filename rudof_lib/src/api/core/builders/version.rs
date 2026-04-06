use crate::{Rudof, api::core::CoreOperations};

/// Builder for `version` operation.
///
/// Provides a fluent interface for configuring and executing version checks.
pub struct VersionBuilder<'a> {
    rudof: &'a Rudof,
}

impl<'a> VersionBuilder<'a> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::version()` and should not
    /// be constructed directly.
    pub(crate) fn new(rudof: &'a Rudof) -> Self {
        Self { rudof }
    }

    /// Executes the version check operation.
    ///
    /// This retrieves the version information from the Rudof instance.
    pub fn execute(self) -> &'a str {
        <Rudof as CoreOperations>::version(self.rudof)
    }
}
