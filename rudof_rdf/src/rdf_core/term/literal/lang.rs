use serde::{Deserialize, Serialize};
use thiserror::Error;

/// A language tag wrapper for IETF BCP 47 language tags.
///
/// Standard BCP-47 tags are normalized on construction:
/// - Primary language subtag lowercased (e.g., "EN" → "en")
/// - Script subtags title-cased (e.g., "latn" → "Latn")
/// - Region subtags uppercased (e.g., "us" → "US")
///
/// Non-standard but syntactically valid tags (e.g., "fr-be-fbcl" from ShEx
/// conformance tests) are accepted and lowercased rather than rejected.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(transparent)]
pub struct Lang {
    lang: String,
}

impl Lang {
    /// Creates a new `Lang` from a string-like value.
    ///
    /// # Errors
    ///
    /// Returns `LangParseError` if the tag is not even syntactically valid
    /// (empty subtag or non-alphanumeric characters).
    pub fn new(lang: impl Into<String>) -> Result<Lang, LangParseError> {
        let s: String = lang.into();
        // Try strict BCP-47 normalization first
        if let Ok(normalized) = oxilangtag::LanguageTag::parse_and_normalize(&s) {
            return Ok(Lang {
                lang: normalized.into_inner(),
            });
        }
        // Fall back to accepting any syntactically valid lang tag
        // (e.g. non-IANA-registered subtag sequences used in ShEx test data)
        if is_syntactically_valid_lang(&s) {
            return Ok(Lang {
                lang: s.to_ascii_lowercase(),
            });
        }
        Err(LangParseError::InvalidLangTag(s))
    }

    /// Returns the language tag as a string slice.
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.lang
    }
}

/// Returns `true` if the string matches the Turtle language tag grammar:
/// `[a-zA-Z]+ ('-' [a-zA-Z0-9]+)*`
fn is_syntactically_valid_lang(s: &str) -> bool {
    let mut parts = s.split('-');
    let first = parts.next().unwrap_or("");
    if first.is_empty() || !first.bytes().all(|b| b.is_ascii_alphabetic()) {
        return false;
    }
    for part in parts {
        if part.is_empty() || !part.bytes().all(|b| b.is_ascii_alphanumeric()) {
            return false;
        }
    }
    true
}

impl std::fmt::Display for Lang {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.lang)
    }
}

/// Error type for language tag parsing failures.
#[derive(Error, Debug)]
pub enum LangParseError {
    #[error("Invalid language tag: {0}")]
    InvalidLangTag(String),
}
