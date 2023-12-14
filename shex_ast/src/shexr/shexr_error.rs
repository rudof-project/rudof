use std::fmt::Display;
use thiserror::Error;

use crate::Node;

#[derive(Debug, Error)]
pub enum ShExRError {
    #[error("RDF error: {err}")]
    SRDFError { err: String },

    #[error("No nodes with `rdf:type sx:Schema`")]
    NoSchemaNodes,

    #[error("More than one nodes with `rdf:type sx:Schema`")]
    MoreThanOneSchema { nodes: Nodes },
}

#[derive(Debug)]

pub struct Nodes {
    values: Vec<Node>,
}

impl Nodes {
    pub fn new(ns: Vec<Node>) -> Nodes {
        Nodes { values: ns }
    }
}

impl Display for Nodes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for n in self.values.iter() {
            write!(f, "{n} ")?;
        }
        writeln!(f, "")?;
        Ok(())
    }
}
