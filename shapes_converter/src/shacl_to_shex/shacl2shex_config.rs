use serde::{Deserialize, Serialize};
use shacl_validation::shacl_config::ShaclConfig;

/// Defines the configuration of the converter
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
pub struct Shacl2ShExConfig {
    pub starting_shapes_mode: Option<StartShapeMode>,
    pub embed_bnodes: Option<bool>,
    pub shacl: Option<ShaclConfig>,
}

impl Shacl2ShExConfig {
    pub fn starting_shapes_mode(&self) -> StartShapeMode {
        match &self.starting_shapes_mode {
            None => StartShapeMode::default(),
            Some(sm) => sm.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
pub enum StartShapeMode {
    #[default]
    NonBNodes,
}
