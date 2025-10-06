use crate::rdf_visualizer::uml_color::UmlColor;

pub struct StereotypeStyle {
    stereotype_name: String,
    background_color: Option<UmlColor>,
    line_thickness: Option<u32>,
    line_color: Option<UmlColor>,
    round_corner: Option<u32>,
}

impl StereotypeStyle {
    pub fn new(stereotype_name: &str) -> Self {
        StereotypeStyle {
            stereotype_name: stereotype_name.to_string(),
            background_color: None,
            line_thickness: None,
            line_color: None,
            round_corner: None,
        }
    }

    pub fn with_background_color(mut self, color: UmlColor) -> Self {
        self.background_color = Some(color);
        self
    }

    pub fn with_line_thickness(mut self, thickness: u32) -> Self {
        self.line_thickness = Some(thickness);
        self
    }

    pub fn with_line_color(mut self, color: UmlColor) -> Self {
        self.line_color = Some(color);
        self
    }

    pub fn with_round_corner(mut self, corner: u32) -> Self {
        self.round_corner = Some(corner);
        self
    }

    fn show_round_corner(&self) -> String {
        if let Some(corner) = self.round_corner {
            format!("RoundCorner {corner}\n")
        } else {
            String::new()
        }
    }

    fn show_line_thickness(&self) -> String {
        if let Some(thickness) = self.line_thickness {
            format!("LineThickness {thickness}\n")
        } else {
            String::new()
        }
    }

    fn show_background_color(&self) -> String {
        if let Some(color) = &self.background_color {
            format!("BackGroundColor {}\n", color.as_plantuml())
        } else {
            String::new()
        }
    }

    fn show_line_color(&self) -> String {
        if let Some(color) = &self.line_color {
            format!("LineColor {}\n", color.as_plantuml())
        } else {
            String::new()
        }
    }

    pub fn as_plantuml(&self) -> String {
        format!(
            ".{} {{\n{}{}{}{}\n}}\n",
            self.stereotype_name,
            self.show_background_color(),
            self.show_line_thickness(),
            self.show_line_color(),
            self.show_round_corner()
        )
    }
}
