use std::str::FromStr;
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

    pub fn from_iri_str(s: String) -> Result<ShapeLabel, IriError> {
        let iri = IriS::from_str(s.as_str())?;
        Ok(ShapeLabel::Iri(iri))
    }

}

