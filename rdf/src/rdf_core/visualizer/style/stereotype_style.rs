use crate::rdf_core::visualizer::style::UmlColor;

/// Represents the visual style of a UML stereotype in PlantUML.
///
/// This struct allows configuring the appearance of a stereotype block, including:
/// - Background color
/// - Line thickness
/// - Line color
/// - Rounded corners
pub struct StereotypeStyle {
    /// Name of the stereotype (e.g., `<<entity>>`) used in PlantUML.
    stereotype_name: String,
    /// Optional background color of the stereotype block.
    background_color: Option<UmlColor>,
    /// Optional line thickness of the stereotype block.
    line_thickness: Option<u32>,
    /// Optional border color of the stereotype block.
    line_color: Option<UmlColor>,
    /// Optional corner radius for rounded corners.
    round_corner: Option<u32>,
}

impl StereotypeStyle {
    /// Creates a new [`StereotypeStyle`] with the given stereotype name.
    ///
    /// All style attributes are initially `None`.
    ///
    /// # Arguments
    /// * `stereotype_name` - Name of the stereotype for PlantUML output.
    pub fn new(stereotype_name: &str) -> Self {
        StereotypeStyle {
            stereotype_name: stereotype_name.to_string(),
            background_color: None,
            line_thickness: None,
            line_color: None,
            round_corner: None,
        }
    }

    /// Sets the background color of the stereotype block.
    pub fn with_background_color(mut self, color: UmlColor) -> Self {
        self.background_color = Some(color);
        self
    }

    /// Sets the line thickness of the stereotype block.
    pub fn with_line_thickness(mut self, thickness: u32) -> Self {
        self.line_thickness = Some(thickness);
        self
    }

    /// Sets the line color of the stereotype block.
    pub fn with_line_color(mut self, color: UmlColor) -> Self {
        self.line_color = Some(color);
        self
    }

    /// Sets the corner radius for rounded corners.
    pub fn with_round_corner(mut self, corner: u32) -> Self {
        self.round_corner = Some(corner);
        self
    }

    /// Generates the PlantUML line for the round corner if specified.
    ///
    /// # Returns
    /// A `String` containing the PlantUML syntax for the round corner (e.g., `"RoundCorner 10\n"`),
    /// or an empty string if no corner radius is set.
    fn show_round_corner(&self) -> String {
        if let Some(corner) = self.round_corner {
            format!("RoundCorner {corner}\n")
        } else {
            String::new()
        }
    }

    /// Generates the PlantUML line for line thickness if specified.
    ///
    /// # Returns
    /// A `String` containing the PlantUML syntax for line thickness (e.g., `"LineThickness 2\n"`),
    /// or an empty string if no thickness is set.
    fn show_line_thickness(&self) -> String {
        if let Some(thickness) = self.line_thickness {
            format!("LineThickness {thickness}\n")
        } else {
            String::new()
        }
    }

    /// Generates the PlantUML line for background color if specified.
    ///
    /// # Returns
    /// A `String` containing the PlantUML syntax for background color
    /// (e.g., `"BackGroundColor red\n"`), or an empty string if no background color is set.
    fn show_background_color(&self) -> String {
        if let Some(color) = &self.background_color {
            format!("BackGroundColor {}\n", color.as_plantuml())
        } else {
            String::new()
        }
    }

    /// Generates the PlantUML line for line color if specified.
    ///
    /// # Returns
    /// A `String` containing the PlantUML syntax for line color
    /// (e.g., `"LineColor black\n"`), or an empty string if no line color is set.
    fn show_line_color(&self) -> String {
        if let Some(color) = &self.line_color {
            format!("LineColor {}\n", color.as_plantuml())
        } else {
            String::new()
        }
    }

    /// Converts the `StereotypeStyle` into a PlantUML-compatible string.
    ///
    /// # Returns
    /// A `String` containing the PlantUML syntax for the stereotype block,
    /// including all configured styles. Unspecified attributes are omitted,
    /// allowing PlantUML defaults to apply.
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
