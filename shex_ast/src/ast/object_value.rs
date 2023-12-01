use std::{result, str::FromStr};

use iri_s::{IriS, IriSError};
use prefixmap::{Deref, DerefError, IriRef};
use rust_decimal::Decimal;
use serde::{Serialize, Serializer};
use serde_derive::{Deserialize, Serialize};
use srdf::{lang::Lang, literal::Literal};

use super::serde_string_or_struct::SerializeStringOrStruct;
use crate::ast::serde_string_or_struct::*;

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum ObjectValue {
    Iri(IriRef),
    
    Literal(Literal),
}

impl ObjectValue {
    pub fn integer(n: isize) -> ObjectValue {
        ObjectValue::Literal(Literal::integer(n))
    }

    pub fn double(n: f64) -> ObjectValue {
        ObjectValue::Literal(Literal::double(n))
    }

    pub fn decimal(n: Decimal) -> ObjectValue {
        ObjectValue::Literal(Literal::decimal(n))
    }

    pub fn bool(b: bool) -> ObjectValue {
        ObjectValue::Literal(Literal::boolean(b))
    }

    pub fn datatype_literal(lexical_form: &str, datatype: &IriRef) -> ObjectValue {
        ObjectValue::Literal(Literal::datatype(lexical_form, datatype))
    }

    pub fn lexical_form(&self) -> String {
        match self {
            ObjectValue::Iri(iri) => iri.to_string(),
            ObjectValue::Literal(lit) => lit.lexical_form(),
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

/*fn serialize_boolean_literal<S>(value: &bool, serializer: S) -> result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        false => serializer.serialize_str("false"),
        true => serializer.serialize_str("true"),
    }
}*/
