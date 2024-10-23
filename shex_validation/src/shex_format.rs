use serde_derive::{Deserialize, Serialize};

/// Contains possible ShEx formats
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub enum ShExFormat {
    ShExC,
    ShExJ,
    Turtle,
}
