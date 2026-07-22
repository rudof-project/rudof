use serde::{Deserialize, Deserializer};
use dctap::{PrefixCC, TapConfig};
use prefixmap::PrefixMap;
use rudof_iri::IriS;

use super::Tap2ShExError;

#[derive(Debug, PartialEq, Clone)]
pub struct Tap2ShExConfig {
    pub(crate) base_iri: Option<IriS>,
    base_iri_needs_fixup: bool,
    pub(crate) datatype_base_iri: Option<IriS>,
    pub(crate) prefixmap: PrefixMap,
    pub(crate) dctap: TapConfig,
    dctap_needs_fixup: bool,

    // TODO - Can we remove this and use the prefix map?
    // #[serde(skip)]
    // prefix_cc: Option<PrefixCC>,
}

impl Tap2ShExConfig {
    pub fn new() -> Self {
        Self {
            base_iri: None,
            base_iri_needs_fixup: false,
            datatype_base_iri: Self::default_datatype_base_iri(),
            prefixmap: Self::default_prefixmap(),
            dctap: Self::default_dctap(),
            dctap_needs_fixup: false,
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

    pub fn fixup(&mut self, base_iri: Option<IriS>, dctap: TapConfig) {
        if self.dctap_needs_fixup {
            self.dctap_needs_fixup = false;
            self.dctap = dctap;
        }

        if self.base_iri_needs_fixup {
            self.base_iri_needs_fixup = false;
            self.base_iri = base_iri;
        }
    }
}

impl<'de> Deserialize<'de> for Tap2ShExConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        #[derive(Deserialize)]
        struct Raw {
            #[serde(rename = "base_iri", default)]
            base: Option<IriS>,
            #[serde(rename = "datatype_base_iri", default = "Tap2ShExConfig::default_datatype_base_iri")]
            datatype_base_iri: Option<IriS>,
            #[serde(rename = "prefixmap", default = "Tap2ShExConfig::default_prefixmap")]
            prefixmap: PrefixMap,
            #[serde(rename = "dctap", default)]
            dctap: Option<TapConfig>
        }

        let raw = Raw::deserialize(deserializer)?;

        Ok(Self {
            base_iri_needs_fixup: raw.base.is_none(),
            base_iri: raw.base,
            datatype_base_iri: raw.datatype_base_iri,
            prefixmap: raw.prefixmap,
            dctap_needs_fixup: raw.dctap.is_none(),
            dctap: raw.dctap.unwrap_or(Self::default_dctap()),
            // prefix_cc: None,
        })
    }
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
