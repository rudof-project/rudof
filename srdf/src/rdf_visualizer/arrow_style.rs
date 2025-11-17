use serde::{Deserialize, Serialize};

use crate::rdf_visualizer::{thickness_style::ThicknessStyle, uml_color::UmlColor};

const DEFAULT_ARROW_LINE_THICKNESS: ThicknessStyle = ThicknessStyle::Normal;
const DEFAULT_ARROW_LINE_COLOR: UmlColor = UmlColor::Black;
const DEFAULT_ARROW_TEXT_COLOR: UmlColor = UmlColor::Black;

pub const DEFAULT_SUBJECT_ARROW_STYLE: ArrowStyle = ArrowStyle {
    line_color: Some(UmlColor::Blue),
    line_thickness: Some(ThicknessStyle::Dashed),
    text_color: Some(UmlColor::Blue),
};

pub const DEFAULT_PREDICATE_ARROW_STYLE: ArrowStyle = ArrowStyle {
    line_color: Some(UmlColor::Red),
    line_thickness: Some(ThicknessStyle::Dashed),
    text_color: Some(UmlColor::Red),
};

pub const DEFAULT_OBJECT_ARROW_STYLE: ArrowStyle = ArrowStyle {
    line_color: Some(UmlColor::Green),
    line_thickness: Some(ThicknessStyle::Dashed),
    text_color: Some(UmlColor::Green),
};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ArrowStyle {
    pub line_color: Option<UmlColor>,
    pub line_thickness: Option<ThicknessStyle>,
    pub text_color: Option<UmlColor>,
}

impl ArrowStyle {
    pub fn new() -> Self {
        ArrowStyle {
            line_color: Some(DEFAULT_ARROW_LINE_COLOR),
            line_thickness: Some(DEFAULT_ARROW_LINE_THICKNESS),
            text_color: Some(DEFAULT_ARROW_TEXT_COLOR),
        }
    }

    pub fn with_line_color(mut self, color: UmlColor) -> Self {
        self.line_color = Some(color);
        self
    }

    pub fn with_line_thickness(mut self, thickness: ThicknessStyle) -> Self {
        self.line_thickness = Some(thickness);
        self
    }

    pub fn with_text_color(mut self, color: UmlColor) -> Self {
        self.text_color = Some(color);
        self
    }

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
