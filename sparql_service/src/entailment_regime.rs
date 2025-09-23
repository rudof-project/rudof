use iri_s::IriS;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default, Serialize, Deserialize)]
pub enum EntailmentRegime {
    #[default]
    Simple,
    RDF,
    RDFS,
    D,
    OWLDirect,
    OWLRDFBased,
    RIF,
    Other(IriS),
}

impl Display for EntailmentRegime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntailmentRegime::Simple => write!(f, "Simple"),
            EntailmentRegime::RDF => write!(f, "RDF"),
            EntailmentRegime::RDFS => write!(f, "RDFS"),
            EntailmentRegime::D => write!(f, "D"),
            EntailmentRegime::OWLDirect => write!(f, "OWLDirect"),
            EntailmentRegime::OWLRDFBased => write!(f, "OWLRDFBased"),
            EntailmentRegime::RIF => write!(f, "RIF"),
            EntailmentRegime::Other(iri) => write!(f, "EntailmentRegime({iri})",),
        }
    }
}
