use iri_s::{IriS, IriSError};
use std::{fmt::Display, str::FromStr};

use crate::BNode;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum ShapeLabel {
    Iri(IriS),
    BNode(BNode),
    Start,
}

impl ShapeLabel {
    pub fn iri(i: IriS) -> ShapeLabel {
        ShapeLabel::Iri(i)
    }
    pub fn from_bnode(bn: BNode) -> ShapeLabel {
        ShapeLabel::BNode(bn)
    }

    pub fn from_iri_str(s: &str) -> Result<ShapeLabel, IriSError> {
        let iri = IriS::from_str(s)?;
        Ok(ShapeLabel::Iri(iri))
    }
}

impl Display for ShapeLabel {
    fn fmt(&self, dest: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ShapeLabel::Iri(iri) => write!(dest, "{iri}"),
            ShapeLabel::BNode(bnode) => write!(dest, "{bnode}"),
            ShapeLabel::Start => write!(dest, "Start"),
        }
    }
}
