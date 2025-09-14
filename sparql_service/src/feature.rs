use iri_s::IriS;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// Features defined in: https://www.w3.org/TR/sparql11-service-description/#sd-Feature
#[derive(Clone, PartialEq, Eq, Debug, Hash, Serialize, Deserialize)]
pub enum Feature {
    DereferencesURIs,
    UnionDefaultGraph,
    RequiresDataset,
    EmptyGraphs,
    BasicFederatedQuery,
    Other(IriS),
}

impl Display for Feature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Feature::DereferencesURIs => write!(f, "DereferencesURIs"),
            Feature::UnionDefaultGraph => write!(f, "UnionDefaultGraph"),
            Feature::RequiresDataset => write!(f, "RequiresDataset"),
            Feature::EmptyGraphs => write!(f, "EmptyGraphs"),
            Feature::BasicFederatedQuery => write!(f, "BasicFederatedQuery"),
            Feature::Other(iri) => write!(f, "Feature({iri})"),
        }
    }
}
