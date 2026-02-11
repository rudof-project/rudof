use shex_ast::{
    ShapeMapParser,
    shapemap::{NodeSelector, ShapeSelector},
};

use crate::RudofError;

pub fn parse_node_selector(node_str: &str) -> Result<NodeSelector, RudofError> {
    let ns = ShapeMapParser::parse_node_selector(node_str).map_err(|e| RudofError::NodeSelectorParseError {
        node_selector: node_str.to_string(),
        error: e.to_string(),
    })?;
    Ok(ns)
}

pub fn start() -> ShapeSelector {
    ShapeSelector::start()
}

pub fn parse_shape_selector(label_str: &str) -> Result<ShapeSelector, RudofError> {
    let selector =
        ShapeMapParser::parse_shape_selector(label_str).map_err(|e| RudofError::ShapeSelectorParseError {
            shape_selector: label_str.to_string(),
            error: e.to_string(),
        })?;
    Ok(selector)
}
