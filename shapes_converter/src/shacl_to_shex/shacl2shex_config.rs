use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
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

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
pub enum StartShapeMode {
    #[default]
    NonBNodes,
}
