use std::{result, str::FromStr, fmt};

use crate::{ast::serde_string_or_struct::*, Deref, DerefError, LangOrWildcard};
use iri_s::IriSError;
use serde::{
    de::{self, MapAccess, Visitor, Unexpected},
    Deserialize, Serialize, Serializer,
};
use serde_derive::{Serialize};
use srdf::lang::Lang;

use super::{
    iri_ref::IriRef, iri_ref_or_wildcard::IriRefOrWildcard,
    string_or_iri_stem::StringOrIriStemWrapper, string_or_literal_stem::StringOrLiteralStemWrapper,
    string_or_wildcard::StringOrWildcard, ObjectValue, ObjectValueWrapper,
};

#[derive(Serialize, Debug, PartialEq, Clone)]
#[serde(tag = "type")]
pub enum ValueSetValue {
    IriStem {
        stem: IriRef,
    },
    IriStemRange {
        #[serde(
            serialize_with = "serialize_string_or_struct",
            deserialize_with = "deserialize_string_or_struct"
        )]
        stem: IriRefOrWildcard,

        #[serde(skip_serializing_if = "Option::is_none")]
        exclusions: Option<Vec<StringOrIriStemWrapper>>,
    },
    LiteralStem {
        stem: String,
    },
    LiteralStemRange {
        #[serde(
            serialize_with = "serialize_string_or_struct",
            deserialize_with = "deserialize_string_or_struct"
        )]
        stem: StringOrWildcard,

        #[serde(skip_serializing_if = "Option::is_none")]
        exclusions: Option<Vec<StringOrLiteralStemWrapper>>,
    },
    Language {
        #[serde(rename = "languageTag", 
           serialize_with = "serialize_lang", 
           deserialize_with = "deserialize_lang")]
        language_tag: Lang,
    },
    LanguageStem {
        #[serde(rename = "stem", 
           serialize_with = "serialize_lang", 
           deserialize_with = "deserialize_lang")]
        stem: Lang,
    },
    LanguageStemRange {
        #[serde(
            serialize_with = "serialize_string_or_struct",
            deserialize_with = "deserialize_string_or_struct"
        )]
        stem: LangOrWildcard,

        #[serde(skip_serializing_if = "Option::is_none")]
        exclusions: Option<Vec<StringOrLiteralStemWrapper>>,
    },
    ObjectValue(ObjectValue),
}

impl ValueSetValue {
    pub fn iri(iri: IriRef) -> ValueSetValue {
        ValueSetValue::ObjectValue(ObjectValue::IriRef(iri))
    }

    pub fn literal(value: &str, language: Option<Lang>, type_: Option<IriRef>) -> ValueSetValue {
        let ov = ObjectValue::ObjectLiteral {
            value: value.to_string(),
            language,
            type_,
        };
        ValueSetValue::ObjectValue(ov)
    }

    pub fn object_value(value: ObjectValue) -> ValueSetValue {
        ValueSetValue::ObjectValue(ov)
    }

    pub fn language(lang: Lang) -> ValueSetValue {
        ValueSetValue::Language { 
           language_tag: lang
        }
    }

    pub fn language_stem(lang: Lang) -> ValueSetValue {
        ValueSetValue::LanguageStem { 
           stem: lang
        }
    }

}

impl Deref for ValueSetValue {
    fn deref(
        &self,
        base: &Option<iri_s::IriS>,
        prefixmap: &Option<prefixmap::PrefixMap>,
    ) -> Result<Self, DerefError>
    where
        Self: Sized,
    {
        match self {
            ValueSetValue::ObjectValue(ov) => {
                let ov = ov.deref(base, prefixmap)?;
                Ok(ValueSetValue::ObjectValue(ov))
            },
            ValueSetValue::Language { language_tag } => {
                Ok(ValueSetValue::Language { language_tag: language_tag.clone() })
            }
            ValueSetValue::LanguageStem { stem } => {
                Ok(ValueSetValue::LanguageStem { stem: stem.clone() })
            }
            _ => {
                todo!()
            }
        }
    }
}

/*#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(transparent)]
pub struct ValueSetValueWrapper {
    #[serde(
        serialize_with = "serialize_string_or_struct",
        deserialize_with = "deserialize_string_or_struct"
    )]
    pub vs: ValueSetValue,
}

impl ValueSetValueWrapper {
    pub fn new(vs: ValueSetValue) -> ValueSetValueWrapper {
        ValueSetValueWrapper { vs: vs }
    }

    pub fn value(&self) -> ValueSetValue {
        self.vs.clone()
    }
}

impl Deref for ValueSetValueWrapper {
    fn deref(
        &self,
        base: &Option<iri_s::IriS>,
        prefixmap: &Option<prefixmap::PrefixMap>,
    ) -> Result<Self, DerefError>
    where
        Self: Sized,
    {
        let vs = self.vs.deref(base, prefixmap)?;
        Ok(ValueSetValueWrapper { vs })
    }
}*/

impl SerializeStringOrStruct for ValueSetValue {
    fn serialize_string_or_struct<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self {
            ValueSetValue::ObjectValue(ObjectValue::IriRef(ref r)) => r.serialize(serializer),
            _ => self.serialize(serializer),
        }
    }
}

impl FromStr for ValueSetValue {
    type Err = IriSError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let iri_ref = IriRef::try_from(s)?;
        Ok(ValueSetValue::ObjectValue(ObjectValue::IriRef(iri_ref)))
    }
}

fn serialize_lang<S>(lang: &Lang, serializer: S) -> result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
   serializer.serialize_str(lang.value().as_str())
}

fn deserialize_lang<'de, D>(deserializer: D) -> Result<Lang, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct LangVisitor;

        impl<'de> Visitor<'de> for LangVisitor {
            type Value = Lang;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Lang")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Lang::new(v))
            }
  

        }
        deserializer.deserialize_str(LangVisitor)
    }


impl<'de> Deserialize<'de> for ValueSetValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            enum Field {
                Type,
                Value,
                Language,
                Stem,
                Exclusions
            }
    
            impl<'de> Deserialize<'de> for Field {
                fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    struct FieldVisitor;
    
                    impl<'de> Visitor<'de> for FieldVisitor {
                        type Value = Field;
    
                        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                            formatter.write_str(
                                "field of value set value: `type` or `value` or `language` or `stem` or `exclusions`",
                            )
                        }
    
                        fn visit_str<E>(self, value: &str) -> Result<Field, E>
                        where
                            E: de::Error,
                        {
                            match value {
                                "type" => Ok(Field::Type),
                                "value" => Ok(Field::Value),
                                "stem" => Ok(Field::Stem),
                                "language" => Ok(Field::Language),
                                "exclusions" => Ok(Field::Exclusions),
                                _ => Err(de::Error::unknown_field(value, FIELDS)),
                            }
                        }
                    }
    
                    deserializer.deserialize_identifier(FieldVisitor)
                }
            }
    
            struct ValueSetValueVisitor;
    
            impl<'de> Visitor<'de> for ValueSetValueVisitor {
                type Value = ValueSetValue;
    
                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("ValueSet value")
                }

                fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
                  where E: de::Error {
                     FromStr::from_str(s).map_err(|e| {
                        de::Error::invalid_value(Unexpected::Str(s), &self)
                     })
                  }

                fn visit_map<V>(self, mut map: V) -> Result<ValueSetValue, V::Error>
                where
                    V: MapAccess<'de>,
                {
                    let mut type_: Option<String> = None;
                    while let Some(key) = map.next_key()? {
                        match key {
                            Field::Type => {
                                if type_.is_some() {
                                    return Err(de::Error::duplicate_field("type"));
                                }
                                let value: String = map.next_value()?;
                                if value != "NodeConstraint" {
                                    return Err(de::Error::custom(format!(
                                        "Expected NodeConstraint, found: {value}"
                                    )));
                                }
                                type_ = Some("NodeConstraint".to_string());
                            }
                        }
                    }
                    Ok(nc)
                }
            }
    
            deserializer.deserialize_any(ValueSetValueVisitor)
        }
    }
    