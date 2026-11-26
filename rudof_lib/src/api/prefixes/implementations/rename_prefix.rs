use crate::{Result, Rudof, errors::PrefixesError};

/// Renames `old_alias` to `new_alias`, keeping the same associated IRI.
///
/// If `new_alias` was already in use, its previous association is overwritten,
/// mirroring the overwrite behaviour of [`prefixmap::PrefixMap::add_prefix`].
pub fn rename_prefix(rudof: &mut Rudof, old_alias: &str, new_alias: &str) -> Result<()> {
    let pm = rudof.prefixes.as_mut().ok_or_else(|| PrefixesError::AliasNotFound {
        alias: old_alias.to_string(),
    })?;
    let iri = pm
        .remove_prefix(old_alias)
        .ok_or_else(|| PrefixesError::AliasNotFound {
            alias: old_alias.to_string(),
        })?;
    pm.add_prefix(new_alias, iri);
    Ok(())
}
