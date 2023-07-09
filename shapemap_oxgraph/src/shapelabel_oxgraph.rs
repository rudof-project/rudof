use std::str::FromStr;

use oxrdf::{BlankNode, NamedNode};
use thiserror::Error;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum ShapeLabelOxGraph {
    Iri(NamedNode),
    BNode(BlankNode),
}

#[derive(Error, Debug)]
pub enum ShapeLabelError {
    #[error("Parsing ShapeLabel: {err:?}")]
    Str2ShapeLabelError { err: String },
}

impl FromStr for ShapeLabelOxGraph {
    type Err = ShapeLabelError;
    fn from_str(str: &str) -> Result<ShapeLabelOxGraph, Self::Err> {
        Ok(ShapeLabelOxGraph::Iri(NamedNode::new_unchecked(str)))
    }
}
