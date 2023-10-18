use std::{fmt::Display, str::FromStr};

use iri_s::{IriS, IriSError};
use serde_derive::{Deserialize, Serialize};
use void::Void;

#[derive(Deserialize, Serialize, Debug, PartialEq, Hash, Eq, Clone)]
#[serde(try_from = "&str", into = "String")]
pub struct IriRef {
    pub value: IriS,
}

impl TryFrom<&str> for IriRef {
    type Error = IriSError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let iri_s = IriS::from_str(s)?;
        Ok(IriRef { value: iri_s})
    }
}

impl Into<IriS> for IriRef {
    fn into(self) -> IriS {
        self.value
    }
}

impl From<IriS> for IriRef {
    fn from(i: IriS) -> IriRef {
        IriRef {
            value: i,
        }
    }
}

impl Into<String> for IriRef {
    fn into(self) -> String {
        self.value.to_string()
    }
}

impl Display for IriRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}>", self.value)?;
        Ok(())
    }
}
