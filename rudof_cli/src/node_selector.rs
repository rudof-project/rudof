use anyhow::Result;
use rudof_lib::ShapeMapParser;
use shex_ast::shapemap::{NodeSelector, ShapeSelector};

pub fn parse_node_selector(node_str: &str) -> Result<NodeSelector> {
    let ns = ShapeMapParser::parse_node_selector(node_str)?;
    Ok(ns)
}

pub fn start() -> ShapeSelector {
    ShapeSelector::start()
}

pub fn parse_shape_selector(label_str: &str) -> Result<ShapeSelector> {
    let selector = ShapeMapParser::parse_shape_selector(label_str)?;
    Ok(selector)
}
