use crate::rdf_core::visualizer::{RDFVisualizationConfig, style::StereotypeStyle};

/// Main style container for RDF visualization.
///
/// Represents a collection of stereotype styles that define the visual appearance
/// of different RDF term types in PlantUML diagrams. This struct manages the
/// overall styling configuration and provides methods to generate PlantUML style
/// declarations.
pub struct Style {
    /// Collection of stereotype styles for different RDF term types.
    stereotype_styles: Vec<StereotypeStyle>,
}

impl Style {
    /// Creates a new empty `Style` instance.
    ///
    /// The returned style has no stereotype styles configured.
    pub fn new() -> Self {
        Style {
            stereotype_styles: Vec::new(),
        }
    }

    /// Adds a stereotype style to this style collection.
    ///
    /// # Arguments
    /// * `style` - The `StereotypeStyle` to add to the collection.
    pub fn add_stereotype_style(&mut self, style: StereotypeStyle) {
        self.stereotype_styles.push(style);
    }

    /// Generates PlantUML style declarations for all configured stereotype styles.
    ///
    /// This method produces a complete PlantUML `<style>` block containing all
    /// the stereotype definitions, followed by a directive to hide stereotypes
    /// in the final diagram.
    ///
    /// # Returns
    /// A string containing the PlantUML style declarations.
    pub fn as_uml(&self) -> String {
        let mut uml = String::new();
        uml.push_str("<style>\n");
        for style in &self.stereotype_styles {
            uml.push_str(&style.as_plantuml());
        }
        uml.push_str("</style>\n");
        uml.push_str("hide stereotype\n");
        uml
    }

    /// Creates a `Style` instance from an RDF visualization configuration.
    ///
    /// This method automatically generates stereotype styles for all RDF term types
    /// (URI nodes, blank nodes, literals, reifiers, asserted triples, non-asserted triples)
    /// based on the provided configuration.
    ///
    /// # Arguments
    /// * `config` - The `RDFVisualizationConfig` containing style settings.
    ///
    /// # Returns
    /// A new `Style` instance with stereotype styles configured according to the config.
    pub fn from_config(config: &RDFVisualizationConfig) -> Self {
        let mut style = Style::new();
        style.add_stereotype_style(reifier_style(config));
        style.add_stereotype_style(literal_style(config));
        style.add_stereotype_style(uri_style(config));
        style.add_stereotype_style(bnode_style(config));
        style.add_stereotype_style(asserted_style(config));
        style.add_stereotype_style(non_asserted_style(config));
        style
    }
}

/// Creates a stereotype style for reifier nodes based on the configuration.
///
/// # Arguments
/// * `config` - The RDF visualization configuration.
///
/// # Returns
/// A `StereotypeStyle` configured for reifier nodes.
fn reifier_style(config: &RDFVisualizationConfig) -> StereotypeStyle {
    StereotypeStyle::new("reifier")
        .with_background_color(config.reifier_background_color())
        .with_line_color(config.reifier_line_color())
        .with_line_thickness(config.reifier_line_thickness())
}

/// Creates a stereotype style for literal nodes based on the configuration.
///
/// # Arguments
/// * `config` - The RDF visualization configuration.
///
/// # Returns
/// A `StereotypeStyle` configured for literal nodes.
fn literal_style(config: &RDFVisualizationConfig) -> StereotypeStyle {
    StereotypeStyle::new("literal")
        .with_background_color(config.literal_background_color())
        .with_line_color(config.literal_line_color())
        .with_line_thickness(config.literal_line_thickness())
}

/// Creates a stereotype style for URI nodes based on the configuration.
///
/// # Arguments
/// * `config` - The RDF visualization configuration.
///
/// # Returns
/// A `StereotypeStyle` configured for URI nodes.
fn uri_style(config: &RDFVisualizationConfig) -> StereotypeStyle {
    StereotypeStyle::new("uri")
        .with_background_color(config.uri_background_color())
        .with_line_color(config.uri_line_color())
        .with_line_thickness(config.uri_line_thickness())
        .with_round_corner(config.uri_round_corner())
}

/// Creates a stereotype style for blank nodes based on the configuration.
///
/// # Arguments
/// * `config` - The RDF visualization configuration.
///
/// # Returns
/// A `StereotypeStyle` configured for blank nodes.
fn bnode_style(config: &RDFVisualizationConfig) -> StereotypeStyle {
    StereotypeStyle::new("bnode")
        .with_background_color(config.bnode_background_color())
        .with_line_color(config.bnode_line_color())
        .with_line_thickness(config.bnode_line_thickness())
        .with_round_corner(config.bnode_round_corner())
}

/// Creates a stereotype style for asserted triple terms based on the configuration.
///
/// # Arguments
/// * `config` - The RDF visualization configuration.
///
/// # Returns
/// A `StereotypeStyle` configured for asserted triple terms.
fn asserted_style(config: &RDFVisualizationConfig) -> StereotypeStyle {
    StereotypeStyle::new("asserted")
        .with_background_color(config.asserted_background_color())
        .with_line_color(config.asserted_line_color())
        .with_line_thickness(config.asserted_line_thickness())
}

/// Creates a stereotype style for non-asserted triple terms based on the configuration.
///
/// # Arguments
/// * `config` - The RDF visualization configuration.
///
/// # Returns
/// A `StereotypeStyle` configured for non-asserted triple terms.
fn non_asserted_style(config: &RDFVisualizationConfig) -> StereotypeStyle {
    StereotypeStyle::new("non_asserted")
        .with_background_color(config.non_asserted_background_color())
        .with_line_color(config.non_asserted_line_color())
        .with_line_thickness(config.non_asserted_line_thickness())
        .with_round_corner(config.non_asserted_round_corner())
}

impl Default for Style {
    /// Returns a default `Style` instance with no stereotype styles configured.
    ///
    /// This is equivalent to calling `Style::new()`.
    fn default() -> Self {
        Style::new()
    }
}
