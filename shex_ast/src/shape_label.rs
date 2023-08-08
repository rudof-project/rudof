use std::{str::FromStr, fmt::Display};
use iri_s::{IriS, IriError};

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum ShapeLabel {
    Iri(IriS),
    BNode(String),
}

impl ShapeLabel {

    pub fn from_bnode_str(s: String) -> ShapeLabel {
        ShapeLabel::BNode(s)
    }

    pub fn from_iri_str(s: &str) -> Result<ShapeLabel, IriError> {
        let iri = IriS::from_str(s)?;
        Ok(ShapeLabel::Iri(iri))
    }

}

impl Display for ShapeLabel {
    fn fmt(&self, dest: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> { 
        match self {
            ShapeLabel::Iri(iri) => write!(dest,"{iri}"),
            ShapeLabel::BNode(bnode) => write!(dest,"{bnode}")
        }
    }
      
}

