use crate::{Result, Rudof, api::prefixes::PrefixesOperations};

/// Builder for `remove_prefix` operation.
///
/// Provides a fluent interface for removing an alias from the default prefixes.
pub struct RemovePrefixBuilder<'a> {
    rudof: &'a mut Rudof,
    alias: &'a str,
}

impl<'a> RemovePrefixBuilder<'a> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::remove_prefix()` and should not
    /// be constructed directly.
    pub(crate) fn new(rudof: &'a mut Rudof, alias: &'a str) -> Self {
        Self { rudof, alias }
    }

    /// Executes the operation.
    pub fn execute(self) -> Result<()> {
        <Rudof as PrefixesOperations>::remove_prefix(self.rudof, self.alias)
    }
}
