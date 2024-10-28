use serde_derive::{Deserialize, Serialize};

/// Contains possible ShEx formats
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone, Default)]
pub enum ShExFormat {
    #[default]
    ShExC,
    ShExJ,
    Turtle,
}
