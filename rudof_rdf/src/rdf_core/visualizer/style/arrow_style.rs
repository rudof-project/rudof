use crate::rdf_core::visualizer::style::{ThicknessStyle, UmlColor};
use serde::{Deserialize, Serialize};

/// Defines the visual style of an arrow in a PlantUML diagram.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ArrowStyle {
    /// Color of the arrow line.
    #[serde(rename = "line_color", default = "ArrowStyle::default_line_color", skip_serializing_if = "ArrowStyle::is_default_line_color")]
    pub(crate) line_color: UmlColor,
    /// Thickness and style of the arrow line.
    #[serde(rename = "line_thickness", default = "ArrowStyle::default_line_thickness", skip_serializing_if = "ArrowStyle::is_default_line_thickness")]
    pub(crate) line_thickness: ThicknessStyle,
    /// Color of the arrow text/label.
    #[serde(rename = "text_color", default = "ArrowStyle::default_text_color", skip_serializing_if = "ArrowStyle::is_default_text_color")]
    pub(crate) text_color: UmlColor,
}

/// Serde stuff
#[allow(dead_code)]
impl ArrowStyle {
    #[inline] fn default_line_color() -> UmlColor { UmlColor::Black }
    #[inline] fn default_line_thickness() -> ThicknessStyle { ThicknessStyle::Normal }
    #[inline] fn default_text_color() -> UmlColor { UmlColor::Black }
    #[inline] fn is_default_line_color(value: &UmlColor) -> bool { value == &Self::default_line_color() }
    #[inline] fn is_default_line_thickness(value: &ThicknessStyle) -> bool { value == &Self::default_line_thickness() }
    #[inline] fn is_default_text_color(value: &UmlColor) -> bool { value == &Self::default_text_color() }
}

impl ArrowStyle {
    /// Creates a new [`ArrowStyle`] initialized with default values.
    pub fn new() -> Self {
        ArrowStyle {
            line_color: Self::default_line_color(),
            line_thickness: Self::default_line_thickness(),
            text_color: Self::default_text_color(),
        }
    }

    /// Sets the line color and returns the modified [`ArrowStyle`].
    pub fn with_line_color(mut self, color: UmlColor) -> Self {
        self.line_color = color;
        self
    }

    /// Sets the line thickness and returns the modified [`ArrowStyle`].
    pub fn with_line_thickness(mut self, thickness: ThicknessStyle) -> Self {
        self.line_thickness = thickness;
        self
    }

    /// Sets the text color and returns the modified [`ArrowStyle`].
    pub fn with_text_color(mut self, color: UmlColor) -> Self {
        self.text_color = color;
        self
    }

    /// Returns the line color
    pub fn line_color(&self) -> &UmlColor {
        &self.line_color
    }

    /// Returns the line thickness
    pub fn line_thickness(&self) -> &ThicknessStyle {
        &self.line_thickness
    }

    /// Returns the text color
    pub fn text_color(&self) -> &UmlColor {
        &self.text_color
    }

    /// Converts the arrow style into a PlantUML-compatible style string.
    ///
    /// # Returns
    /// A `String` representing the arrow style in PlantUML syntax.
    pub fn as_plantuml(&self) -> String {
        format!(
            "#line:{};{}text:{}",
            &self.line_color.as_plantuml().to_lowercase(),
            &self.line_thickness.as_plantuml().to_lowercase(),
            &self.text_color.as_plantuml().to_lowercase()
        )
    }
}

impl Default for ArrowStyle {
    fn default() -> Self {
        Self::new()
    }
}
