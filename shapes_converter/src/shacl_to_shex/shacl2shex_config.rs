use serde::{Deserialize, Serialize};
use shacl_validation::shacl_config::ShaclConfig;

/// Defines the configuration of the converter
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
pub struct Shacl2ShExConfig {
    /// Starting shapes mode. Default: NonBNodes
    pub starting_shapes_mode: Option<StartShapeMode>,

    /// If true, embed blank nodes in the ShEx schema
    pub embed_bnodes: Option<bool>,

    /// SHACL configuration
    pub shacl: Option<ShaclConfig>,

    /// Add an `rdf:type` constraint for `sh:targetClass` declarations
    pub add_target_class: Option<bool>,
}

impl Shacl2ShExConfig {
    pub fn starting_shapes_mode(&self) -> StartShapeMode {
        match &self.starting_shapes_mode {
            None => StartShapeMode::default(),
            Some(sm) => sm.clone(),
        }
    }

    pub fn add_target_class(&self) -> bool {
        match &self.add_target_class {
            None => true,
            Some(atc) => *atc,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
pub enum StartShapeMode {
    /// Process shapes which are not blank nodes
    #[default]
    NonBNodes,
}
