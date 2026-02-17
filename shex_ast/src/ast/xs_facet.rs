use std::result;

use rdf::rdf_core::term::literal::NumericLiteral;
use rust_decimal::prelude::*;
use serde::ser::SerializeMap;
use serde::{Deserialize, Serialize, Serializer};
use thiserror::Error;
// use void::Void;

use crate::ast::serde_string_or_struct::*;

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum XsFacet {
    StringFacet(StringFacet),
    NumericFacet(NumericFacet),
}

impl XsFacet {
    pub fn pattern(pat: &str) -> XsFacet {
        XsFacet::StringFacet(StringFacet::Pattern(Pattern::new(pat)))
    }

    pub fn pattern_flags(pat: &str, flags: &str) -> XsFacet {
        XsFacet::StringFacet(StringFacet::Pattern(Pattern::new_flags(pat, flags)))
    }

    pub fn length(len: usize) -> XsFacet {
        XsFacet::StringFacet(StringFacet::Length(len))
    }

    pub fn min_length(len: usize) -> XsFacet {
        XsFacet::StringFacet(StringFacet::MinLength(len))
    }

    pub fn max_length(len: usize) -> XsFacet {
        XsFacet::StringFacet(StringFacet::MaxLength(len))
    }

    pub fn min_inclusive(nl: NumericLiteral) -> XsFacet {
        XsFacet::NumericFacet(NumericFacet::MinInclusive(nl))
    }

    pub fn max_inclusive(nl: NumericLiteral) -> XsFacet {
        XsFacet::NumericFacet(NumericFacet::MaxInclusive(nl))
    }

    pub fn min_exclusive(nl: NumericLiteral) -> XsFacet {
        XsFacet::NumericFacet(NumericFacet::MinExclusive(nl))
    }

    pub fn max_exclusive(nl: NumericLiteral) -> XsFacet {
        XsFacet::NumericFacet(NumericFacet::MaxExclusive(nl))
    }

    pub fn totaldigits(n: usize) -> XsFacet {
        XsFacet::NumericFacet(NumericFacet::TotalDigits(n))
    }

    pub fn fractiondigits(n: usize) -> XsFacet {
        XsFacet::NumericFacet(NumericFacet::FractionDigits(n))
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum StringFacet {
    Length(usize),
    MinLength(usize),
    MaxLength(usize),

    #[serde(
        serialize_with = "serialize_pattern",
        deserialize_with = "deserialize_string_or_struct"
    )]
    Pattern(Pattern),
}

fn serialize_pattern<S>(p: &Pattern, serializer: S) -> result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match p {
        Pattern { str, flags: None } => {
            let mut map = serializer.serialize_map(Some(1))?;
            map.serialize_entry("pattern", str)?;
            map.end()
        },
        // str.serialize(serializer),
        Pattern { str, flags: Some(fs) } => {
            let mut map = serializer.serialize_map(Some(2))?;
            map.serialize_entry("pattern", str)?;
            map.serialize_entry("flags", fs)?;
            map.end()
        },
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct Pattern {
    pub str: String,
    pub flags: Option<String>,
}

impl Pattern {
    pub fn new(str: &str) -> Pattern {
        Pattern {
            str: str.to_string(),
            flags: None,
        }
    }

    pub fn new_flags(str: &str, flags: &str) -> Pattern {
        Pattern {
            str: str.to_string(),
            flags: Some(flags.to_string()),
        }
    }

    pub fn regex(&self) -> &str {
        &self.str
    }

    pub fn flags(&self) -> Option<&str> {
        self.flags.as_deref()
    }
}

impl FromStr for Pattern {
    type Err = PatternError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Pattern {
            str: s.to_string(),
            flags: None,
        })
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone, Error)]
pub enum PatternError {
    #[error("Invalid pattern")]
    InvalidPattern,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub enum NumericFacet {
    MinInclusive(NumericLiteral),
    MinExclusive(NumericLiteral),
    MaxInclusive(NumericLiteral),
    MaxExclusive(NumericLiteral),
    TotalDigits(usize),
    FractionDigits(usize),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde_xsfacet_pattern() {
        let pattern = StringFacet::Pattern(Pattern {
            str: "o*".to_string(),
            flags: None,
        });

        let json_pattern = serde_json::to_string(&pattern).unwrap();
        assert_eq!(json_pattern, "{\"pattern\":\"o*\"}");
    }

    #[test]
    fn test_serde_xsfacet_pattern_flags() {
        let pattern = StringFacet::Pattern(Pattern {
            str: "o*".to_string(),
            flags: Some("i".to_string()),
        });

        let json_pattern = serde_json::to_string(&pattern).unwrap();
        assert_eq!(json_pattern, "{\"pattern\":\"o*\",\"flags\":\"i\"}");
    }
}
