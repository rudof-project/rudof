use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct RDFVisualizationConfig {
    blank_node_color: Option<Color>,
    named_node_color: Option<Color>,
    literal_color: Option<Color>,
    blank_node_background_color: Option<Color>,
    named_node_background_color: Option<Color>,
    literal_background_color: Option<Color>,
}

impl RDFVisualizationConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_blank_node_color(mut self, color: Color) -> Self {
        self.blank_node_color = Some(color);
        self
    }

    pub fn with_named_node_color(mut self, color: Color) -> Self {
        self.named_node_color = Some(color);
        self
    }

    pub fn with_literal_color(mut self, color: Color) -> Self {
        self.literal_color = Some(color);
        self
    }

    pub fn with_blank_node_background_color(mut self, color: Color) -> Self {
        self.blank_node_background_color = Some(color);
        self
    }

    pub fn with_named_node_background_color(mut self, color: Color) -> Self {
        self.named_node_background_color = Some(color);
        self
    }

    pub fn with_literal_background_color(mut self, color: Color) -> Self {
        self.literal_background_color = Some(color);
        self
    }
}

impl Default for RDFVisualizationConfig {
    fn default() -> Self {
        RDFVisualizationConfig {
            blank_node_color: Some(Color::Blue),
            named_node_color: Some(Color::Green),
            literal_color: Some(Color::Red),
            blank_node_background_color: Some(Color::LightBlue),
            named_node_background_color: Some(Color::LightGreen),
            literal_background_color: Some(Color::LightCoral),
        }
    }
}

/// Possible colors.
/// These colors should be the same colors as the colors supported by PlantUML
/// https://github.com/qywx/PlantUML-colors
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum Color {
    Red,
    Green,
    Blue,
    LightBlue,
    LightGreen,
    LightCoral,
}
