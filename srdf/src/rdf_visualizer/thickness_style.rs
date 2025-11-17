use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum ThicknessStyle {
    Bold,
    Normal,
    Dashed,
    Dotted,
}

impl ThicknessStyle {
    pub fn as_plantuml(&self) -> &'static str {
        match self {
            ThicknessStyle::Bold => "line.bold;",
            ThicknessStyle::Normal => "",
            ThicknessStyle::Dashed => "line.dashed;",
            ThicknessStyle::Dotted => "line.dotted;",
        }
    }
}
