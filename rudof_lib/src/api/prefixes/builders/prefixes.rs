use crate::{Rudof, api::prefixes::PrefixesOperations};
use prefixmap::PrefixMap;

/// Builder for `prefixes` operation.
///
/// Provides a fluent interface for retrieving the current default `PrefixMap`.
pub struct PrefixesBuilder<'a> {
    rudof: &'a Rudof,
}

impl<'a> PrefixesBuilder<'a> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::prefixes()` and should not
    /// be constructed directly.
    pub(crate) fn new(rudof: &'a Rudof) -> Self {
        Self { rudof }
    }

    /// Executes the operation, returning the current default `PrefixMap`
    /// (empty if none has been set).
    pub fn execute(self) -> PrefixMap {
        <Rudof as PrefixesOperations>::prefixes(self.rudof)
    }
}
