use serde::{Deserialize, Serialize};

/// A named color from the small palette shared by rudof's visualization backends.
///
/// # Notes
/// - The variants must stay in sync with the colors supported by PlantUML.
/// - See: <https://github.com/qywx/PlantUML-colors>
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Copy, Default, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Color {
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

impl Color {
    /// Returns the canonical (PascalCase) name of the color, as understood by every backend.
    pub fn name(&self) -> &'static str {
        match self {
            Color::White => "White",
            Color::Black => "Black",
            Color::Cyan => "Cyan",
            Color::Gray => "Gray",
            Color::Red => "Red",
            Color::Green => "Green",
            Color::Blue => "Blue",
            Color::Yellow => "Yellow",
            Color::LightBlue => "LightBlue",
            Color::LightGreen => "LightGreen",
            Color::LightCoral => "LightCoral",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Color;

    #[test]
    fn default_is_black() {
        assert_eq!(Color::default(), Color::Black);
    }

    #[test]
    fn name_is_pascal_case() {
        assert_eq!(Color::LightCoral.name(), "LightCoral");
        assert_eq!(Color::Red.name(), "Red");
    }
}
