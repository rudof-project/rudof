use rbe::Key;
use rudof_iri::IriS;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(PartialEq, Eq, Hash, Debug, Default, Clone, Serialize, Deserialize)]
pub struct Pred {
    iri: IriS,
    is_direct: bool,
}

impl Pred {
    pub fn new(iri: IriS, is_direct: bool) -> Self {
        Pred { iri, is_direct }
    }

    pub fn new_unchecked(str: &str) -> Self {
        Pred {
            iri: IriS::new_unchecked(str),
            is_direct: true,
        }
    }

    pub fn iri(&self) -> &IriS {
        &self.iri
    }

    pub fn is_direct(&self) -> bool {
        self.is_direct
    }
}

impl Display for Pred {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_direct {
            write!(f, "{}", self.iri)
        } else {
            write!(f, "^{}", self.iri)
        }
    }
}

impl From<IriS> for Pred {
    fn from(iri: IriS) -> Self {
        Pred { iri, is_direct: true }
    }
}

impl Key for Pred {}
