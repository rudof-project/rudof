use crate::{Result, Rudof, errors::PrefixesError};
use prefixmap::PrefixMap;

/// Adds `new_alias` associated with the same IRI as `old_alias`, keeping `old_alias` too.
pub fn copy_prefix(rudof: &mut Rudof, old_alias: &str, new_alias: &str) -> Result<()> {
    let pm = rudof.prefixes.as_mut().ok_or_else(|| PrefixesError::AliasNotFound {
        alias: old_alias.to_string(),
    })?;
    let iri = PrefixMap::find(pm, old_alias)
        .cloned()
        .ok_or_else(|| PrefixesError::AliasNotFound {
            alias: old_alias.to_string(),
        })?;
    pm.add_prefix(new_alias, iri);
    Ok(())
}
