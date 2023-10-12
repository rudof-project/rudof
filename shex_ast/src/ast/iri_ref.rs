use std::fmt::Display;

use serde_derive::{Deserialize, Serialize};
use void::Void;

#[derive(Deserialize, Serialize, Debug, PartialEq, Hash, Eq, Clone)]
#[serde(try_from = "&str", into = "String")]
pub struct IriRef {
    pub value: String,
}

impl TryFrom<&str> for IriRef {
    type Error = Void;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(IriRef {
            value: s.to_string(),
        })
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
