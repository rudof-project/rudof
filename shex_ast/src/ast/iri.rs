use serde_derive::{Deserialize, Serialize};
use void::Void;

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(try_from = "String", into = "String")]
pub struct Iri {
    value: String,
}

impl Iri {
    pub fn new(str: &str) -> Iri {
        Iri {
            value: str.to_string(),
        }
    }
}

impl Into<String> for Iri {
    fn into(self) -> String {
        self.value
    }
}

impl TryFrom<String> for Iri {
    type Error = Void;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Ok(Iri { value: s })
    }
}
