use serde::{Deserialize, Serialize};

use crate::rdf_visualizer::{style::Style, uml_color::UmlColor};

/// RDF Visualization config
/// Contains values for customizing the appearance of RDF visualizations.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct RDFVisualizationConfig {
    /// URI nodes
    uri_line_color: Option<UmlColor>,
    uri_line_thickness: Option<u32>,
    uri_background_color: Option<UmlColor>,
    uri_round_corner: Option<u32>,

    /// Blank nodes
    bnode_line_color: Option<UmlColor>,
    bnode_line_thickness: Option<u32>,
    bnode_background_color: Option<UmlColor>,
    bnode_round_corner: Option<u32>,

    /// Literals
    literal_line_color: Option<UmlColor>,
    literal_line_thickness: Option<u32>,
    literal_background_color: Option<UmlColor>,
    literal_round_corner: Option<u32>,

    /// Reifier nodes
    reifier_line_color: Option<UmlColor>,
    reifier_line_thickness: Option<u32>,
    reifier_background_color: Option<UmlColor>,
    reifier_round_corner: Option<u32>,

    /// Asserted triple terms
    asserted_line_color: Option<UmlColor>,
    asserted_line_thickness: Option<u32>,
    asserted_background_color: Option<UmlColor>,
    asserted_round_corner: Option<u32>,

    /// Non-asserted triple terms
    non_asserted_line_color: Option<UmlColor>,
    non_asserted_line_thickness: Option<u32>,
    non_asserted_background_color: Option<UmlColor>,
    non_asserted_round_corner: Option<u32>,

    // Labels
    triple_term_subject_label: Option<String>,
    triple_term_predicate_label: Option<String>,
    triple_term_object_label: Option<String>,
    reifies_label: Option<String>,
    unasserted_triple_shape: Option<UmlShape>,
    asserted_triple_shape: Option<UmlShape>,
}

impl RDFVisualizationConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_literal_background_color(mut self, color: UmlColor) -> Self {
        self.literal_background_color = Some(color);
        self
    }

    pub fn with_triple_term_subject_label(mut self, label: String) -> Self {
        self.triple_term_subject_label = Some(label);
        self
    }

    pub fn with_triple_term_predicate_label(mut self, label: String) -> Self {
        self.triple_term_predicate_label = Some(label);
        self
    }

    pub fn with_triple_term_object_label(mut self, label: String) -> Self {
        self.triple_term_object_label = Some(label);
        self
    }

    pub fn with_reifies_label(mut self, label: String) -> Self {
        self.reifies_label = Some(label);
        self
    }

    pub fn uri_line_color(&self) -> UmlColor {
        self.uri_line_color.clone().unwrap_or(URI_LINE_COLOR)
    }

    pub fn uri_line_thickness(&self) -> u32 {
        self.uri_line_thickness.unwrap_or(URI_LINE_THICKNESS)
    }

    pub fn uri_background_color(&self) -> UmlColor {
        self.uri_background_color
            .clone()
            .unwrap_or(URI_BACKGROUND_COLOR)
    }

    pub fn uri_round_corner(&self) -> u32 {
        self.uri_round_corner.unwrap_or(URI_ROUND_CORNER)
    }

    pub fn bnode_line_color(&self) -> UmlColor {
        self.bnode_line_color.clone().unwrap_or(BNODE_LINE_COLOR)
    }

    pub fn bnode_line_thickness(&self) -> u32 {
        self.bnode_line_thickness.unwrap_or(BNODE_LINE_THICKNESS)
    }

    pub fn bnode_background_color(&self) -> UmlColor {
        self.bnode_background_color
            .clone()
            .unwrap_or(BNODE_BACKGROUND_COLOR)
    }

    pub fn bnode_round_corner(&self) -> u32 {
        self.bnode_round_corner.unwrap_or(BNODE_ROUND_CORNER)
    }

    pub fn literal_line_color(&self) -> UmlColor {
        self.literal_line_color
            .clone()
            .unwrap_or(LITERAL_LINE_COLOR)
    }

    pub fn literal_line_thickness(&self) -> u32 {
        self.literal_line_thickness
            .unwrap_or(LITERAL_LINE_THICKNESS)
    }

    pub fn literal_background_color(&self) -> UmlColor {
        self.literal_background_color
            .clone()
            .unwrap_or(LITERAL_BACKGROUND_COLOR)
    }

    pub fn literal_round_corner(&self) -> u32 {
        self.literal_round_corner.unwrap_or(LITERAL_ROUND_CORNER)
    }

    pub fn reifier_line_color(&self) -> UmlColor {
        self.reifier_line_color
            .clone()
            .unwrap_or(REIFIER_LINE_COLOR)
    }

    pub fn reifier_line_thickness(&self) -> u32 {
        self.reifier_line_thickness
            .unwrap_or(REIFIER_LINE_THICKNESS)
    }

    pub fn reifier_background_color(&self) -> UmlColor {
        self.reifier_background_color
            .clone()
            .unwrap_or(REIFIER_BACKGROUND_COLOR)
    }

    pub fn reifier_round_corner(&self) -> u32 {
        self.reifier_round_corner.unwrap_or(REIFIER_ROUND_CORNER)
    }

    pub fn asserted_line_color(&self) -> UmlColor {
        self.asserted_line_color.clone().unwrap_or(URI_LINE_COLOR)
    }

    pub fn asserted_line_thickness(&self) -> u32 {
        self.asserted_line_thickness.unwrap_or(URI_LINE_THICKNESS)
    }

    pub fn asserted_background_color(&self) -> UmlColor {
        self.asserted_background_color
            .clone()
            .unwrap_or(URI_BACKGROUND_COLOR)
    }

    pub fn asserted_round_corner(&self) -> u32 {
        self.asserted_round_corner.unwrap_or(URI_ROUND_CORNER)
    }

    pub fn non_asserted_line_color(&self) -> UmlColor {
        self.non_asserted_line_color
            .clone()
            .unwrap_or(BNODE_LINE_COLOR)
    }

    pub fn non_asserted_line_thickness(&self) -> u32 {
        self.non_asserted_line_thickness
            .unwrap_or(BNODE_LINE_THICKNESS)
    }

    pub fn non_asserted_background_color(&self) -> UmlColor {
        self.non_asserted_background_color
            .clone()
            .unwrap_or(BNODE_BACKGROUND_COLOR)
    }

    pub fn non_asserted_round_corner(&self) -> u32 {
        self.non_asserted_round_corner.unwrap_or(BNODE_ROUND_CORNER)
    }

    pub fn get_style(&self) -> Style {
        Style::from_config(self)
    }
}

// Default values

const URI_LINE_COLOR: UmlColor = UmlColor::Blue;
const URI_LINE_THICKNESS: u32 = 1;
const URI_BACKGROUND_COLOR: UmlColor = UmlColor::White;
const URI_ROUND_CORNER: u32 = 25;

const BNODE_LINE_COLOR: UmlColor = UmlColor::Blue;
const BNODE_LINE_THICKNESS: u32 = 1;
const BNODE_BACKGROUND_COLOR: UmlColor = UmlColor::Gray;
const BNODE_ROUND_CORNER: u32 = 25;

const LITERAL_LINE_COLOR: UmlColor = UmlColor::Black;
const LITERAL_LINE_THICKNESS: u32 = 1;
const LITERAL_BACKGROUND_COLOR: UmlColor = UmlColor::Cyan;
const LITERAL_ROUND_CORNER: u32 = 0;

const REIFIER_LINE_COLOR: UmlColor = UmlColor::Black;
const REIFIER_LINE_THICKNESS: u32 = 1;
const REIFIER_BACKGROUND_COLOR: UmlColor = UmlColor::Yellow;
const REIFIER_ROUND_CORNER: u32 = 0;

const ASSERTED_LINE_COLOR: UmlColor = UmlColor::Black;
const ASSERTED_LINE_THICKNESS: u32 = 2;
const ASSERTED_BACKGROUND_COLOR: UmlColor = UmlColor::White;
const ASSERTED_ROUND_CORNER: u32 = 0;

const NON_ASSERTED_LINE_COLOR: UmlColor = UmlColor::Blue;
const NON_ASSERTED_LINE_THICKNESS: u32 = 2;
const NON_ASSERTED_BACKGROUND_COLOR: UmlColor = UmlColor::White;
const NON_ASSERTED_ROUND_CORNER: u32 = 0;

const TRIPLE_TERM_SUBJECT_LABEL: &str = "subject";
const TRIPLE_TERM_PREDICATE_LABEL: &str = "predicate";
const TRIPLE_TERM_OBJECT_LABEL: &str = "object";
const REIFIES_LABEL: &str = "reifies";

const ASSERTED_TRIPLE_SHAPE: UmlShape = UmlShape::Rectangle;
const NON_ASSERTED_TRIPLE_SHAPE: UmlShape = UmlShape::Cloud;

impl Default for RDFVisualizationConfig {
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
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum UmlShape {
    Cloud,
    Rectangle,
}
