use iri_s::IriS;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ComparatorConfig {
    prefixes_equivalences: HashSet<(IriS, IriS)>,
}

impl ComparatorConfig {
    pub fn new() -> Self {
        ComparatorConfig {
            prefixes_equivalences: HashSet::new(),
        }
    }
}

impl Default for ComparatorConfig {
    fn default() -> Self {
        Self::new()
    }
}
