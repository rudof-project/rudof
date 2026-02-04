use iri_s::error::IriSError;
use iri_s::IriS;
use prefixmap::IriRef;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;

/// IriOrStr represents either an IRI or a String.
/// This enum is used mainly for parsing ShEx schemas which contain an import declaration
/// The value of the import declaration can be a well formed IRI or a relative IRI which is kept as a string
#[derive(Deserialize, Serialize, Debug, PartialEq, Hash, Eq, Clone)]
#[serde(try_from = "String", into = "String")]
pub enum IriOrStr {
    String(String),
    IriRef(IriRef),
}

impl IriOrStr {
    pub fn new(str: &str) -> IriOrStr {
        IriOrStr::String(str.to_string())
    }

    pub fn iri(iri: IriS) -> IriOrStr {
        IriOrStr::IriRef(IriRef::iri(iri))
    }

    /// Converts a `Iri`` represented as a `String` into an parsed Iri represented by a `IriS`
    /// `base` is useful to obtain an absolute Iri
    pub fn resolve(&self, base: &Option<IriS>) -> Result<IriS, IriSError> {
        match self {
            IriOrStr::String(s) => match base {
                None => {
                    let iri = IriS::from_str(s.as_str())?;
                    Ok(iri)
                }
                Some(base) => {
                    let iri = base.resolve_str(s)?;
                    Ok(iri)
                }
            },
            IriOrStr::IriRef(iri_ref) => match iri_ref {
                IriRef::Iri(iri_s) => Ok(iri_s.clone()),
                IriRef::Prefixed {
                    prefix: _,
                    local: _,
                } => todo!(),
            },
        }
    }
}

impl Display for IriOrStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            IriOrStr::String(s) => s.clone(),
            IriOrStr::IriRef(iri_ref) => iri_ref.to_string(),
        };
        write!(f, "{str}")
    }
}

// This is required by serde serialization
impl From<IriOrStr> for String {
    fn from(val: IriOrStr) -> Self {
        match val {
            IriOrStr::String(s) => s,
            IriOrStr::IriRef(iri) => iri.to_string(),
        }
    }
}

impl From<String> for IriOrStr {
    fn from(s: String) -> Self {
        IriOrStr::String(s)
    }
}
