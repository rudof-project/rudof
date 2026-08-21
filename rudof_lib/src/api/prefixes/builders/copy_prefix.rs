use crate::{Result, Rudof, api::prefixes::PrefixesOperations};

/// Builder for `copy_prefix` operation.
///
/// Provides a fluent interface for adding a new alias associated with the
/// same IRI as an existing one in the default prefixes.
pub struct CopyPrefixBuilder<'a> {
    rudof: &'a mut Rudof,
    old_alias: &'a str,
    new_alias: &'a str,
}

impl<'a> CopyPrefixBuilder<'a> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::copy_prefix()` and should not
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
        <Rudof as PrefixesOperations>::copy_prefix(self.rudof, self.old_alias, self.new_alias)
    }
}
