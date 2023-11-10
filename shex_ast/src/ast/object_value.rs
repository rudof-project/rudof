use std::{result, str::FromStr};

use iri_s::{IriS, IriSError};
use serde::{Serialize, Serializer};
use serde_derive::{Deserialize, Serialize};
use srdf::lang::Lang;

use super::{iri_ref::IriRef, serde_string_or_struct::SerializeStringOrStruct};
use crate::{ast::serde_string_or_struct::*, Deref, DerefError, NumericLiteral};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum ObjectValue {
    IriRef(IriRef),

    NumericLiteral(NumericLiteral),

    ObjectLiteral {
        value: String,

        #[serde(skip_serializing_if = "Option::is_none")]
        language: Option<Lang>,

        #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
        type_: Option<IriRef>,
    },
}

fn serialize_integer_literal<S>(v: &isize, serializer: S) -> result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    v.serialize(serializer)
}

impl ObjectValue {
    pub fn integer(n: isize) -> ObjectValue {
        let dt_integer = IriRef::Iri(IriS::xsd_integer());
        ObjectValue::ObjectLiteral {
            value: n.to_string(),
            language: None,
            type_: Some(dt_integer),
        }
    }

    pub fn bool(b: bool) -> ObjectValue {
        let dt_boolean = IriRef::Iri(IriS::xsd_boolean());
        ObjectValue::ObjectLiteral {
            value: b.to_string(),
            language: None,
            type_: Some(dt_boolean),
        }
    }
}

impl Deref for ObjectValue {
    fn deref(
        &self,
        base: &Option<iri_s::IriS>,
        prefixmap: &Option<prefixmap::PrefixMap>,
    ) -> Result<Self, DerefError> {
        match self {
            ObjectValue::IriRef(iri_ref) => {
                let new_iri_ref = iri_ref.deref(base, prefixmap)?;
                Ok(ObjectValue::IriRef(new_iri_ref))
            }
            ObjectValue::NumericLiteral(n) => Ok(ObjectValue::NumericLiteral(n.clone())),
            ObjectValue::ObjectLiteral {
                value,
                language,
                type_,
            } => {
                let new_type_ = type_
                    .as_ref()
                    .map(|dt| dt.deref(base, prefixmap))
                    .transpose()?;
                Ok(ObjectValue::ObjectLiteral {
                    value: value.clone(),
                    language: language.clone(),
                    type_: new_type_,
                })
            }
        }
    }
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

impl Deref for ObjectValueWrapper {
    fn deref(
        &self,
        base: &Option<iri_s::IriS>,
        prefixmap: &Option<prefixmap::PrefixMap>,
    ) -> Result<Self, DerefError>
    where
        Self: Sized,
    {
        let ov = self.ov.deref(base, prefixmap)?;
        Ok(ObjectValueWrapper { ov })
    }
}
