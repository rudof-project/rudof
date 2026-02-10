use crate::rdf_core::visualizer::style::{UmlColor, ThicknessStyle};
use serde::{Deserialize, Serialize};

/// Default line thickness used when no explicit thickness is specified.
const DEFAULT_ARROW_LINE_THICKNESS: ThicknessStyle = ThicknessStyle::Normal;
/// Default line color used when no explicit color is specified.
const DEFAULT_ARROW_LINE_COLOR: UmlColor = UmlColor::Black;
/// Default text color used when no explicit color is specified.
const DEFAULT_ARROW_TEXT_COLOR: UmlColor = UmlColor::Black;

/// Default style for subject arrows.
pub const DEFAULT_SUBJECT_ARROW_STYLE: ArrowStyle = ArrowStyle {
    line_color: Some(UmlColor::Blue),
    line_thickness: Some(ThicknessStyle::Dashed),
    text_color: Some(UmlColor::Blue),
};
/// Default style for predicate arrows.
pub const DEFAULT_PREDICATE_ARROW_STYLE: ArrowStyle = ArrowStyle {
    line_color: Some(UmlColor::Red),
    line_thickness: Some(ThicknessStyle::Dashed),
    text_color: Some(UmlColor::Red),
};
/// Default style for object arrows.
pub const DEFAULT_OBJECT_ARROW_STYLE: ArrowStyle = ArrowStyle {
    line_color: Some(UmlColor::Green),
    line_thickness: Some(ThicknessStyle::Dashed),
    text_color: Some(UmlColor::Green),
};

/// Defines the visual style of an arrow in a PlantUML diagram.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ArrowStyle {
    /// Color of the arrow line.
    pub line_color: Option<UmlColor>,
    /// Thickness and style of the arrow line.
    pub line_thickness: Option<ThicknessStyle>,
    /// Color of the arrow text/label.
    pub text_color: Option<UmlColor>,
}

impl ArrowStyle {
    /// Creates a new [`ArrowStyle`] initialized with default values.
    pub fn new() -> Self {
        ArrowStyle {
            line_color: Some(DEFAULT_ARROW_LINE_COLOR),
            line_thickness: Some(DEFAULT_ARROW_LINE_THICKNESS),
            text_color: Some(DEFAULT_ARROW_TEXT_COLOR),
        }
    }

    /// Sets the line color and returns the modified [`ArrowStyle`].
    pub fn with_line_color(mut self, color: UmlColor) -> Self {
        self.line_color = Some(color);
        self
    }

    /// Sets the line thickness and returns the modified [`ArrowStyle`].
    pub fn with_line_thickness(mut self, thickness: ThicknessStyle) -> Self {
        self.line_thickness = Some(thickness);
        self
    }

    /// Sets the text color and returns the modified [`ArrowStyle`].
    pub fn with_text_color(mut self, color: UmlColor) -> Self {
        self.text_color = Some(color);
        self
    }

    /// Converts the arrow style into a PlantUML-compatible style string.
    /// 
    /// # Returns
    /// A `String` representing the arrow style in PlantUML syntax.
    pub fn as_plantuml(&self) -> String {
        format!(
            "#line:{};{}text:{}",
            self.line_color
                .as_ref()
                .map(|c| c.as_plantuml().to_lowercase())
                .unwrap_or_default(),
            self.line_thickness
                .as_ref()
                .map(|t| t.as_plantuml().to_lowercase())
                .unwrap_or_default(),
            self.text_color
                .as_ref()
                .map(|c| c.as_plantuml().to_lowercase())
                .unwrap_or_default()
        )
    }
}

impl Default for ArrowStyle {
    fn default() -> Self {
        Self::new()
    }
}
