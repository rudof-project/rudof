use serde::{Deserialize, Serialize};
use crate::rdf_core::visualizer::style::{
    ArrowStyle, DEFAULT_PREDICATE_ARROW_STYLE, DEFAULT_SUBJECT_ARROW_STYLE, DEFAULT_OBJECT_ARROW_STYLE,
    Style, UmlColor
};

/// Default text subject label used when no custom values are provided.
const DEFAULT_SUBJECT_TEXT: &str = "subj";
/// Default text predicate label used when no custom values are provided.
const DEFAULT_PREDICATE_TEXT: &str = "pred";
/// Default text object label used when no custom values are provided.
const DEFAULT_OBJECT_TEXT: &str = "obj";

/// Enum representing the available UML node shapes for visualization.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
enum UmlShape {
    /// Cloud shape.
    Cloud,
    /// Rectangle shape.
    Rectangle,
}

/// Configuration object controlling the visual appearance of RDF graphs.
///
/// This struct allows customization of node and edge styles, labels, and shapes for different RDF term types.
/// All fields are optional and will fall back to default values if not specified.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct RDFVisualizationConfig {
    // === URI node styling ===
    /// Stroke color of URI nodes.
    uri_line_color: Option<UmlColor>,
    /// Stroke thickness of URI nodes.
    uri_line_thickness: Option<u32>,
    /// Background color of URI nodes.
    uri_background_color: Option<UmlColor>,
    /// Corner radius of URI node shapes.
    uri_round_corner: Option<u32>,

    // === Blank node styling ===
    /// Stroke color of blank nodes.
    bnode_line_color: Option<UmlColor>,
    /// Stroke thickness of blank nodes.
    bnode_line_thickness: Option<u32>,
    /// Background color of blank nodes.
    bnode_background_color: Option<UmlColor>,
    /// Corner radius of blank node shapes.
    bnode_round_corner: Option<u32>,

    // === Literal node styling ===
    /// Stroke color of literal nodes.
    literal_line_color: Option<UmlColor>,
    /// Stroke thickness of literal nodes.
    literal_line_thickness: Option<u32>,
    /// Background color of literal nodes.
    literal_background_color: Option<UmlColor>,
    /// Corner radius of literal node shapes.
    literal_round_corner: Option<u32>,

    // === Reifier node styling ===
    /// Stroke color of reifier nodes.
    reifier_line_color: Option<UmlColor>,
    /// Stroke thickness of reifier nodes.
    reifier_line_thickness: Option<u32>,
    /// Background color of reifier nodes.
    reifier_background_color: Option<UmlColor>,
    /// Corner radius of reifier node shapes.
    reifier_round_corner: Option<u32>,

    // === Asserted triple term styling ===
    /// Stroke color of asserted triple terms.
    asserted_line_color: Option<UmlColor>,
    /// Stroke thickness of asserted triple terms.
    asserted_line_thickness: Option<u32>,
    /// Background color of asserted triple terms.
    asserted_background_color: Option<UmlColor>,
    /// Corner radius of asserted triple term shapes.
    asserted_round_corner: Option<u32>,

    // === Non-asserted triple term styling ===
    /// Stroke color of non-asserted triple terms.
    non_asserted_line_color: Option<UmlColor>,
    /// Stroke thickness of non-asserted triple terms.
    non_asserted_line_thickness: Option<u32>,
    /// Background color of non-asserted triple terms.
    non_asserted_background_color: Option<UmlColor>,
    /// Corner radius of non-asserted triple term shapes.
    non_asserted_round_corner: Option<u32>,

    // === Labels and shapes ===
    /// Label for subject triple term.
    triple_term_subject_label: Option<String>,
    /// Label for predicate triple term.
    triple_term_predicate_label: Option<String>,
    /// Label for object triple term.
    triple_term_object_label: Option<String>,
    /// Label for reification.
    reifies_label: Option<String>,
    /// Shape for unasserted triple.
    unasserted_triple_shape: Option<UmlShape>,
    /// Shape for asserted triple.
    asserted_triple_shape: Option<UmlShape>,

    // === Arrow styles ===
    /// Arrow style for subject.
    subject_arrow_style: Option<ArrowStyle>,
    /// Arrow style for predicate.
    predicate_arrow_style: Option<ArrowStyle>,
    /// Arrow style for object.
    object_arrow_style: Option<ArrowStyle>,

    // === Text for subject, predicate, object ===
    /// Text for subject.
    subject_text: Option<String>,
    /// Text for predicate.
    predicate_text: Option<String>,
    /// Text for object.
    object_text: Option<String>,
}

impl RDFVisualizationConfig {
    /// Creates a new configuration with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the background color for literal nodes.
    pub fn with_literal_background_color(mut self, color: UmlColor) -> Self {
        self.literal_background_color = Some(color);
        self
    }

    /// Sets the label for the subject triple term.
    pub fn with_triple_term_subject_label(mut self, label: String) -> Self {
        self.triple_term_subject_label = Some(label);
        self
    }

    /// Sets the label for the predicate triple term.
    pub fn with_triple_term_predicate_label(mut self, label: String) -> Self {
        self.triple_term_predicate_label = Some(label);
        self
    }

    /// Sets the label for the object triple term.
    pub fn with_triple_term_object_label(mut self, label: String) -> Self {
        self.triple_term_object_label = Some(label);
        self
    }

    /// Sets the label for reification.
    pub fn with_reifies_label(mut self, label: String) -> Self {
        self.reifies_label = Some(label);
        self
    }

    /// Gets the stroke color for URI nodes.
    pub fn uri_line_color(&self) -> UmlColor {
        self.uri_line_color.clone().unwrap_or(URI_LINE_COLOR)
    }

    /// Gets the stroke thickness for URI nodes.
    pub fn uri_line_thickness(&self) -> u32 {
        self.uri_line_thickness.unwrap_or(URI_LINE_THICKNESS)
    }

    /// Gets the background color for URI nodes.
    pub fn uri_background_color(&self) -> UmlColor {
        self.uri_background_color.clone().unwrap_or(URI_BACKGROUND_COLOR)
    }

    /// Gets the corner radius for URI node shapes.
    pub fn uri_round_corner(&self) -> u32 {
        self.uri_round_corner.unwrap_or(URI_ROUND_CORNER)
    }

    /// Gets the stroke color for blank nodes.
    pub fn bnode_line_color(&self) -> UmlColor {
        self.bnode_line_color.clone().unwrap_or(BNODE_LINE_COLOR)
    }

    /// Gets the stroke thickness for blank nodes.
    pub fn bnode_line_thickness(&self) -> u32 {
        self.bnode_line_thickness.unwrap_or(BNODE_LINE_THICKNESS)
    }

    /// Gets the background color for blank nodes.
    pub fn bnode_background_color(&self) -> UmlColor {
        self.bnode_background_color.clone().unwrap_or(BNODE_BACKGROUND_COLOR)
    }

    /// Gets the corner radius for blank node shapes.
    pub fn bnode_round_corner(&self) -> u32 {
        self.bnode_round_corner.unwrap_or(BNODE_ROUND_CORNER)
    }

    /// Gets the stroke color for literal nodes.
    pub fn literal_line_color(&self) -> UmlColor {
        self.literal_line_color.clone().unwrap_or(LITERAL_LINE_COLOR)
    }

    /// Gets the stroke thickness for literal nodes.
    pub fn literal_line_thickness(&self) -> u32 {
        self.literal_line_thickness.unwrap_or(LITERAL_LINE_THICKNESS)
    }

    /// Gets the background color for literal nodes.
    pub fn literal_background_color(&self) -> UmlColor {
        self.literal_background_color
            .clone()
            .unwrap_or(LITERAL_BACKGROUND_COLOR)
    }

    /// Gets the corner radius for literal node shapes.
    pub fn literal_round_corner(&self) -> u32 {
        self.literal_round_corner.unwrap_or(LITERAL_ROUND_CORNER)
    }

    /// Gets the stroke color for reifier nodes.
    pub fn reifier_line_color(&self) -> UmlColor {
        self.reifier_line_color.clone().unwrap_or(REIFIER_LINE_COLOR)
    }

    /// Gets the stroke thickness for reifier nodes.
    pub fn reifier_line_thickness(&self) -> u32 {
        self.reifier_line_thickness.unwrap_or(REIFIER_LINE_THICKNESS)
    }

    /// Gets the background color for reifier nodes.
    pub fn reifier_background_color(&self) -> UmlColor {
        self.reifier_background_color
            .clone()
            .unwrap_or(REIFIER_BACKGROUND_COLOR)
    }

    /// Gets the corner radius for reifier node shapes.
    pub fn reifier_round_corner(&self) -> u32 {
        self.reifier_round_corner.unwrap_or(REIFIER_ROUND_CORNER)
    }

    /// Gets the stroke color for asserted triple terms.
    pub fn asserted_line_color(&self) -> UmlColor {
        self.asserted_line_color.clone().unwrap_or(URI_LINE_COLOR)
    }

    /// Gets the stroke thickness for asserted triple terms.
    pub fn asserted_line_thickness(&self) -> u32 {
        self.asserted_line_thickness.unwrap_or(URI_LINE_THICKNESS)
    }

    /// Gets the background color for asserted triple terms.
    pub fn asserted_background_color(&self) -> UmlColor {
        self.asserted_background_color.clone().unwrap_or(URI_BACKGROUND_COLOR)
    }

    /// Gets the corner radius for asserted triple term shapes.
    pub fn asserted_round_corner(&self) -> u32 {
        self.asserted_round_corner.unwrap_or(URI_ROUND_CORNER)
    }

    /// Gets the stroke color for non-asserted triple terms.
    pub fn non_asserted_line_color(&self) -> UmlColor {
        self.non_asserted_line_color.clone().unwrap_or(BNODE_LINE_COLOR)
    }

    /// Gets the stroke thickness for non-asserted triple terms.
    pub fn non_asserted_line_thickness(&self) -> u32 {
        self.non_asserted_line_thickness.unwrap_or(BNODE_LINE_THICKNESS)
    }

    /// Gets the background color for non-asserted triple terms.
    pub fn non_asserted_background_color(&self) -> UmlColor {
        self.non_asserted_background_color
            .clone()
            .unwrap_or(BNODE_BACKGROUND_COLOR)
    }

    /// Gets the corner radius for non-asserted triple term shapes.
    pub fn non_asserted_round_corner(&self) -> u32 {
        self.non_asserted_round_corner.unwrap_or(BNODE_ROUND_CORNER)
    }

    /// Returns a `Style` object constructed from this configuration.
    pub fn get_style(&self) -> Style {
        Style::from_config(self)
    }

    /// Gets the arrow style for subject edges.
    pub fn get_subject_arrow_style(&self) -> ArrowStyle {
        self.subject_arrow_style.clone().unwrap_or(DEFAULT_SUBJECT_ARROW_STYLE)
    }

    /// Gets the label text for subject edges.
    pub fn get_subject_text(&self) -> String {
        self.subject_text.clone().unwrap_or(DEFAULT_SUBJECT_TEXT.into())
    }

    /// Gets the arrow style for predicate edges.
    pub fn get_predicate_arrow_style(&self) -> ArrowStyle {
        self.predicate_arrow_style
            .clone()
            .unwrap_or(DEFAULT_PREDICATE_ARROW_STYLE)
    }

    /// Gets the label text for predicate edges.
    pub fn get_predicate_text(&self) -> String {
        self.predicate_text.clone().unwrap_or(DEFAULT_PREDICATE_TEXT.into())
    }

    /// Gets the arrow style for object edges.
    pub fn get_object_arrow_style(&self) -> ArrowStyle {
        self.object_arrow_style.clone().unwrap_or(DEFAULT_OBJECT_ARROW_STYLE)
    }

    /// Gets the label text for object edges.
    pub fn get_object_text(&self) -> String {
        self.object_text.clone().unwrap_or(DEFAULT_OBJECT_TEXT.into())
    }
}

// === Default values for visualizer configuration ===

/// Default stroke color for URI nodes.
const URI_LINE_COLOR: UmlColor = UmlColor::Blue;
/// Default stroke thickness for URI nodes.
const URI_LINE_THICKNESS: u32 = 1;
/// Default background color for URI nodes.
const URI_BACKGROUND_COLOR: UmlColor = UmlColor::White;
/// Default corner radius for URI node shapes.
const URI_ROUND_CORNER: u32 = 25;


/// Default stroke color for blank nodes.
const BNODE_LINE_COLOR: UmlColor = UmlColor::Blue;
/// Default stroke thickness for blank nodes.
const BNODE_LINE_THICKNESS: u32 = 1;
/// Default background color for blank nodes.
const BNODE_BACKGROUND_COLOR: UmlColor = UmlColor::Gray;
/// Default corner radius for blank node shapes.
const BNODE_ROUND_CORNER: u32 = 25;

/// Default stroke color for literal nodes.
const LITERAL_LINE_COLOR: UmlColor = UmlColor::Black;
/// Default stroke thickness for literal nodes.
const LITERAL_LINE_THICKNESS: u32 = 1;
/// Default background color for literal nodes.
const LITERAL_BACKGROUND_COLOR: UmlColor = UmlColor::Cyan;
/// Default corner radius for literal node shapes.
const LITERAL_ROUND_CORNER: u32 = 0;

/// Default stroke color for reifier nodes.
const REIFIER_LINE_COLOR: UmlColor = UmlColor::Black;
/// Default stroke thickness for reifier nodes.
const REIFIER_LINE_THICKNESS: u32 = 1;
/// Default background color for reifier nodes.
const REIFIER_BACKGROUND_COLOR: UmlColor = UmlColor::Yellow;
/// Default corner radius for reifier node shapes.
const REIFIER_ROUND_CORNER: u32 = 0;

/// Default stroke color for asserted triple terms.
const ASSERTED_LINE_COLOR: UmlColor = UmlColor::Black;
/// Default stroke thickness for asserted triple terms.
const ASSERTED_LINE_THICKNESS: u32 = 2;
/// Default background color for asserted triple terms.
const ASSERTED_BACKGROUND_COLOR: UmlColor = UmlColor::White;
/// Default corner radius for asserted triple term shapes.
const ASSERTED_ROUND_CORNER: u32 = 0;

/// Default stroke color for non-asserted triple terms.
const NON_ASSERTED_LINE_COLOR: UmlColor = UmlColor::Blue;
/// Default stroke thickness for non-asserted triple terms.
const NON_ASSERTED_LINE_THICKNESS: u32 = 2;
/// Default background color for non-asserted triple terms.
const NON_ASSERTED_BACKGROUND_COLOR: UmlColor = UmlColor::White;
/// Default corner radius for non-asserted triple term shapes.
const NON_ASSERTED_ROUND_CORNER: u32 = 0;

/// Default label for subject triple term.
const TRIPLE_TERM_SUBJECT_LABEL: &str = "subject";
/// Default label for predicate triple term.
const TRIPLE_TERM_PREDICATE_LABEL: &str = "predicate";
/// Default label for object triple term.
const TRIPLE_TERM_OBJECT_LABEL: &str = "object";
/// Default label for reification.
const REIFIES_LABEL: &str = "reifies";

/// Default shape for asserted triple.
const ASSERTED_TRIPLE_SHAPE: UmlShape = UmlShape::Rectangle;
/// Default shape for non-asserted triple.
const NON_ASSERTED_TRIPLE_SHAPE: UmlShape = UmlShape::Cloud;

impl Default for RDFVisualizationConfig {
    /// Returns a configuration with all fields set to their default values.
    fn default() -> Self {
        RDFVisualizationConfig {
            uri_line_color: Some(URI_LINE_COLOR),
            uri_line_thickness: Some(URI_LINE_THICKNESS),
            uri_background_color: Some(URI_BACKGROUND_COLOR),
            uri_round_corner: Some(URI_ROUND_CORNER),

            bnode_line_color: Some(BNODE_LINE_COLOR),
            bnode_line_thickness: Some(BNODE_LINE_THICKNESS),
            bnode_background_color: Some(BNODE_BACKGROUND_COLOR),
            bnode_round_corner: Some(BNODE_ROUND_CORNER),

            literal_line_color: Some(LITERAL_LINE_COLOR),
            literal_line_thickness: Some(LITERAL_LINE_THICKNESS),
            literal_background_color: Some(LITERAL_BACKGROUND_COLOR),
            literal_round_corner: Some(LITERAL_ROUND_CORNER),

            reifier_line_color: Some(REIFIER_LINE_COLOR),
            reifier_line_thickness: Some(REIFIER_LINE_THICKNESS),
            reifier_background_color: Some(REIFIER_BACKGROUND_COLOR),
            reifier_round_corner: Some(REIFIER_ROUND_CORNER),

            asserted_line_color: Some(ASSERTED_LINE_COLOR),
            asserted_line_thickness: Some(ASSERTED_LINE_THICKNESS),
            asserted_background_color: Some(ASSERTED_BACKGROUND_COLOR),
            asserted_round_corner: Some(ASSERTED_ROUND_CORNER),

            non_asserted_line_color: Some(NON_ASSERTED_LINE_COLOR),
            non_asserted_line_thickness: Some(NON_ASSERTED_LINE_THICKNESS),
            non_asserted_background_color: Some(NON_ASSERTED_BACKGROUND_COLOR),
            non_asserted_round_corner: Some(NON_ASSERTED_ROUND_CORNER),

            triple_term_subject_label: Some(TRIPLE_TERM_SUBJECT_LABEL.into()),
            triple_term_predicate_label: Some(TRIPLE_TERM_PREDICATE_LABEL.into()),
            triple_term_object_label: Some(TRIPLE_TERM_OBJECT_LABEL.into()),

            reifies_label: Some(REIFIES_LABEL.into()),
            unasserted_triple_shape: Some(NON_ASSERTED_TRIPLE_SHAPE),
            asserted_triple_shape: Some(ASSERTED_TRIPLE_SHAPE),

            subject_arrow_style: Some(DEFAULT_SUBJECT_ARROW_STYLE),
            predicate_arrow_style: Some(DEFAULT_PREDICATE_ARROW_STYLE),
            object_arrow_style: Some(DEFAULT_OBJECT_ARROW_STYLE),

            subject_text: Some(DEFAULT_SUBJECT_TEXT.into()),
            predicate_text: Some(DEFAULT_PREDICATE_TEXT.into()),
            object_text: Some(DEFAULT_OBJECT_TEXT.into()),
        }
    }
}
