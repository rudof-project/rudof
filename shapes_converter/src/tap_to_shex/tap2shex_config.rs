use dctap::{TapConfig};
use serde::{Deserialize, Serialize};
use prefixmap::PrefixMap;
use rudof_iri::IriS;

use super::Tap2ShExError;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Tap2ShExConfig {
    #[serde(rename = "base_iri", skip_serializing_if = "Option::is_none")]
    pub(crate) base_iri: Option<IriS>,
    #[serde(rename = "datatype_base_iri", skip_serializing_if = "Option::is_none")]
    pub(crate) datatype_base_iri: Option<IriS>,
    #[serde(rename = "prefixmap", skip_serializing_if = "PrefixMap::is_empty")]
    pub(crate) prefixmap: PrefixMap,
    #[serde(rename = "dctap", skip_serializing)]
    pub(crate) dctap: TapConfig,

    // TODO - Can we remove this and use the prefix map?
    // #[serde(skip)]
    // prefix_cc: Option<PrefixCC>,
}

impl Tap2ShExConfig {
    pub fn new() -> Self {
        Self {
            base_iri: None,
            datatype_base_iri: Self::default_datatype_base_iri(),
            prefixmap: Self::default_prefixmap(),
            dctap: Self::default_dctap(),
            // prefix_cc: None,
        }
    }

    pub fn with_base_iri(mut self, iri: Option<IriS>) -> Self {
        self.base_iri = iri;
        self
    }

    pub fn with_datatype_base_iri(mut self, iri: Option<IriS>) -> Self {
        self.datatype_base_iri = iri;
        self
    }

    pub fn with_prefixmap(mut self, prefixmap: PrefixMap) -> Self {
        self.prefixmap = prefixmap;
        self
    }

    pub fn with_dctap(mut self, cfg: TapConfig) -> Self {
        self.dctap = cfg;
        self
    }
}

impl Tap2ShExConfig {
    pub fn base_iri(&self) -> Option<&IriS> {
        self.base_iri.as_ref()
    }

    pub fn datatype_base_iri(&self) -> Option<&IriS> {
        self.datatype_base_iri.as_ref()
    }

    pub fn prefixmap(&self) -> &PrefixMap {
        &self.prefixmap
    }

    pub fn dctap(&self) -> &TapConfig {
        &self.dctap
    }
}

/// Serde stuff
#[allow(dead_code)]
#[cfg_attr(rustfmt, rustfmt_skip)]
impl Tap2ShExConfig {
    #[inline] fn default_datatype_base_iri() -> Option<IriS> { None }
    #[inline] fn default_prefixmap() -> PrefixMap { PrefixMap::basic() }
    #[inline] fn default_dctap() -> TapConfig { TapConfig::default() }
}

impl Tap2ShExConfig {

    // TODO: Refactor Tap2ShExError to reduce its size and avoid the result_large_err warning
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
                },
            }
        } else {
            let iri = match &self.base_iri {
                None => Err(Tap2ShExError::IriNoPrefix {
                    str: str.to_string(),
                    line,
                }),
                Some(base_iri) => base_iri.extend(str).map_err(|e| Tap2ShExError::IriSError { err: e }),
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
        Self::new()
    }
}
