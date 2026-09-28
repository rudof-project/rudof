use crate::{api::PyRudof, error::Result};
use pyo3::prelude::*;

#[cfg_attr(feature = "stub-gen", pyo3_stub_gen_derive::gen_stub_pymethods)]
#[pymethods]
impl PyRudof {
    /// Returns the current default prefix map.
    ///
    /// These are the prefix declarations assumed and prepended by default to RDF data,
    /// SPARQL queries, ShEx schemas and SHACL shapes.
    ///
    /// Returns:
    ///     list[tuple[str, str]]: ``(alias, iri)`` tuples.
    fn prefixes(&self) -> Vec<(String, String)> {
        self.inner
            .prefixes()
            .execute()
            .iter()
            .map(|(alias, iri)| (alias.clone(), iri.to_string()))
            .collect()
    }

    /// Adds a prefix declaration to the default prefix map.
    ///
    /// Args:
    ///     alias (str): The prefix alias, without the trailing colon.
    ///     iri (str): The IRI the alias expands to.
    ///
    /// Raises:
    ///     PrefixesError: If the alias is already declared or the IRI is malformed.
    fn add_prefix(&mut self, alias: &str, iri: &str) -> Result<()> {
        self.inner.add_prefix(alias, iri).execute()?;
        Ok(())
    }

    /// Removes a prefix declaration from the default prefix map.
    ///
    /// Args:
    ///     alias (str): The prefix alias to remove.
    ///
    /// Raises:
    ///     PrefixesError: If the alias is not declared.
    fn remove_prefix(&mut self, alias: &str) -> Result<()> {
        self.inner.remove_prefix(alias).execute()?;
        Ok(())
    }

    /// Renames a prefix alias, keeping its IRI.
    ///
    /// Args:
    ///     old_alias (str): The alias to rename.
    ///     new_alias (str): The new alias.
    ///
    /// Raises:
    ///     PrefixesError: If ``old_alias`` is not declared or ``new_alias`` already is.
    fn rename_prefix(&mut self, old_alias: &str, new_alias: &str) -> Result<()> {
        self.inner.rename_prefix(old_alias, new_alias).execute()?;
        Ok(())
    }

    /// Declares ``new_alias`` for the same IRI as ``old_alias``, keeping both.
    ///
    /// Args:
    ///     old_alias (str): The alias to copy.
    ///     new_alias (str): The new alias.
    ///
    /// Raises:
    ///     PrefixesError: If ``old_alias`` is not declared or ``new_alias`` already is.
    fn copy_prefix(&mut self, old_alias: &str, new_alias: &str) -> Result<()> {
        self.inner.copy_prefix(old_alias, new_alias).execute()?;
        Ok(())
    }
}
