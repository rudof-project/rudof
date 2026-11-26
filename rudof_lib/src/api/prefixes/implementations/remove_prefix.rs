use crate::{Result, Rudof, errors::PrefixesError};

pub fn remove_prefix(rudof: &mut Rudof, alias: &str) -> Result<()> {
    let removed = rudof.prefixes.as_mut().and_then(|pm| pm.remove_prefix(alias));
    match removed {
        Some(_) => Ok(()),
        None => Err(PrefixesError::AliasNotFound {
            alias: alias.to_string(),
        }
        .into()),
    }
}
