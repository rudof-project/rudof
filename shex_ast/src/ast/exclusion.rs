use std::str::FromStr;
use std::{fmt, result};

use serde::de::{MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{de, Deserialize, Serialize, Serializer};
use serde_derive::{Deserialize, Serialize};
use srdf::lang::Lang;

use crate::IriRef;

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub enum LiteralExclusion {
    Literal(String),
    LiteralStem(String),
}

impl Serialize for LiteralExclusion {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            LiteralExclusion::Literal(str) => serializer.serialize_str(str),
            LiteralExclusion::LiteralStem(stem) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "LiteralStem")?;
                map.serialize_entry("stem", stem)?;
                map.end()
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub enum IriExclusion {
    Iri(IriRef),
    IriStem(String),
}

impl Serialize for IriExclusion {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            IriExclusion::Iri(iri) => serializer.serialize_str(iri.to_string().as_str()),
            IriExclusion::IriStem(stem) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "IriStem")?;
                map.serialize_entry("stem", stem)?;
                map.end()
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum LanguageExclusion {
    Language(Lang),
    LanguageStem(String),
}

impl Serialize for LanguageExclusion {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            LanguageExclusion::Language(lang) => serializer.serialize_str(lang.value().as_str()),
            LanguageExclusion::LanguageStem(stem) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "LanguageStem")?;
                map.serialize_entry("stem", stem)?;
                map.end()
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Exclusion {
    LiteralExclusion(LiteralExclusion),
    LanguageExclusion(LanguageExclusion),
    IriExclusion(IriExclusion),
}

#[derive(Debug)]
pub struct SomeNoLitExclusion {
    exc: Exclusion,
}

#[derive(Debug)]
pub struct SomeNoIriExclusion {
    exc: Exclusion,
}

#[derive(Debug)]
pub struct SomeNoLanguageExclusion {
    exc: Exclusion,
}

impl Exclusion {
    pub fn parse_literal_exclusions(
        excs: Vec<Exclusion>,
    ) -> Result<Vec<LiteralExclusion>, SomeNoLitExclusion> {
        let mut lit_excs = Vec::new();
        for e in excs {
            match e {
                Exclusion::LiteralExclusion(le) => lit_excs.push(le),
                other => return Err(SomeNoLitExclusion { exc: other }),
            }
        }
        Ok(lit_excs)
    }

    pub fn parse_iri_exclusions(
        excs: Vec<Exclusion>,
    ) -> Result<Vec<IriExclusion>, SomeNoIriExclusion> {
        let mut iri_excs = Vec::new();
        for e in excs {
            match e {
                Exclusion::IriExclusion(le) => iri_excs.push(le),
                other => return Err(SomeNoIriExclusion { exc: other }),
            }
        }
        Ok(iri_excs)
    }

    pub fn parse_language_exclusions(
        excs: Vec<Exclusion>,
    ) -> Result<Vec<LanguageExclusion>, SomeNoIriExclusion> {
        let mut lang_excs = Vec::new();
        for e in excs {
            match e {
                Exclusion::LanguageExclusion(le) => lang_excs.push(le),
                other => return Err(SomeNoIriExclusion { exc: other }),
            }
        }
        Ok(lang_excs)
    }
}

impl Serialize for Exclusion {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Exclusion::IriExclusion(iri) => todo!(),
            Exclusion::LiteralExclusion(LiteralExclusion::Literal(lit)) => {
                todo!()
            }
            Exclusion::LiteralExclusion(LiteralExclusion::LiteralStem(stem)) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "LiteralStem")?;
                map.serialize_entry("stem", stem)?;
                map.end()
            }
            Exclusion::LanguageExclusion(_) => todo!(),
        }
    }
}

impl<'de> Deserialize<'de> for Exclusion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        enum Field {
            Type,
            Stem,
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
                        formatter.write_str("field of exclusion: `type` or `stem`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "type" => Ok(Field::Type),
                            "stem" => Ok(Field::Stem),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct ExclusionVisitor;

        const FIELDS: &'static [&'static str] = &["type", "stem"];

        impl<'de> Visitor<'de> for ExclusionVisitor {
            type Value = Exclusion;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Exclusion value")
            }

            /*fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                FromStr::from_str(s)
                    .map_err(|e| de::Error::custom(format!("Error parsing string `{s}`: {e}")))
            }*/

            fn visit_map<V>(self, mut map: V) -> Result<Exclusion, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut type_: Option<ExclusionType> = None;
                let mut stem: Option<String> = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Type => {
                            if type_.is_some() {
                                return Err(de::Error::duplicate_field("type"));
                            }
                            let value: String = map.next_value()?;

                            let parsed_type_ =
                                ExclusionType::parse(&value.as_str()).map_err(|e| {
                                    de::Error::custom(format!(
                                        "Error parsing Exclusion type, found: {value}. Error: {e:?}"
                                    ))
                                })?;
                            type_ = Some(parsed_type_);
                        }
                        Field::Stem => {
                            if stem.is_some() {
                                return Err(de::Error::duplicate_field("stem"));
                            }
                            stem = Some(map.next_value()?);
                        }
                    }
                }
                match type_ {
                    Some(ExclusionType::LiteralStem) => match stem {
                        Some(stem) => Ok(Exclusion::LiteralExclusion(
                            LiteralExclusion::LiteralStem(stem),
                        )),
                        None => Err(de::Error::missing_field("stem")),
                    },
                    Some(ExclusionType::LanguageStem) => match stem {
                        Some(stem) => Ok(Exclusion::LanguageExclusion(
                            LanguageExclusion::LanguageStem(stem),
                        )),
                        None => Err(de::Error::missing_field("stem")),
                    },
                    Some(ExclusionType::IriStem) => match stem {
                        Some(stem) => Ok(Exclusion::IriExclusion(IriExclusion::IriStem(stem))),
                        None => Err(de::Error::missing_field("stem")),
                    },
                    None => todo!(),
                }
            }
        }

        deserializer.deserialize_any(ExclusionVisitor)
    }
}

#[derive(Debug, PartialEq)]
enum ExclusionType {
    IriStem,
    LiteralStem,
    LanguageStem,
}

#[derive(Debug)]
struct ExclusionTypeError {
    value: String,
}

impl ExclusionType {
    fn parse(s: &str) -> Result<ExclusionType, ExclusionTypeError> {
        match s {
            "IriStem" => Ok(ExclusionType::IriStem),
            "LanguageStem" => Ok(ExclusionType::LanguageStem),
            "LiteralStem" => Ok(ExclusionType::LiteralStem),
            _ => Err(ExclusionTypeError {
                value: s.to_string(),
            }),
        }
    }
}
