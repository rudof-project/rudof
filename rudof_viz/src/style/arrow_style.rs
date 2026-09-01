use crate::style::{Color, LineStyle};
use serde::{Deserialize, Serialize};

/// Visual style of a connector (arrow) between two boxes.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ArrowStyle {
    #[serde(
        rename = "line_color",
        default = "ArrowStyle::default_line_color",
        skip_serializing_if = "ArrowStyle::is_default_line_color"
    )]
    line_color: Color,
    #[serde(
        rename = "line_thickness",
        default = "ArrowStyle::default_line_thickness",
        skip_serializing_if = "ArrowStyle::is_default_line_thickness"
    )]
    line_thickness: LineStyle,
    #[serde(
        rename = "text_color",
        default = "ArrowStyle::default_text_color",
        skip_serializing_if = "ArrowStyle::is_default_text_color"
    )]
    text_color: Color,
}

impl ArrowStyle {
    pub fn new() -> Self {
        ArrowStyle {
            line_color: Self::default_line_color(),
            line_thickness: Self::default_line_thickness(),
            text_color: Self::default_text_color(),
        }
    }

    pub fn with_line_color(mut self, color: Color) -> Self {
        self.line_color = color;
        self
    }

    pub fn with_line_thickness(mut self, thickness: LineStyle) -> Self {
        self.line_thickness = thickness;
        self
    }

    pub fn with_text_color(mut self, color: Color) -> Self {
        self.text_color = color;
        self
    }

    pub fn line_color(&self) -> Color {
        self.line_color
    }

    pub fn line_thickness(&self) -> LineStyle {
        self.line_thickness
    }

    pub fn text_color(&self) -> Color {
        self.text_color
    }
}

/// Serde stuff
#[allow(dead_code)]
#[rustfmt::skip]
impl ArrowStyle {
    #[inline] fn default_line_color() -> Color { Color::Black }
    #[inline] fn default_line_thickness() -> LineStyle { LineStyle::Normal }
    #[inline] fn default_text_color() -> Color { Color::Black }
    #[inline] fn is_default_line_color(value: &Color) -> bool { *value == Self::default_line_color() }
    #[inline] fn is_default_line_thickness(value: &LineStyle) -> bool { *value == Self::default_line_thickness() }
    #[inline] fn is_default_text_color(value: &Color) -> bool { *value == Self::default_text_color() }
}

impl Default for ArrowStyle {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_overrides_defaults() {
        let style = ArrowStyle::new()
            .with_line_color(Color::Blue)
            .with_line_thickness(LineStyle::Dashed)
            .with_text_color(Color::Green);
        assert_eq!(style.line_color(), Color::Blue);
        assert_eq!(style.line_thickness(), LineStyle::Dashed);
        assert_eq!(style.text_color(), Color::Green);
    }

    #[test]
    fn default_matches_new() {
        assert_eq!(ArrowStyle::default(), ArrowStyle::new());
    }
}
