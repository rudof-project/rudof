use crate::style::Color;
use serde::{Deserialize, Serialize};

/// Visual style of a diagram box (background/line color, line thickness, corner rounding).
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct BoxStyle {
    #[serde(
        rename = "line_color",
        default = "BoxStyle::default_line_color",
        skip_serializing_if = "BoxStyle::is_default_line_color"
    )]
    line_color: Color,
    #[serde(
        rename = "line_thickness",
        default = "BoxStyle::default_line_thickness",
        skip_serializing_if = "BoxStyle::is_default_line_thickness"
    )]
    line_thickness: u32,
    #[serde(
        rename = "background_color",
        default = "BoxStyle::default_background_color",
        skip_serializing_if = "BoxStyle::is_default_background_color"
    )]
    background_color: Color,
    #[serde(
        rename = "round_corner",
        default = "BoxStyle::default_round_corner",
        skip_serializing_if = "BoxStyle::is_default_round_corner"
    )]
    round_corner: u32,
}

impl BoxStyle {
    pub fn new() -> Self {
        Self {
            line_color: Self::default_line_color(),
            line_thickness: Self::default_line_thickness(),
            background_color: Self::default_background_color(),
            round_corner: Self::default_round_corner(),
        }
    }

    pub fn with_line_color(mut self, color: Color) -> Self {
        self.line_color = color;
        self
    }

    pub fn with_line_thickness(mut self, v: u32) -> Self {
        self.line_thickness = v;
        self
    }

    pub fn with_background_color(mut self, color: Color) -> Self {
        self.background_color = color;
        self
    }

    pub fn with_round_corner(mut self, v: u32) -> Self {
        self.round_corner = v;
        self
    }

    pub fn line_color(&self) -> Color {
        self.line_color
    }

    pub fn line_thickness(&self) -> u32 {
        self.line_thickness
    }

    pub fn background_color(&self) -> Color {
        self.background_color
    }

    pub fn round_corner(&self) -> u32 {
        self.round_corner
    }
}

/// Serde stuff
#[allow(dead_code)]
#[rustfmt::skip]
impl BoxStyle {
    #[inline] fn default_line_color() -> Color { Color::Black }
    #[inline] fn default_line_thickness() -> u32 { 10 }
    #[inline] fn default_background_color() -> Color { Color::White }
    #[inline] fn default_round_corner() -> u32 { 0 }
    #[inline] fn is_default_line_color(value: &Color) -> bool { *value == Self::default_line_color() }
    #[inline] fn is_default_line_thickness(value: &u32) -> bool { *value == Self::default_line_thickness() }
    #[inline] fn is_default_background_color(value: &Color) -> bool { *value == Self::default_background_color() }
    #[inline] fn is_default_round_corner(value: &u32) -> bool { *value == Self::default_round_corner() }
}

impl Default for BoxStyle {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_overrides_defaults() {
        let style = BoxStyle::new()
            .with_line_color(Color::Blue)
            .with_line_thickness(1)
            .with_background_color(Color::Gray)
            .with_round_corner(25);
        assert_eq!(style.line_color(), Color::Blue);
        assert_eq!(style.line_thickness(), 1);
        assert_eq!(style.background_color(), Color::Gray);
        assert_eq!(style.round_corner(), 25);
    }

    #[test]
    fn default_matches_new() {
        assert_eq!(BoxStyle::default(), BoxStyle::new());
    }
}
