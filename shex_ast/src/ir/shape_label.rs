use crate::{BNode, SchemaJsonError, ShapeExprLabel};
use iri_s::{IriS, IriSError};
use prefixmap::PrefixMap;
use prefixmap::error::PrefixMapError;
use serde::Serialize;
use std::{fmt::Display, str::FromStr};
use thiserror::Error;

/// Shape labels can be IRIs, Blank nodes or the special `Start` label
#[derive(PartialEq, Eq, Hash, Debug, Clone, PartialOrd, Ord)]
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

    pub fn from_object(obj: &srdf::Object) -> Result<ShapeLabel, SchemaJsonError> {
        match obj {
            srdf::Object::Iri(iri) => Ok(ShapeLabel::Iri(iri.clone())),
            srdf::Object::BlankNode(bnode_id) => Ok(ShapeLabel::BNode(BNode::new(bnode_id))),
            srdf::Object::Literal(_) => Err(SchemaJsonError::InvalidShapeLabel {
                value: obj.to_string(),
                error: "Literal cannot be a ShapeLabel".to_string(),
            }),
            srdf::Object::Triple { .. } => Err(SchemaJsonError::InvalidShapeLabel {
                value: obj.to_string(),
                error: "Triple cannot be a ShapeLabel".to_string(),
            }),
        }
    }

    pub fn from_iri_str(s: &str) -> Result<ShapeLabel, IriSError> {
        let iri = IriS::from_str(s)?;
        Ok(ShapeLabel::Iri(iri))
    }

    pub fn from_shape_expr_label(
        label: &ShapeExprLabel,
        prefixmap: &PrefixMap,
    ) -> Result<ShapeLabel, PrefixMapError> {
        match label {
            ShapeExprLabel::IriRef { value } => {
                Ok(ShapeLabel::Iri(value.get_iri_prefixmap(prefixmap)?.into_owned()))
            }
            ShapeExprLabel::BNode { value } => Ok(ShapeLabel::BNode(value.clone())),
            ShapeExprLabel::Start => Ok(ShapeLabel::Start),
        }
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
        } else {
            Ok(ShapeLabel::BNode(BNode::from(s)))
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
