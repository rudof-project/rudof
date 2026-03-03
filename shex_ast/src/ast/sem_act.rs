use prefixmap::error::DerefError;
use prefixmap::{DerefIri, IriRef};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct SemAct {
    name: IriRef,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    code: Option<String>,
}

impl SemAct {
    pub fn new(name: IriRef, code: Option<String>) -> SemAct {
        SemAct { name, code }
    }

    pub fn name(&self) -> IriRef {
        self.name.clone()
    }

    pub fn code(&self) -> Option<String> {
        self.code.clone()
    }
}

impl DerefIri for SemAct {
    fn deref_iri(
        self,
        base: Option<&iri_s::IriS>,
        prefixmap: Option<&prefixmap::PrefixMap>,
    ) -> Result<Self, DerefError> {
        let new_name = self.name.deref_iri(base, prefixmap)?;
        Ok(SemAct {
            name: new_name,
            code: self.code.clone(),
        })
    }
}
