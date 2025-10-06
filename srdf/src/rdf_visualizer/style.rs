use crate::rdf_visualizer::{
    rdf_visualizer_config::RDFVisualizationConfig, stereotype_style::StereotypeStyle,
};

pub struct Style {
    stereotype_styles: Vec<StereotypeStyle>,
}

impl Style {
    pub fn new() -> Self {
        Style {
            stereotype_styles: Vec::new(),
        }
    }

    pub fn add_stereotype_style(&mut self, style: StereotypeStyle) {
        self.stereotype_styles.push(style);
    }

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

fn reifier_style(config: &RDFVisualizationConfig) -> StereotypeStyle {
    StereotypeStyle::new("reifier")
        .with_background_color(config.reifier_background_color())
        .with_line_color(config.reifier_line_color())
        .with_line_thickness(config.reifier_line_thickness())
}

fn literal_style(config: &RDFVisualizationConfig) -> StereotypeStyle {
    StereotypeStyle::new("literal")
        .with_background_color(config.literal_background_color())
        .with_line_color(config.literal_line_color())
        .with_line_thickness(config.literal_line_thickness())
}

fn uri_style(config: &RDFVisualizationConfig) -> StereotypeStyle {
    StereotypeStyle::new("uri")
        .with_background_color(config.uri_background_color())
        .with_line_color(config.uri_line_color())
        .with_line_thickness(config.uri_line_thickness())
        .with_round_corner(config.uri_round_corner())
}

fn bnode_style(config: &RDFVisualizationConfig) -> StereotypeStyle {
    StereotypeStyle::new("bnode")
        .with_background_color(config.bnode_background_color())
        .with_line_color(config.bnode_line_color())
        .with_line_thickness(config.bnode_line_thickness())
        .with_round_corner(config.bnode_round_corner())
}

fn asserted_style(config: &RDFVisualizationConfig) -> StereotypeStyle {
    StereotypeStyle::new("asserted")
        .with_background_color(config.asserted_background_color())
        .with_line_color(config.asserted_line_color())
        .with_line_thickness(config.asserted_line_thickness())
}

fn non_asserted_style(config: &RDFVisualizationConfig) -> StereotypeStyle {
    StereotypeStyle::new("non_asserted")
        .with_background_color(config.non_asserted_background_color())
        .with_line_color(config.non_asserted_line_color())
        .with_line_thickness(config.non_asserted_line_thickness())
        .with_round_corner(config.non_asserted_round_corner())
}

impl Default for Style {
    fn default() -> Self {
        Style::new()
    }
}
