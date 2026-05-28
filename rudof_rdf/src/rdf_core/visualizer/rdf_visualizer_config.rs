use crate::rdf_core::visualizer::style::{ArrowStyle, Style, UmlColor, ThicknessStyle};
use serde::{Deserialize, Serialize};

/// Enum representing the available UML node shapes for visualization.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
#[serde(rename = "snake_case")]
pub enum UmlShape {
    /// Cloud shape.
    Cloud,
    /// Rectangle shape.
    #[default]
    Rectangle,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct NodeStyle {
    #[serde(rename = "line_color", default = "NodeStyle::default_line_color", skip_serializing_if = "NodeStyle::is_default_line_color")]
    pub(crate) line_color: UmlColor,
    #[serde(rename = "line_thickness", default = "NodeStyle::default_line_thickness", skip_serializing_if = "NodeStyle::is_default_line_thickness")]
    pub(crate) line_thickness: u32,
    #[serde(rename = "background_color", default = "NodeStyle::default_background_color", skip_serializing_if = "NodeStyle::is_default_background_color")]
    pub(crate) background_color: UmlColor,
    #[serde(rename = "round_corner", default = "NodeStyle::default_round_corner", skip_serializing_if = "NodeStyle::is_default_round_corner")]
    pub(crate) round_corner: u32,
}

impl NodeStyle {
    pub fn new() -> Self {
        Self {
            round_corner: Self::default_round_corner(),
            line_color: Self::default_line_color(),
            line_thickness: Self::default_line_thickness(),
            background_color: Self::default_background_color(),
        }
    }

    pub fn with_line_color(mut self, color: UmlColor) -> Self {
        self.line_color = color;
        self
    }

    pub fn with_line_thickness(mut self, v: u32) -> Self {
        self.line_thickness = v;
        self
    }

    pub fn with_background_color(mut self, color: UmlColor) -> Self {
        self.background_color = color;
        self
    }

    pub fn with_round_corner(mut self, v: u32) -> Self {
        self.round_corner = v;
        self
    }
}

impl NodeStyle {
    pub fn line_color(&self) -> &UmlColor {
        &self.line_color
    }

    pub fn line_thickness(&self) -> u32 {
        self.line_thickness
    }

    pub fn background_color(&self) -> &UmlColor {
        &self.background_color
    }

    pub fn round_corner(&self) -> u32 {
        self.round_corner
    }
}

/// Serde stuff
#[allow(dead_code)]
impl NodeStyle {
    #[inline] fn default_line_color() -> UmlColor { UmlColor::Black }
    #[inline] fn default_line_thickness() -> u32 { 10 }
    #[inline] fn default_background_color() -> UmlColor { UmlColor::White }
    #[inline] fn default_round_corner() -> u32 { 0 }
    #[inline] fn is_default_line_color(value: &UmlColor) -> bool { value == &Self::default_line_color() }
    #[inline] fn is_default_line_thickness(value: &u32) -> bool { value == &Self::default_line_thickness() }
    #[inline] fn is_default_background_color(value: &UmlColor) -> bool { value == &Self::default_background_color() }
    #[inline] fn is_default_round_corner(value: &u32) -> bool { value == &Self::default_round_corner() }
}

impl Default for NodeStyle {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration object controlling the visual appearance of RDF graphs.
///
/// This struct allows customization of node and edge styles, labels, and shapes for different RDF term types.
/// All fields are optional and will fall back to default values if not specified.
#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct RDFVisualizationConfig {
    // === URI node styling ===
    #[serde(rename = "uri_style", default = "RDFVisualizationConfig::default_uri_style")]
    pub(crate) uri_style: NodeStyle,

    // === Blank node styling ===
    #[serde(rename = "bnode_style", default = "RDFVisualizationConfig::default_bnode_style")]
    pub(crate) bnode_style: NodeStyle,

    // === Literal node styling ===
    #[serde(rename = "literal_style", default = "RDFVisualizationConfig::default_literal_style")]
    pub(crate) literal_style: NodeStyle,

    // === Reifier node styling ===
    #[serde(rename = "reifier_style", default = "RDFVisualizationConfig::default_reifier_style")]
    pub(crate) reifier_style: NodeStyle,

    // === Asserted triple term styling ===
    #[serde(rename = "asserted_style", default = "RDFVisualizationConfig::default_asserted_style")]
    pub(crate) asserted_style: NodeStyle,

    // === Non-asserted triple term styling ===
    #[serde(rename = "non_asserted_style", default = "RDFVisualizationConfig::default_non_asserted_style")]
    pub(crate) non_asserted_style: NodeStyle,

    // === Labels and shapes ===
    /// Label for subject triple term.
    #[serde(rename = "subject_label", default = "RDFVisualizationConfig::default_subject_label")]
    pub(crate) triple_term_subject_label: String,
    /// Label for predicate triple term.
    #[serde(rename = "predicate_label", default = "RDFVisualizationConfig::default_predicate_label")]
    pub(crate) triple_term_predicate_label: String,
    /// Label for object triple term.
    #[serde(rename = "object_label", default = "RDFVisualizationConfig::default_object_label")]
    pub(crate) triple_term_object_label: String,
    /// Label for reification.
    #[serde(rename = "reifies_label", default = "RDFVisualizationConfig::default_reifies_label")]
    pub(crate) reifies_label: String,
    /// Shape for unasserted triple.
    #[serde(rename = "unasserted_triple_shape", default = "RDFVisualizationConfig::default_unasserted_triple_shape")]
    pub(crate) unasserted_triple_shape: UmlShape,
    /// Shape for asserted triple.
    #[serde(rename = "asserted_triple_shape", default = "RDFVisualizationConfig::default_asserted_triple_shape")]
    pub(crate) asserted_triple_shape: UmlShape,

    // === Arrow styles ===
    /// Arrow style for subject.
    #[serde(rename = "subject_arrow_style", default = "RDFVisualizationConfig::default_subject_arrow_style")]
    pub(crate) subject_arrow_style: ArrowStyle,
    /// Arrow style for predicate.
    #[serde(rename = "predicate_arrow_style", default = "RDFVisualizationConfig::default_predicate_arrow_style")]
    pub(crate) predicate_arrow_style: ArrowStyle,
    /// Arrow style for object.
    #[serde(rename = "object_arrow_style", default = "RDFVisualizationConfig::default_object_arrow_style")]
    pub(crate) object_arrow_style: ArrowStyle,

    // === Text for subject, predicate, object ===
    /// Text for subject.
    #[serde(rename = "subject_text", default = "RDFVisualizationConfig::default_subject_text")]
    pub(crate) subject_text: String,
    /// Text for predicate.
    #[serde(rename = "predicate_text", default = "RDFVisualizationConfig::default_predicate_text")]
    pub(crate) predicate_text: String,
    /// Text for object.
    #[serde(rename = "object_text", default = "RDFVisualizationConfig::default_object_text")]
    pub(crate) object_text: String,
}

/// Serde stuff
#[allow(dead_code)]
impl RDFVisualizationConfig {
    #[inline] fn default_uri_style() -> NodeStyle {
        NodeStyle {
            line_color: UmlColor::Blue,
            line_thickness: 1,
            background_color: UmlColor::White,
            round_corner: 25,
        }
    }
    #[inline] fn default_bnode_style() -> NodeStyle {
        NodeStyle {
            line_color: UmlColor::Blue,
            line_thickness: 1,
            background_color: UmlColor::Gray,
            round_corner: 25,
        }
    }
    #[inline] fn default_literal_style() -> NodeStyle {
        NodeStyle {
            line_color: UmlColor::Black,
            line_thickness: 1,
            background_color: UmlColor::Cyan,
            round_corner: 0,
        }
    }
    #[inline] fn default_reifier_style() -> NodeStyle {
        NodeStyle {
            line_color: UmlColor::Black,
            line_thickness: 1,
            background_color: UmlColor::Yellow,
            round_corner: 0,
        }
    }
    #[inline] fn default_asserted_style() -> NodeStyle {
        NodeStyle {
            line_color: UmlColor::Black,
            line_thickness: 2,
            background_color: UmlColor::White,
            round_corner: 0,
        }
    }
    #[inline] fn default_non_asserted_style() -> NodeStyle {
        NodeStyle {
            line_color: UmlColor::Blue,
            line_thickness: 2,
            background_color: UmlColor::White,
            round_corner: 0,
        }
    }

    #[inline] fn default_subject_label() -> String { "subject".to_string() }
    #[inline] fn default_predicate_label() -> String { "predicate".to_string() }
    #[inline] fn default_object_label() -> String { "object".to_string() }
    #[inline] fn default_reifies_label() -> String { "reifies".to_string() }

    #[inline] fn default_unasserted_triple_shape() -> UmlShape { UmlShape::Cloud }
    #[inline] fn default_asserted_triple_shape() -> UmlShape { UmlShape::Rectangle }

    #[inline] fn default_subject_arrow_style() -> ArrowStyle {
        ArrowStyle {
            line_color: UmlColor::Blue,
            text_color: UmlColor::Blue,
            line_thickness: ThicknessStyle::Dashed,
        }
    }
    #[inline] fn default_predicate_arrow_style() -> ArrowStyle {
        ArrowStyle {
            line_color: UmlColor::Red,
            text_color: UmlColor::Red,
            line_thickness: ThicknessStyle::Dashed,
        }
    }
    #[inline] fn default_object_arrow_style() -> ArrowStyle {
        ArrowStyle {
            line_color: UmlColor::Green,
            text_color: UmlColor::Green,
            line_thickness: ThicknessStyle::Dashed,
        }
    }

    #[inline] fn default_subject_text() -> String { "subj".to_string() }
    #[inline] fn default_predicate_text() -> String { "pred".to_string() }
    #[inline] fn default_object_text() -> String { "obj".to_string() }
}

impl RDFVisualizationConfig {
    /// Creates a new configuration with default values.
    pub fn new() -> Self {
        Self {
            asserted_style: Self::default_asserted_style(),
            uri_style: Self::default_uri_style(),
            bnode_style: Self::default_bnode_style(),
            literal_style: Self::default_literal_style(),
            non_asserted_style: Self::default_non_asserted_style(),
            reifier_style: Self::default_reifier_style(),

            triple_term_subject_label: Self::default_subject_label(),
            triple_term_predicate_label: Self::default_predicate_label(),
            triple_term_object_label: Self::default_object_label(),
            reifies_label: Self::default_reifies_label(),

            unasserted_triple_shape: Self::default_unasserted_triple_shape(),
            asserted_triple_shape: Self::default_asserted_triple_shape(),

            subject_arrow_style: Self::default_subject_arrow_style(),
            predicate_arrow_style: Self::default_predicate_arrow_style(),
            object_arrow_style: Self::default_object_arrow_style(),

            subject_text: Self::default_subject_text(),
            predicate_text: Self::default_predicate_text(),
            object_text: Self::default_object_text(),
        }
    }

    /// Sets the asserted nodes style
    pub fn with_asserted(mut self, style: NodeStyle) -> Self {
        self.asserted_style = style;
        self
    }

    /// Sets the uri nodes style
    pub fn with_uri(mut self, style: NodeStyle) -> Self {
        self.uri_style = style;
        self
    }

    /// Sets the blank nodes style
    pub fn with_bnode(mut self, style: NodeStyle) -> Self {
        self.bnode_style = style;
        self
    }

    /// Sets the literal nodes style
    pub fn with_literal(mut self, style: NodeStyle) -> Self {
        self.literal_style = style;
        self
    }

    /// Sets the non asserted nodes style
    pub fn with_non_asserted(mut self, style: NodeStyle) -> Self {
        self.non_asserted_style = style;
        self
    }

    /// Sets the reifier nodes style
    pub fn with_reifier(mut self, style: NodeStyle) -> Self {
        self.reifier_style = style;
        self
    }

    /// Sets the subject triple term label
    pub fn with_triple_term_subject_label(mut self, label: String) -> Self {
        self.triple_term_subject_label = label;
        self
    }

    /// Sets the predicate triple term label
    pub fn with_triple_term_predicate_label(mut self, label: String) -> Self {
        self.triple_term_predicate_label = label;
        self
    }

    /// Sets the object triple term label
    pub fn with_triple_term_object_label(mut self, label: String) -> Self {
        self.triple_term_object_label = label;
        self
    }

    /// Sets the reification label
    pub fn with_reifies_label(mut self, label: String) -> Self {
        self.reifies_label = label;
        self
    }

    /// Sets the unasserted triples shape
    pub fn with_unasserted_triple_shapes(mut self, shape: UmlShape) -> Self {
        self.unasserted_triple_shape = shape;
        self
    }

    /// Sets the asserted triples shape
    pub fn with_asserted_triple_shapes(mut self, shape: UmlShape) -> Self {
        self.asserted_triple_shape = shape;
        self
    }

    // Sets the subject arrow style
    pub fn with_subject_arrow_style(mut self, style: ArrowStyle) -> Self {
        self.subject_arrow_style = style;
        self
    }

    /// Sets the predicate arrow style
    pub fn with_predicate_arrow_style(mut self, style: ArrowStyle) -> Self {
        self.predicate_arrow_style = style;
        self
    }

    /// Sets the object arrow style
    pub fn with_object_arrow_style(mut self, style: ArrowStyle) -> Self {
        self.object_arrow_style = style;
        self
    }

    /// Sets the subject text
    pub fn with_subject_text(mut self, text: String) -> Self {
        self.subject_text = text;
        self
    }

    /// Sets the predicate text
    pub fn with_predicate_text(mut self, text: String) -> Self {
        self.predicate_text = text;
        self
    }

    /// Sets the object text
    pub fn with_object_text(mut self, text: String) -> Self {
        self.object_text = text;
        self
    }
}

impl RDFVisualizationConfig {
    /// Gets the uri nodes style
    pub fn uri_style(&self) -> &NodeStyle {
        &self.uri_style
    }

    /// Gets the blank nodes style
    pub fn bnode_style(&self) -> &NodeStyle {
        &self.bnode_style
    }

    /// Gets the literal nodes style
    pub fn literal_style(&self) -> &NodeStyle {
        &self.literal_style
    }

    /// Gets the reifier nodes style
    pub fn reifier_style(&self) -> &NodeStyle {
        &self.reifier_style
    }

    /// Gets the asserted nodes style
    pub fn asserted_style(&self) -> &NodeStyle {
        &self.asserted_style
    }

    /// Gets the non asserted nodes style
    pub fn non_asserted_style(&self) -> &NodeStyle {
        &self.non_asserted_style
    }

    /// Gets the subject triple term label
    pub fn triple_term_subject_label(&self) -> &String {
        &self.triple_term_subject_label
    }

    /// Gets the predicate triple term label
    pub fn triple_term_predicate_label(&self) -> &String {
        &self.triple_term_predicate_label
    }

    /// Gets the object triple term label
    pub fn triple_term_object_label(&self) -> &String {
        &self.triple_term_object_label
    }

    /// Gets the reification label
    pub fn reifies_label(&self) -> &String {
        &self.reifies_label
    }

    /// Gets the unasserted triples shape
    pub fn unasserted_triple_shape(&self) -> &UmlShape {
        &self.unasserted_triple_shape
    }

    /// Gets the asserted triples shape
    pub fn asserted_triple_shape(&self) -> &UmlShape {
        &self.asserted_triple_shape
    }

    /// Gets the subject arrow style
    pub fn subject_arrow_style(&self) -> &ArrowStyle {
        &self.subject_arrow_style
    }

    /// Gets the predicate arrow style
    pub fn predicate_arrow_style(&self) -> &ArrowStyle {
        &self.predicate_arrow_style
    }

    /// Gets the object arrow style
    pub fn object_arrow_style(&self) -> &ArrowStyle {
        &self.object_arrow_style
    }

    /// Gets the subject text
    pub fn subject_text(&self) -> &String {
        &self.subject_text
    }

    /// Gets the predicate text
    pub fn predicate_text(&self) -> &String {
        &self.predicate_text
    }

    /// Gets the object text
    pub fn object_text(&self) -> &String {
        &self.object_text
    }
}

impl From<RDFVisualizationConfig> for Style {
    fn from(value: RDFVisualizationConfig) -> Self {
        Style::from_config(&value)
    }
}

impl Default for RDFVisualizationConfig {
    fn default() -> Self {
        Self::new()
    }
}
