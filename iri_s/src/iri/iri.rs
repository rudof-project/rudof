use crate::error::IriSError;
use crate::iri::iris::IriS;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// An IRI that can be either a raw [`String`] or a parsed [`IriS`]
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(try_from = "String", into = "String")]
pub enum Iri {
    String(String),
    IriS(IriS)
}

impl Iri {
    pub fn new(str: &str) -> Iri {
        Iri::String(str.to_string())
    }

    /// Converts a [`Iri`] represented as a [`String`] into a parsed [`Iri`] represented by a [`IriS`]
    /// `base` is useful to obtain an absolute Iri
    pub fn resolve(&self, base: Option<IriS>) -> Result<Iri, IriSError> {
        match self {
            Iri::String(s) => match base {
                None => {
                    let iri = IriS::from_str(s.as_str())?;
                    Ok(Iri::IriS(iri))
                }
                Some(base) => {
                    let iri = base.extend(s)?;
                    Ok(Iri::IriS(iri))
                }
            },
            Iri::IriS(_) => Ok(self.clone()),
        }
    }
}

impl Display for Iri {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Iri::String(s) => s,
            Iri::IriS(iri_s) => iri_s.as_str(),
        };
        write!(f, "{}", str)
    }
}

// This is required by serde serialization
impl From<Iri> for String {
    fn from(val: Iri) -> Self {
        match val {
            Iri::String(s) => s,
            Iri::IriS(iri_s) => iri_s.as_str().to_string(),
        }
    }
}

impl From<String> for Iri {
    fn from(value: String) -> Self {
        Iri::String(value)
    }
}
