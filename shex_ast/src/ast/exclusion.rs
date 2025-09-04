use std::str::FromStr;
use std::{fmt, result};

use serde::de::{MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Serialize, Serializer, de};
use srdf::lang::Lang;

use prefixmap::IriRef;

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
            LiteralExclusion::Literal(lit) => serializer.serialize_str(lit.as_str()),
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
    IriStem(IriRef),
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
    LanguageStem(Lang),
}

impl Serialize for LanguageExclusion {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            LanguageExclusion::Language(lang) => serializer.serialize_str(&lang.to_string()),
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
    Untyped(String),
}

#[derive(Debug)]
pub struct SomeNoLitExclusion {
    pub exc: Exclusion,
}

#[derive(Debug)]
pub struct SomeNoIriExclusion {
    pub exc: Exclusion,
}

#[derive(Debug)]
pub struct SomeNoLanguageExclusion {
    pub exc: Exclusion,
}

impl Exclusion {
    pub fn parse_literal_exclusions(
        excs: Vec<Exclusion>,
    ) -> Result<Vec<LiteralExclusion>, SomeNoLitExclusion> {
        let mut lit_excs = Vec::new();
        for e in excs {
            match e {
                Exclusion::LiteralExclusion(le) => lit_excs.push(le),
                Exclusion::Untyped(s) => lit_excs.push(LiteralExclusion::Literal(s)),
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
            match &e {
                Exclusion::IriExclusion(le) => iri_excs.push(le.clone()),
                v @ Exclusion::Untyped(s) => {
                    let iri = FromStr::from_str(s.as_str())
                        .map_err(|_e| SomeNoIriExclusion { exc: v.clone() })?;
                    iri_excs.push(IriExclusion::Iri(iri))
                }
                other => return Err(SomeNoIriExclusion { exc: other.clone() }),
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
                Exclusion::Untyped(s) => {
                    lang_excs.push(LanguageExclusion::Language(Lang::new_unchecked(s)))
                }
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
            Exclusion::IriExclusion(_iri) => todo!(),
            Exclusion::LiteralExclusion(LiteralExclusion::Literal(_lit)) => {
                todo!()
            }
            Exclusion::LiteralExclusion(LiteralExclusion::LiteralStem(stem)) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "LiteralStem")?;
                map.serialize_entry("stem", stem)?;
                map.end()
            }
            Exclusion::LanguageExclusion(stem) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "LanguageStem")?;
                map.serialize_entry("stem", stem)?;
                map.end()
            }
            Exclusion::Untyped(str) => serializer.serialize_str(str),
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

                impl Visitor<'_> for FieldVisitor {
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

        const FIELDS: &[&str] = &["type", "stem"];

        impl<'de> Visitor<'de> for ExclusionVisitor {
            type Value = Exclusion;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Exclusion value")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Exclusion::Untyped(s.to_string()))
            }

            fn visit_map<V>(self, mut map: V) -> Result<Exclusion, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut type_: Option<ExclusionType> = None;
                let mut stem: Option<StemValue> = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Type => {
                            if type_.is_some() {
                                return Err(de::Error::duplicate_field("type"));
                            }
                            let value: String = map.next_value()?;

                            let parsed_type_ =
                                ExclusionType::parse(value.as_str()).map_err(|e| {
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
                        Some(StemValue::Literal(lit)) => Ok(Exclusion::LiteralExclusion(
                            LiteralExclusion::LiteralStem(lit),
                        )),
                        Some(_) => Err(de::Error::custom(format!(
                            "Stem {stem:?} must be a literal"
                        ))),
                        None => Err(de::Error::missing_field("stem")),
                    },
                    Some(ExclusionType::LanguageStem) => match stem {
                        Some(StemValue::Language(lang)) => Ok(Exclusion::LanguageExclusion(
                            LanguageExclusion::LanguageStem(lang),
                        )),
                        Some(StemValue::Literal(l)) => Ok(Exclusion::LanguageExclusion(
                            LanguageExclusion::LanguageStem(Lang::new_unchecked(l)),
                        )),
                        Some(_) => Err(de::Error::custom(format!(
                            "Stem {stem:?} must be a language"
                        ))),
                        None => Err(de::Error::missing_field("stem")),
                    },
                    Some(ExclusionType::IriStem) => match stem {
                        Some(StemValue::Iri(iri)) => {
                            Ok(Exclusion::IriExclusion(IriExclusion::IriStem(iri)))
                        }
                        Some(_) => Err(de::Error::custom(format!("Stem {stem:?} must be an IRI"))),
                        None => Err(de::Error::missing_field("stem")),
                    },
                    None => Err(de::Error::custom("No value of exclusion type")),
                }
            }
        }

        deserializer.deserialize_any(ExclusionVisitor)
    }
}

#[derive(Debug, PartialEq)]
#[allow(clippy::enum_variant_names)]
enum ExclusionType {
    IriStem,
    LiteralStem,
    LanguageStem,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(untagged)]
enum StemValue {
    Iri(IriRef),
    Literal(String),
    Language(Lang),
}

#[derive(Debug)]
#[allow(dead_code)]
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
