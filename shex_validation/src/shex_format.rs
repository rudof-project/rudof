use serde::{Deserialize, Serialize};

/// Contains possible ShEx formats
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone, Default)]
pub enum ShExFormat {
    #[default]
    ShExC,
    ShExJ,
    Turtle,
}

impl ShExFormat {
    /// Returns the MIME type for the ShEx format
    pub fn mime_type(&self) -> &str {
        match self {
            ShExFormat::ShExC => "text/shex",
            ShExFormat::ShExJ => "application/shex+json",
            ShExFormat::Turtle => "text/turtle",
        }
    }
}
