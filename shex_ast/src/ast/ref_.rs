use std::str::FromStr;

use serde_derive::{Deserialize, Serialize};

use super::FromStrRefError;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub enum Ref {
    IriRef { value: String },
    BNode { value: String },
}

impl FromStr for Ref {
    type Err = FromStrRefError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Ref::IriRef {
            value: s.to_string(),
        })
    }
}
