use crate::{Result, Rudof, errors::IriError};
use prefixmap::PrefixMap;
use rudof_iri::IriS;
use std::str::FromStr;

pub fn add_prefix(rudof: &mut Rudof, alias: &str, iri: &str) -> Result<()> {
    let iri = IriS::from_str(iri).map_err(|error| IriError::ParseError {
        iri: iri.to_string(),
        error: error.to_string(),
    })?;
    rudof.prefixes.get_or_insert_with(PrefixMap::new).add_prefix(alias, iri);
    Ok(())
}
