use iri_s::{iri, IriS};
use prefixmap::PrefixMap;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct Tap2ShExConfig {
    pub base_iri: Option<IriS>,
    pub datatype_base_iri: Option<IriS>,
    pub prefixmap: PrefixMap,
}

impl Default for Tap2ShExConfig {
    fn default() -> Self {
        Self {
            base_iri: Some(iri!("http://example.org/")),
            datatype_base_iri: None,
            prefixmap: PrefixMap::basic(),
        }
    }
}
