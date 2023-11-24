use std::{result, str::FromStr};

use iri_s::{IriS, IriSError};
use rust_decimal::Decimal;
use serde::{Serialize, Serializer};
use serde_derive::{Deserialize, Serialize};
use srdf::lang::Lang;

use super::{iri_ref::IriRef, serde_string_or_struct::SerializeStringOrStruct};
use crate::{ast::serde_string_or_struct::*, Deref, DerefError, NumericLiteral};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum ObjectValue {
    Iri(IriRef),
    Literal(Literal),
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub enum Literal {
    NumericLiteral(NumericLiteral),

    #[serde(serialize_with = "serialize_boolean_literal")]
    BooleanLiteral {
        value: bool,
    },

    ObjectLiteral {
        value: String,

        #[serde(skip_serializing_if = "Option::is_none")]
        language: Option<Lang>,

        #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
        type_: Option<IriRef>,
    },
}

/*fn serialize_integer_literal<S>(v: &isize, serializer: S) -> result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    v.serialize(serializer)
}*/

impl ObjectValue {
    pub fn integer(n: isize) -> ObjectValue {
        let dt_integer = IriRef::Iri(IriS::xsd_integer());
        ObjectValue::Literal(Literal::ObjectLiteral {
            value: n.to_string(),
            language: None,
            type_: Some(dt_integer),
        })
    }

    pub fn double(n: f64) -> ObjectValue {
        ObjectValue::Literal(Literal::NumericLiteral(NumericLiteral::Double(n)))
    }

    pub fn decimal(n: Decimal) -> ObjectValue {
        ObjectValue::Literal(Literal::NumericLiteral(NumericLiteral::decimal(n)))
    }

    pub fn bool(b: bool) -> ObjectValue {
        ObjectValue::Literal(Literal::BooleanLiteral { value: b })
    }

    pub fn lexical_form(&self) -> String {
        match self {
            ObjectValue::Iri(iri) => iri.to_string(),
            ObjectValue::Literal(lit) => lit.lexical_form(),
        }
    }
}

impl Literal {
    pub fn lexical_form(&self) -> String {
        match self {
            Literal::BooleanLiteral { value: true } => "true".to_string(),
            Literal::BooleanLiteral { value: false } => "false".to_string(),
            Literal::NumericLiteral(n) => n.to_string(),
            Literal::ObjectLiteral { value, .. } => value.to_string(),
        }
    }

    pub fn bool(b: bool) -> Literal {
        Literal::BooleanLiteral { value: b }
    }
}

impl Deref for ObjectValue {
    fn deref(
        &self,
        base: &Option<iri_s::IriS>,
        prefixmap: &Option<prefixmap::PrefixMap>,
    ) -> Result<Self, DerefError> {
        match self {
            ObjectValue::Iri(iri_ref) => {
                let new_iri_ref = iri_ref.deref(base, prefixmap)?;
                Ok(ObjectValue::Iri(new_iri_ref))
            }
            ObjectValue::Literal(lit) => {
                let new_lit = lit.deref(base, prefixmap)?;
                Ok(ObjectValue::Literal(new_lit))
            }
        }
    }
}

impl Deref for Literal {
    fn deref(
        &self,
        base: &Option<iri_s::IriS>,
        prefixmap: &Option<prefixmap::PrefixMap>,
    ) -> Result<Self, DerefError> {
        match self {
            Literal::NumericLiteral(n) => Ok(Literal::NumericLiteral(n.clone())),
            Literal::BooleanLiteral { value } => Ok(Literal::BooleanLiteral {
                value: value.clone(),
            }),
            Literal::ObjectLiteral {
                value,
                language,
                type_,
            } => {
                let new_type_ = type_
                    .as_ref()
                    .map(|dt| dt.deref(base, prefixmap))
                    .transpose()?;
                Ok(Literal::ObjectLiteral {
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
        Ok(ObjectValue::Iri(iri_ref))
    }
}

impl SerializeStringOrStruct for ObjectValue {
    fn serialize_string_or_struct<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self {
            ObjectValue::Iri(ref r) => r.serialize(serializer),
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

fn serialize_boolean_literal<S>(value: &bool, serializer: S) -> result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        false => serializer.serialize_str("false"),
        true => serializer.serialize_str("true"),
    }
}
