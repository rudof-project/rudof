use serde::{Deserialize, Serialize};

/// Possible UML colors.
/// These colors should be the same colors as the colors supported by PlantUML
/// https://github.com/qywx/PlantUML-colors
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum UmlColor {
    // TODO: Add more colors...
    White,
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

impl UmlColor {
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
