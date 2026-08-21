use crate::{Result, Rudof, api::prefixes::PrefixesOperations};

/// Builder for `add_prefix` operation.
///
/// Provides a fluent interface for adding an alias/IRI association to the
/// default prefixes.
pub struct AddPrefixBuilder<'a> {
    rudof: &'a mut Rudof,
    alias: &'a str,
    iri: &'a str,
}

impl<'a> AddPrefixBuilder<'a> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::add_prefix()` and should not
    /// be constructed directly.
    pub(crate) fn new(rudof: &'a mut Rudof, alias: &'a str, iri: &'a str) -> Self {
        Self { rudof, alias, iri }
    }

    /// Executes the operation.
    pub fn execute(self) -> Result<()> {
        <Rudof as PrefixesOperations>::add_prefix(self.rudof, self.alias, self.iri)
    }
}
