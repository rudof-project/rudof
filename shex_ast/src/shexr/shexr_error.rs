use iri_s::IriS;
use srdf::{literal::Literal, RDFParseError};
use std::fmt::Display;
use thiserror::Error;

use crate::Node;

#[derive(Debug, Error)]
pub enum ShExRError {
    #[error("RDF parse error: {err}")]
    RDFParseError {
        #[from]
        err: RDFParseError,
    },

    #[error("No nodes with `rdf:type sx:Schema`")]
    NoSchemaNodes,

    #[error("More than one nodes with `rdf:type sx:Schema`")]
    MoreThanOneSchema { nodes: Nodes },

    #[error("Shape Label can not be a literal {lit}")]
    ShapeExprLabelLiteral { lit: Literal },

    #[error("Unexpected value for nodeKind: {iri}")]
    UnexpectedNodeKind { iri: IriS },
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
