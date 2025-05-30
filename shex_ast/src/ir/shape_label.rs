use iri_s::{IriS, IriSError};
use serde::Serialize;
use std::{fmt::Display, str::FromStr};
use thiserror::Error;

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

impl TryFrom<&str> for ShapeLabel {
    type Error = ShapeLabelError;

    #[allow(irrefutable_let_patterns)]
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s == "Start" {
            Ok(ShapeLabel::Start)
        } else if let Ok(iri) = IriS::from_str(s) {
            Ok(ShapeLabel::Iri(iri))
        } else if let Ok(bnode) = BNode::try_from(s) {
            Ok(ShapeLabel::BNode(bnode))
        } else {
            Err(ShapeLabelError::InvalidStr(s.to_string()))
        }
    }
}

#[derive(Error, Debug, Clone)]

pub enum ShapeLabelError {
    #[error("Invalid ShapeLabel string: {0}")]
    InvalidStr(String),
}

impl Serialize for ShapeLabel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            ShapeLabel::Iri(iri) => serializer.serialize_str(&iri.to_string()),
            ShapeLabel::BNode(bnode) => serializer.serialize_str(&bnode.to_string()),
            ShapeLabel::Start => serializer.serialize_str("Start"),
        }
    }
}
