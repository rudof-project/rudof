use iri_s::{IriS, IriSError};
use serde_derive::{Deserialize, Serialize};
use std::str::FromStr;
use void::Void;

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(try_from = "String", into = "String")]
pub enum Iri {
    String(String),
    IriS(IriS),
}

impl Iri {
    pub fn new(str: &str) -> Iri {
        Iri::String(str.to_string())
    }

    /// Converts a `Iri`` represented as a `String` into an parsed Iri represented by a `IriS`
    /// `base` is useful to obtain an absolute Iri
    pub fn resolve(&mut self, base: Option<IriS>) -> Result<Iri, IriSError> {
        match self {
            Iri::String(s) => match base {
                None => {
                    let iri = IriS::from_str(s.as_str())?;
                    Ok(Iri::IriS(iri))
                }
                Some(base) => {
                    let iri = base.clone().extend(s)?;
                    Ok(Iri::IriS(iri))
                }
            },
            Iri::IriS(_) => Ok(self.clone()),
        }
    }
}

impl Into<String> for Iri {
    fn into(self) -> String {
        match self {
            Iri::String(s) => s,
            Iri::IriS(iri_s) => iri_s.as_str().to_string(),
        }
    }
}

impl TryFrom<String> for Iri {
    type Error = Void;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Ok(Iri::String(s))
    }
}
