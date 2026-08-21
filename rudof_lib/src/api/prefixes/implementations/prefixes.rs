use crate::Rudof;
use prefixmap::PrefixMap;

pub fn prefixes(rudof: &Rudof) -> PrefixMap {
    rudof.prefixes.clone().unwrap_or_default()
}
