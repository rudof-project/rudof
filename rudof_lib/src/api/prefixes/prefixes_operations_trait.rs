use crate::{
    Result, Rudof,
    api::prefixes::implementations::{add_prefix, copy_prefix, prefixes, remove_prefix, rename_prefix},
};
use prefixmap::PrefixMap;

/// Operations for managing the default list of prefix declarations.
///
/// These are the prefix declarations assumed and prepended by default to RDF
/// data, SPARQL queries, ShEx schemas and SHACL shapes to facilitate handling
/// prefixed names, independently of whatever prefixes a loaded resource
/// already declares.
pub trait PrefixesOperations {
    /// Returns the current default `PrefixMap` (empty if none has been set).
    fn prefixes(&self) -> PrefixMap;

    /// Adds `alias` associated with `iri` to the default prefixes, creating
    /// the list if this is the first prefix added. Overwrites any existing
    /// association for `alias`.
    fn add_prefix(&mut self, alias: &str, iri: &str) -> Result<()>;

    /// Removes `alias` from the default prefixes.
    ///
    /// # Errors
    ///
    /// Returns an error if `alias` is not present in the default prefixes.
    fn remove_prefix(&mut self, alias: &str) -> Result<()>;

    /// Renames `old_alias` to `new_alias`, keeping the same associated IRI.
    ///
    /// # Errors
    ///
    /// Returns an error if `old_alias` is not present in the default prefixes.
    fn rename_prefix(&mut self, old_alias: &str, new_alias: &str) -> Result<()>;

    /// Adds `new_alias` associated with the same IRI as `old_alias`.
    ///
    /// # Errors
    ///
    /// Returns an error if `old_alias` is not present in the default prefixes.
    fn copy_prefix(&mut self, old_alias: &str, new_alias: &str) -> Result<()>;
}

impl PrefixesOperations for Rudof {
    fn prefixes(&self) -> PrefixMap {
        prefixes(self)
    }

    fn add_prefix(&mut self, alias: &str, iri: &str) -> Result<()> {
        add_prefix(self, alias, iri)
    }

    fn remove_prefix(&mut self, alias: &str) -> Result<()> {
        remove_prefix(self, alias)
    }

    fn rename_prefix(&mut self, old_alias: &str, new_alias: &str) -> Result<()> {
        rename_prefix(self, old_alias, new_alias)
    }

    fn copy_prefix(&mut self, old_alias: &str, new_alias: &str) -> Result<()> {
        copy_prefix(self, old_alias, new_alias)
    }
}
