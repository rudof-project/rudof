use std::str::FromStr;

use serde_derive::{Deserialize, Serialize};
use void::Void;

use super::FromStrRefError;

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(try_from = "String", into = "String")]
pub enum Ref {
    IriRef { value: String },
    BNode { value: String },
}

impl Into<String> for Ref {
    fn into(self) -> String {
        match self {
            Ref::IriRef { value } => value,
            Ref::BNode { value } => value,
        }
    }
}


impl TryFrom<String> for Ref {
    type Error = Void;

    // TODO: We should parse the bnode
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Ok(Ref::IriRef { value: s })
    }
}

impl FromStr for Ref {
    type Err = FromStrRefError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Ref::IriRef {
            value: s.to_string(),
        })
    }
}
