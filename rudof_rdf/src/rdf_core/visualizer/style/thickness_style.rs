use serde::{Deserialize, Serialize};

/// Represents the available line thickness and style options in PlantUML.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub enum ThicknessStyle {
    Bold,
    #[default]
    Normal,
    Dashed,
    Dotted,
}

impl ThicknessStyle {
    /// Returns the PlantUML-compatible directive for the line style.
    pub fn as_plantuml(&self) -> &'static str {
        match self {
            ThicknessStyle::Bold => "line.bold;",
            ThicknessStyle::Normal => "",
            ThicknessStyle::Dashed => "line.dashed;",
            ThicknessStyle::Dotted => "line.dotted;",
        }
    }
}
