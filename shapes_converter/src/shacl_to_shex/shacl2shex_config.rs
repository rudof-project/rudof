use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Shacl2ShExConfig {}

impl Default for Shacl2ShExConfig {
    fn default() -> Self {
        Self {}
    }
}

impl Shacl2ShExConfig {}
