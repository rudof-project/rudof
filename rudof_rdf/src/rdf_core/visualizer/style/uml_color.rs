use serde::{Deserialize, Serialize};

/// Represents the set of colors supported by PlantUML.
///
/// # Notes
/// - The variants must stay in sync with the colors supported by PlantUML.
/// - See: https://github.com/qywx/PlantUML-colors
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub enum UmlColor {
    White,
    #[default]
    Black,
    Cyan,
    Gray,
    Red,
    Green,
    Blue,
    Yellow,
    LightBlue,
    LightGreen,
    LightCoral,
}

// REVIEW - This could be a trait?
impl UmlColor {
    /// Returns the PlantUML-compatible string representation of the color.
    pub fn as_plantuml(&self) -> String {
        match self {
            UmlColor::Red => "Red".to_string(),
            UmlColor::Green => "Green".to_string(),
            UmlColor::Blue => "Blue".to_string(),
            UmlColor::Yellow => "Yellow".to_string(),
            UmlColor::LightBlue => "LightBlue".to_string(),
            UmlColor::LightGreen => "LightGreen".to_string(),
            UmlColor::LightCoral => "LightCoral".to_string(),
            UmlColor::White => "White".to_string(),
            UmlColor::Black => "Black".to_string(),
            UmlColor::Cyan => "Cyan".to_string(),
            UmlColor::Gray => "Gray".to_string(),
        }
    }
}
