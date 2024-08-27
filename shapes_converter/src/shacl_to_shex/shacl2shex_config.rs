use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Shacl2ShExConfig {
    starting_shapes_mode: Option<StartShapeMode>,
    embed_bnodes: Option<bool>,
}

impl Shacl2ShExConfig {
    pub fn starting_shapes_mode(&self) -> StartShapeMode {
        match &self.starting_shapes_mode {
            None => StartShapeMode::default(),
            Some(sm) => sm.clone(),
        }
    }
}

impl Default for Shacl2ShExConfig {
    fn default() -> Self {
        Self {
            starting_shapes_mode: Default::default(),
            embed_bnodes: Default::default(),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
pub enum StartShapeMode {
    #[default]
    NonBNodes,
}
