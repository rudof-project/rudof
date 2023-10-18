use std::fmt::Display;

use iri_s::IriS;
use serde_derive::{Deserialize, Serialize};
use void::Void;

#[derive(Deserialize, Serialize, Debug, PartialEq, Hash, Eq, Clone)]
#[serde(try_from = "&str", into = "String")]
pub struct IriRef {
    pub value: String,
}

impl From<&str> for IriRef {
    fn from(s: &str) -> Self {
        IriRef {
            value: s.to_string(),
        }
    }
}

impl From<IriS> for IriRef {
    fn from(i: IriS) -> IriRef {
        IriRef {
            value: i.to_string(),
        }
    }
}

impl Into<String> for IriRef {
    fn into(self) -> String {
        self.value
    }
}

impl Display for IriRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)?;
        Ok(())
    }
}
