use iri_s::IriS;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

const DEFAULT_SHOW_DATA_DESCRIPTIONS: bool = true;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ComparatorConfig {
    prefixes_equivalences: HashSet<(IriS, IriS)>,
    show_data_desciptions: Option<bool>,
}

impl ComparatorConfig {
    pub fn new() -> Self {
        ComparatorConfig {
            prefixes_equivalences: HashSet::new(),
            show_data_desciptions: None,
        }
    }

    pub fn show_data_descriptions(&self) -> bool {
        self.show_data_desciptions
            .unwrap_or(DEFAULT_SHOW_DATA_DESCRIPTIONS)
    }
}

impl Default for ComparatorConfig {
    fn default() -> Self {
        Self::new()
    }
}
