use iri_s::{IriS, IriSError};
use prefixmap::error::DerefError;
use prefixmap::{Deref, IriRef};
use rust_decimal::Decimal;
use serde::de::Unexpected;
use serde::ser::SerializeMap;
use serde::{
    Deserialize, Serialize, Serializer,
    de::{self, MapAccess, Visitor},
};
use srdf::SLiteral;
use srdf::lang::Lang;
use srdf::numeric_literal::NumericLiteral;
use std::fmt::{self, Display};
use std::{result, str::FromStr};

use crate::Node;
use crate::ast::{
    BYTE_STR, DATETIME_STR, FLOAT_STR, LONG_STR, NEGATIVE_INTEGER_STR, NON_NEGATIVE_INTEGER_STR,
    NON_POSITIVE_INTEGER_STR, POSITIVE_INTEGER_STR, SHORT_STR, UNSIGNED_BYTE_STR, UNSIGNED_INT_STR,
    UNSIGNED_LONG_STR, UNSIGNED_SHORT_STR,
};

use super::{BOOLEAN_STR, DECIMAL_STR, DOUBLE_STR, INTEGER_STR};

/// An object value can be either an IRI reference or a literal
/// It is used in various places in ShEx, for example in ValueSetValue, or as focus node in ShapeMap
/// It is similar to srdf::Object but does not include blank nodes
#[derive(Debug, PartialEq, Clone)]
pub enum ObjectValue {
    IriRef(IriRef),
    Literal(SLiteral),
    // TODO: consider adding Triples
}

impl ObjectValue {
    pub fn integer(n: isize) -> ObjectValue {
        ObjectValue::Literal(SLiteral::integer(n))
    }

    pub fn double(n: f64) -> ObjectValue {
        ObjectValue::Literal(SLiteral::double(n))
    }

    pub fn decimal(n: Decimal) -> ObjectValue {
        ObjectValue::Literal(SLiteral::decimal(n))
    }

    pub fn bool(b: bool) -> ObjectValue {
        ObjectValue::Literal(SLiteral::boolean(b))
    }

    pub fn literal(lit: SLiteral) -> ObjectValue {
        ObjectValue::Literal(lit)
    }

    pub fn datatype_literal(lexical_form: &str, datatype: &IriRef) -> ObjectValue {
        ObjectValue::Literal(SLiteral::lit_datatype(lexical_form, datatype))
    }

    pub fn lexical_form(&self) -> String {
        match self {
            ObjectValue::IriRef(iri) => iri.to_string(),
            ObjectValue::Literal(lit) => lit.lexical_form(),
        }
    }

    pub fn iri(iri: IriS) -> Self {
        ObjectValue::IriRef(IriRef::iri(iri))
    }

    pub fn iri_ref(iri: IriRef) -> Self {
        ObjectValue::IriRef(iri)
    }

    pub fn prefixed(alias: &str, local: &str) -> Self {
        ObjectValue::IriRef(IriRef::prefixed(alias, local))
    }

    pub fn str(str: &str) -> Self {
        ObjectValue::Literal(SLiteral::str(str))
    }
}

impl Deref for ObjectValue {
    fn deref(
        self,
        base: Option<&IriS>,
        prefixmap: Option<&prefixmap::PrefixMap>,
    ) -> Result<Self, DerefError> {
        match self {
            ObjectValue::IriRef(iri_ref) => {
                let new_iri_ref = iri_ref.deref(base, prefixmap)?;
                Ok(ObjectValue::IriRef(new_iri_ref))
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
        Ok(ObjectValue::IriRef(iri_ref))
    }
}

impl Serialize for ObjectValue {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ObjectValue::Literal(SLiteral::BooleanLiteral(value)) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", BOOLEAN_STR)?;
                let value_str = if *value { "true" } else { "false" };
                map.serialize_entry("value", value_str)?;
                map.end()
            }
            ObjectValue::Literal(SLiteral::NumericLiteral(num)) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", get_type_str(num))?;
                map.serialize_entry("value", &num.to_string())?;
                map.end()
            }
            ObjectValue::Literal(SLiteral::DatetimeLiteral(date_time)) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", DATETIME_STR)?;
                map.serialize_entry("value", &date_time.to_string())?;
                map.end()
            }

            ObjectValue::IriRef(iri) => serializer.serialize_str(iri.to_string().as_str()),
            ObjectValue::Literal(SLiteral::StringLiteral { lexical_form, lang }) => {
                let mut map = serializer.serialize_map(Some(3))?;
                if let Some(lan) = lang {
                    map.serialize_entry("language", &Some(lan))?;
                }
                map.serialize_entry("value", lexical_form)?;
                map.end()
            }
            ObjectValue::Literal(SLiteral::DatatypeLiteral {
                lexical_form,
                datatype,
            }) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", datatype)?;
                map.serialize_entry("value", lexical_form)?;
                map.end()
            }
            ObjectValue::Literal(SLiteral::WrongDatatypeLiteral {
                lexical_form,
                datatype,
                error,
            }) => {
                // TODO: Maybe raise some warning instead of using the error field?
                let mut map = serializer.serialize_map(Some(3))?;
                map.serialize_entry("type", datatype)?;
                map.serialize_entry("value", lexical_form)?;
                map.serialize_entry("error", error)?;
                map.end()
            }
        }
    }
}

fn get_type_str(n: &NumericLiteral) -> &str {
    match n {
        NumericLiteral::Integer(_) => INTEGER_STR,
        NumericLiteral::Double(_) => DOUBLE_STR,
        NumericLiteral::Decimal(_) => DECIMAL_STR,
        NumericLiteral::Long(_) => LONG_STR,
        NumericLiteral::Float(_) => FLOAT_STR,
        NumericLiteral::Byte(_) => BYTE_STR,
        NumericLiteral::Short(_) => SHORT_STR,
        NumericLiteral::NonNegativeInteger(_) => NON_NEGATIVE_INTEGER_STR,
        NumericLiteral::UnsignedLong(_) => UNSIGNED_LONG_STR,
        NumericLiteral::UnsignedInt(_) => UNSIGNED_INT_STR,
        NumericLiteral::UnsignedShort(_) => UNSIGNED_SHORT_STR,
        NumericLiteral::UnsignedByte(_) => UNSIGNED_BYTE_STR,
        NumericLiteral::PositiveInteger(_) => POSITIVE_INTEGER_STR,
        NumericLiteral::NegativeInteger(_) => NEGATIVE_INTEGER_STR,
        NumericLiteral::NonPositiveInteger(_) => NON_POSITIVE_INTEGER_STR,
    }
}

#[derive(Debug, PartialEq)]
enum ObjectValueType {
    Boolean,
    Integer,
    Decimal,
    Double,
    Other(IriRef),
}

impl ObjectValueType {
    fn parse(s: &str) -> Result<ObjectValueType, IriSError> {
        match s {
            BOOLEAN_STR => Ok(ObjectValueType::Boolean),
            DECIMAL_STR => Ok(ObjectValueType::Decimal),
            DOUBLE_STR => Ok(ObjectValueType::Double),
            INTEGER_STR => Ok(ObjectValueType::Integer),
            other => {
                let iri = FromStr::from_str(other)?;
                Ok(ObjectValueType::Other(iri))
            }
        }
    }
}

impl<'de> Deserialize<'de> for ObjectValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        enum Field {
            Type,
            Value,
            Language,
            LanguageTag,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct FieldVisitor;

                impl Visitor<'_> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`value` for object value")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "type" => Ok(Field::Type),
                            "value" => Ok(Field::Value),
                            "language" => Ok(Field::Language),
                            "languageTag" => Ok(Field::LanguageTag),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct ObjectValueVisitor;

        impl<'de> Visitor<'de> for ObjectValueVisitor {
            type Value = ObjectValue;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("object value")
            }

            fn visit_map<V>(self, mut map: V) -> Result<ObjectValue, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut value: Option<String> = None;
                let mut type_: Option<ObjectValueType> = None;
                let mut language: Option<String> = None;
                let mut language_tag: Option<String> = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Type => {
                            if type_.is_some() {
                                return Err(de::Error::duplicate_field("type"));
                            }
                            let value: String = map.next_value()?;

                            let parsed_type_ =
                                ObjectValueType::parse(value.as_str()).map_err(|e| {
                                    de::Error::custom(format!(
                                    "Error parsing ValueSetValue type, found: {value}. Error: {e}"
                                ))
                                })?;
                            type_ = Some(parsed_type_);
                        }
                        Field::Value => {
                            if value.is_some() {
                                return Err(de::Error::duplicate_field("value"));
                            }
                            value = Some(map.next_value()?);
                        }
                        Field::Language => {
                            if language.is_some() {
                                return Err(de::Error::duplicate_field("language"));
                            }
                            language = Some(map.next_value()?);
                        }
                        Field::LanguageTag => {
                            if language_tag.is_some() {
                                return Err(de::Error::duplicate_field("languageTag"));
                            }
                            language_tag = Some(map.next_value()?);
                        }
                    }
                }
                match type_ {
                    Some(ObjectValueType::Boolean) => match value {
                        Some(s) => match s.as_str() {
                            "false" => Ok(ObjectValue::bool(false)),
                            "true" => Ok(ObjectValue::bool(true)),
                            _ => Err(de::Error::invalid_value(Unexpected::Str(&s), &self)),
                        },
                        None => Err(de::Error::missing_field("value")),
                    },
                    Some(ObjectValueType::Decimal) => match value {
                        Some(s) => {
                            let n = Decimal::from_str(&s).map_err(|e| {
                                de::Error::custom(format!(
                                    "Can't parse value {s} as decimal: Error {e}"
                                ))
                            })?;
                            Ok(ObjectValue::decimal(n))
                        }
                        None => Err(de::Error::missing_field("value")),
                    },
                    Some(ObjectValueType::Double) => match value {
                        Some(s) => {
                            let n = f64::from_str(&s).map_err(|e| {
                                de::Error::custom(format!(
                                    "Can't parse value {s} as double: Error {e}"
                                ))
                            })?;
                            Ok(ObjectValue::double(n))
                        }
                        None => Err(de::Error::missing_field("value")),
                    },
                    Some(ObjectValueType::Integer) => match value {
                        Some(s) => {
                            let n = isize::from_str(&s).map_err(|e| {
                                de::Error::custom(format!(
                                    "Can't parse value {s} as integer: Error {e}"
                                ))
                            })?;
                            Ok(ObjectValue::integer(n))
                        }
                        None => Err(de::Error::missing_field("value")),
                    },
                    Some(ObjectValueType::Other(iri)) => match value {
                        Some(v) => match language_tag {
                            Some(lang) => {
                                let lang = Lang::new(&lang).map_err(|e| {
                                    de::Error::custom(format!(
                                        "Invalid language tag {lang} in object value: {e}"
                                    ))
                                })?;
                                Ok(ObjectValue::Literal(SLiteral::StringLiteral {
                                    lexical_form: v,
                                    lang: Some(lang),
                                }))
                            }
                            None => Ok(ObjectValue::datatype_literal(&v, &iri)),
                        },
                        None => Err(de::Error::missing_field("value")),
                    },
                    None => match value {
                        Some(lexical_form) => match language {
                            Some(language) => {
                                let language = Lang::new(&language).map_err(|e| {
                                    de::Error::custom(format!(
                                        "Invalid language tag {language} in object value: {e}"
                                    ))
                                })?;
                                Ok(ObjectValue::Literal(SLiteral::StringLiteral {
                                    lexical_form,
                                    lang: Some(language),
                                }))
                            }
                            None => Ok(ObjectValue::Literal(SLiteral::StringLiteral {
                                lexical_form,
                                lang: None,
                            })),
                        },
                        None => Err(de::Error::missing_field("value")),
                    },
                }
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let iri_ref = IriRef::from_str(s).map_err(|e| {
                    de::Error::custom(format!("Cannot convert string `{s}` to Iri: {e}"))
                })?;
                Ok(ObjectValue::IriRef(iri_ref))
            }
        }

        const FIELDS: &[&str] = &["value", "type", "languageTag"];
        deserializer.deserialize_any(ObjectValueVisitor)
    }
}

impl From<&ObjectValue> for srdf::Object {
    fn from(value: &ObjectValue) -> Self {
        match value {
            ObjectValue::IriRef(iri_ref) => {
                let iri = iri_ref.get_iri().unwrap(); // Should not fail, as it was already deref'ed
                srdf::Object::from(iri.clone())
            }
            ObjectValue::Literal(literal) => literal.into(),
        }
    }
}

impl From<&ObjectValue> for Node {
    fn from(value: &ObjectValue) -> Self {
        let obj: srdf::Object = value.into();
        Node::from(obj)
    }
}

impl Display for ObjectValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ObjectValue::IriRef(iri_ref) => write!(f, "{iri_ref}"),
            ObjectValue::Literal(sliteral) => write!(f, "{sliteral}"),
        }
    }
}
