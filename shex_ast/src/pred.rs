use iri_s::IriS;
use rbe::Key;
use std::fmt::Display;

#[derive(PartialEq, Eq, Hash, Debug, Default, Clone)]
pub struct Pred {
    iri: IriS,
}

impl Display for Pred {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.iri)
    }
}

impl From<IriS> for Pred {
    fn from(iri: IriS) -> Self {
        Pred { iri }
    }
}

impl Key for Pred {}
