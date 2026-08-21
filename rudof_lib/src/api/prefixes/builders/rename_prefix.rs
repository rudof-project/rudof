use crate::{Result, Rudof, api::prefixes::PrefixesOperations};

/// Builder for `rename_prefix` operation.
///
/// Provides a fluent interface for renaming an alias in the default prefixes,
/// keeping its associated IRI.
pub struct RenamePrefixBuilder<'a> {
    rudof: &'a mut Rudof,
    old_alias: &'a str,
    new_alias: &'a str,
}

impl<'a> RenamePrefixBuilder<'a> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::rename_prefix()` and should not
    /// be constructed directly.
    pub(crate) fn new(rudof: &'a mut Rudof, old_alias: &'a str, new_alias: &'a str) -> Self {
        Self {
            rudof,
            old_alias,
            new_alias,
        }
    }

    /// Executes the operation.
    pub fn execute(self) -> Result<()> {
        <Rudof as PrefixesOperations>::rename_prefix(self.rudof, self.old_alias, self.new_alias)
    }
}
