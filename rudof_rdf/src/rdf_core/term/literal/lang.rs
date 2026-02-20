use std::hash::Hash;

use oxilangtag::LanguageTag;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// A validated language tag wrapper that ensures normalization and validity.
///
/// This type wraps an `oxilangtag::LanguageTag` and provides a type-safe way to work
/// with IETF BCP 47 language tags (e.g., "en", "en-US", "es-ES"). The language tag is
/// validated and normalized upon construction, ensuring it conforms to the standard.
///
/// # Normalization
///
/// Language tags are automatically normalized according to BCP 47 rules:
/// - The primary language subtag is lowercased (e.g., "EN" → "en")
/// - Script subtags are title-cased (e.g., "latn" → "Latn")
/// - Region subtags are uppercased (e.g., "us" → "US")
///
/// # Serialization
///
/// The `#[serde(transparent)]` attribute ensures that this type serializes as a plain
/// string rather than a struct with a single field, making it compatible with JSON
/// schemas that expect language tags as strings.
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(transparent)]
pub struct Lang {
    lang: LanguageTag<String>,
}

impl Lang {
    /// Creates a new `Lang` from a string-like value.
    ///
    /// # Errors
    ///
    /// Returns `LangParseError` if the language tag is invalid.
    pub fn new(lang: impl Into<String>) -> Result<Lang, LangParseError> {
        let lang = oxilangtag::LanguageTag::parse_and_normalize(&lang.into())?;
        Ok(Lang { lang })
    }

    /// Returns the language tag as a string slice.
    #[inline]
    pub fn as_str(&self) -> &str {
        self.lang.as_str()
    }
}

impl std::fmt::Display for Lang {
    /// Formats the language tag for display.
    ///
    /// This outputs the normalized form of the language tag, making it suitable
    /// for user-facing output, logging, and serialization to text formats.
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.lang)
    }
}

/// Error type for language tag parsing failures.
///
/// This error is returned when attempting to construct a `Lang` from an invalid
/// language tag string. It wraps the underlying parsing error from `oxilangtag`.
#[derive(Error, Debug)]
pub enum LangParseError {
    #[error("Invalid language tag: {0}")]
    InvalidLangTag(#[from] oxilangtag::LanguageTagParseError),
}
