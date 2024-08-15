use dctap::{PrefixCC, TapConfig};
use iri_s::{iri, IriS};
use prefixmap::PrefixMap;
use serde_derive::{Deserialize, Serialize};

use super::Tap2ShExError;

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct Tap2ShExConfig {
    pub base_iri: Option<IriS>,
    pub datatype_base_iri: Option<IriS>,
    prefixmap: Option<PrefixMap>,

    dctap: Option<TapConfig>,

    #[serde(skip)]
    prefix_cc: Option<PrefixCC>,
}

impl Tap2ShExConfig {
    pub fn prefixmap(&self) -> PrefixMap {
        match &self.prefixmap {
            Some(pm) => pm.clone(),
            None => PrefixMap::basic(),
        }
    }

    // TOOD: Refactor Tap2ShExError to reduce its size and avoid the result_large_err warning
    #[allow(clippy::result_large_err)]
    pub fn resolve_iri(&self, str: &str, line: u64) -> Result<IriS, Tap2ShExError> {
        if let Some((prefix, localname)) = prefix_local_name(str) {
            match self
                .prefixmap()
                .resolve_prefix_local(prefix.as_str(), localname.as_str())
            {
                Ok(iri) => Ok(iri),
                Err(e) => {
                    if prefix.is_empty() {
                        match &self.base_iri {
                            None => Err(Tap2ShExError::IriNoPrefix {
                                str: str.to_string(),
                                line,
                            }),
                            Some(base_iri) => base_iri
                                .extend(localname.as_str())
                                .map_err(|e| Tap2ShExError::IriSError { err: e }),
                        }
                    } else {
                        // TODO: Match with prefix_cc
                        Err(Tap2ShExError::ResolvingPrefixError {
                            err: e,
                            line,
                            field: str.to_string(),
                        })
                    }
                }
            }
        } else {
            let iri = match &self.base_iri {
                None => Err(Tap2ShExError::IriNoPrefix {
                    str: str.to_string(),
                    line,
                }),
                Some(base_iri) => base_iri
                    .extend(str)
                    .map_err(|e| Tap2ShExError::IriSError { err: e }),
            }?;
            Ok(iri)
        }
    }
}

pub fn prefix_local_name(str: &str) -> Option<(String, String)> {
    // TODO: Check how to escape special characters
    if let Some((prefix, localname)) = str.rsplit_once(':') {
        Some((prefix.to_string(), localname.to_string()))
    } else {
        None
    }
}

impl Default for Tap2ShExConfig {
    fn default() -> Self {
        Self {
            base_iri: Some(iri!("http://example.org/")),
            datatype_base_iri: None,
            dctap: None,
            prefixmap: Some(PrefixMap::basic()),
            prefix_cc: None,
        }
    }
}
