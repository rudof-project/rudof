use iri_s::{IriS, IriSError};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;
use void::Void;

/// IriOrStr represents either an IRI or a String.
/// This enum is used mainly for parsing ShEx schemas which contain an import declaration
/// The value of the import declaration can be a well formed IRI or a relative IRI which is kept as a string
#[derive(Deserialize, Serialize, Debug, PartialEq, Hash, Eq, Clone)]
#[serde(try_from = "String", into = "String")]
pub enum IriOrStr {
    String(String),
    IriS(IriS),
}

impl IriOrStr {
    pub fn new(str: &str) -> IriOrStr {
        IriOrStr::String(str.to_string())
    }

    pub fn iri(iri: IriS) -> IriOrStr {
        IriOrStr::IriS(iri)
    }

    /// Converts a `Iri`` represented as a `String` into an parsed Iri represented by a `IriS`
    /// `base` is useful to obtain an absolute Iri
    pub fn resolve(&mut self, base: Option<IriS>) -> Result<IriOrStr, IriSError> {
        match self {
            IriOrStr::String(s) => match base {
                None => {
                    let iri = IriS::from_str(s.as_str())?;
                    Ok(IriOrStr::IriS(iri))
                }
                Some(base) => {
                    let iri = base.clone().extend(s)?;
                    Ok(IriOrStr::IriS(iri))
                }
            },
            IriOrStr::IriS(_) => Ok(self.clone()),
        }
    }
}

impl Display for IriOrStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            IriOrStr::String(s) => s,
            IriOrStr::IriS(iri_s) => iri_s.as_str(),
        };
        write!(f, "{}", str)
    }
}

// This is required by serde serialization
impl From<IriOrStr> for String {
    fn from(val: IriOrStr) -> Self {
        match val {
            IriOrStr::String(s) => s,
            IriOrStr::IriS(iri_s) => iri_s.as_str().to_string(),
        }
    }
}

impl TryFrom<String> for IriOrStr {
    type Error = Void;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Ok(IriOrStr::String(s))
    }
}
