use iri_s::IriS;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default, Serialize, Deserialize)]
pub enum EntailmentProfile {
    #[default]
    DL,
    EL,
    QL,
    RL,
    Full,
    Other(IriS),
}

impl Display for EntailmentProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntailmentProfile::Other(iri) => write!(f, "EntailmentProfile({iri})",),
            EntailmentProfile::DL => write!(f, "DL"),
            EntailmentProfile::EL => write!(f, "EL"),
            EntailmentProfile::QL => write!(f, "QL"),
            EntailmentProfile::RL => write!(f, "RL"),
            EntailmentProfile::Full => write!(f, "Full"),
        }
    }
}
