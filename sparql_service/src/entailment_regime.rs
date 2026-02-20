use iri_s::IriS;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default, Serialize, Deserialize)]
pub enum EntailmentRegime {
    #[default]
    Simple,
    Rdf,
    Rdfs,
    D,
    OWLDirect,
    OWLRDFBased,
    Rif,
    Other(IriS),
}

impl Display for EntailmentRegime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntailmentRegime::Simple => write!(f, "Simple"),
            EntailmentRegime::Rdf => write!(f, "RDF"),
            EntailmentRegime::Rdfs => write!(f, "RDFS"),
            EntailmentRegime::D => write!(f, "D"),
            EntailmentRegime::OWLDirect => write!(f, "OWLDirect"),
            EntailmentRegime::OWLRDFBased => write!(f, "OWLRDFBased"),
            EntailmentRegime::Rif => write!(f, "RIF"),
            EntailmentRegime::Other(iri) => write!(f, "EntailmentRegime({iri})",),
        }
    }
}
