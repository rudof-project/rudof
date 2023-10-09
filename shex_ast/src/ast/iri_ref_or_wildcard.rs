use std::{result, str::FromStr};

use serde::{Serialize, Serializer};
use serde_derive::{Deserialize, Serialize};
use void::Void;

use super::{iri_ref::IriRef, serde_string_or_struct::SerializeStringOrStruct};

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum IriRefOrWildcard {
    IriRef(IriRef),
    Wildcard {
        #[serde(rename = "type")]
        type_: String,
    },
}

impl FromStr for IriRefOrWildcard {
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(IriRefOrWildcard::IriRef(IriRef {
            value: s.to_string(),
        }))
    }
}

impl SerializeStringOrStruct for IriRefOrWildcard {
    fn serialize_string_or_struct<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self {
            IriRefOrWildcard::IriRef(ref r) => r.serialize(serializer),
            _ => self.serialize(serializer),
        }
    }
}
