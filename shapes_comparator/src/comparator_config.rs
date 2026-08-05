use rudof_iri::IriS;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ComparatorConfig {
    #[serde(rename = "prefixes_equivalences", skip_serializing_if = "HashSet::is_empty")]
    pub(crate) prefixes_equivalences: HashSet<(IriS, IriS)>,
    #[serde(rename = "ignore_value_constraints")]
    pub(crate) ignore_value_constraints: bool,
}

/// Serde stuff
#[allow(dead_code)]
#[cfg_attr(rustfmt, rustfmt_skip)]
impl ComparatorConfig {
    #[inline] fn default_prefixes_equivalences() -> HashSet<(IriS, IriS)> { HashSet::new() }
    #[inline] fn default_ignore_value_constraints() -> bool { false }
}

impl ComparatorConfig {
    pub fn new() -> Self {
        Self {
            prefixes_equivalences: Self::default_prefixes_equivalences(),
            ignore_value_constraints: Self::default_ignore_value_constraints(),
        }
    }

    pub fn with_prefixes_equivalences(mut self, value: HashSet<(IriS, IriS)>) -> Self {
        self.prefixes_equivalences = value;
        self
    }

    pub fn with_ignore_value_constraints(mut self, flag: bool) -> Self {
        self.ignore_value_constraints = flag;
        self
    }
}

impl ComparatorConfig {
    pub fn prefixes_equivalences(&self) -> &HashSet<(IriS, IriS)> {
        &self.prefixes_equivalences
    }

    pub fn ignore_value_constraints(&self) -> bool {
        self.ignore_value_constraints
    }
}

impl Default for ComparatorConfig {
    fn default() -> Self {
        Self::new()
    }
}
