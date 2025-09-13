use std::fmt::Display;

use iri_s::IriS;

#[derive(Clone, PartialEq, Eq, Debug, Default)]
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
