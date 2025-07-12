use rudof_lib::ShapeMapParser;
use shapemap::NodeSelector;

use crate::anyhow::Result;

pub fn parse_node_selector(node_str: &str) -> Result<NodeSelector> {
    let ns = ShapeMapParser::parse_node_selector(node_str)?;
    Ok(ns)
}
