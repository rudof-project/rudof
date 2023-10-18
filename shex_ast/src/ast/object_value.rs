use std::{result, str::FromStr};

use iri_s::IriSError;
use serde::{Serialize, Serializer};
use serde_derive::{Deserialize, Serialize};
use void::Void;

use super::{iri_ref::IriRef, serde_string_or_struct::SerializeStringOrStruct};
use crate::ast::serde_string_or_struct::*;

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum ObjectValue {
    IriRef(IriRef),

    ObjectLiteral {
        value: String,

        #[serde(skip_serializing_if = "Option::is_none")]
        language: Option<String>,

        #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
        type_: Option<String>,
    },
}

impl FromStr for ObjectValue {
    type Err = IriSError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let iri_ref = IriRef::try_from(s)?;
        Ok(ObjectValue::IriRef(iri_ref))
    }
}

impl SerializeStringOrStruct for ObjectValue {
    fn serialize_string_or_struct<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self {
            ObjectValue::IriRef(ref r) => r.serialize(serializer),
            _ => self.serialize(serializer),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(transparent)]
pub struct ObjectValueWrapper {
    #[serde(
        serialize_with = "serialize_string_or_struct",
        deserialize_with = "deserialize_string_or_struct"
    )]
    pub ov: ObjectValue,
}
