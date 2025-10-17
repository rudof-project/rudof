use iri_s::IriS;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

const DEFAULT_IGNORE_VALUE_CONSTRAINTS: bool = false;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ComparatorConfig {
    prefixes_equivalences: HashSet<(IriS, IriS)>,
    ignore_value_constraints: Option<bool>,
}

impl ComparatorConfig {
    pub fn new() -> Self {
        ComparatorConfig {
            prefixes_equivalences: HashSet::new(),
            ignore_value_constraints: None,
        }
    }

    pub fn ignore_value_constraints(&self) -> bool {
        self.ignore_value_constraints
            .unwrap_or(DEFAULT_IGNORE_VALUE_CONSTRAINTS)
    }
}

impl Default for ComparatorConfig {
    fn default() -> Self {
        Self::new()
    }
}
