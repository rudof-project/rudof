use oxsdatatypes::DateTime;
use std::str::FromStr;
use thiserror::Error;
use core::fmt;
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// A validated XSD DateTime wrapper that ensures type safety and correct formatting.
///
/// This type wraps `oxsdatatypes::DateTime` and provides a type-safe way to work
/// with XSD DateTime values (e.g., "2026-01-20T12:34:56Z"). The datetime string
/// is validated upon construction, ensuring it can be parsed into a proper `DateTime`.
#[derive(Debug, PartialEq, Eq, Clone, PartialOrd)]
pub struct XsdDateTime {
    value: DateTime,
}

impl XsdDateTime {
    /// Creates a new `XsdDateTime` from a string slice.
    ///
    /// # Errors
    ///
    /// Returns `XsdDateTimeParseError` if the string cannot be parsed as a valid XSD DateTime.
    pub fn new(value: &str) -> Result<Self, XsdDateTimeParseError> {
        DateTime::from_str(value)
            .map(|dt| Self { value: dt })
            .map_err(|e| XsdDateTimeParseError::InvalidDateTime(e))
    }

    /// Returns a reference to the underlying `DateTime` value.
    #[inline]
    pub fn value(&self) -> &DateTime {
        &self.value
    }

    /// Consumes self and returns the underlying `DateTime` value.
    #[inline]
    pub fn into_inner(self) -> DateTime {
        self.value
    }
}

impl std::hash::Hash for XsdDateTime {
    /// Serializes the `XsdDateTime` as its string lexical representation.
    ///
    /// This uses `DateTime`'s `Display` implementation via `to_string()` and
    /// encodes it as a JSON string (or equivalent, depending on the serializer).
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl Serialize for XsdDateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.value.to_string())
    }
}

impl<'de> Deserialize<'de> for XsdDateTime {
    /// Deserializes an `XsdDateTime` from a string.
    ///
    /// The input is expected to be a string containing a valid XSD `dateTime`
    /// lexical value. Any parsing error is converted into a serde
    /// deserialization error via `de::Error::custom`.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct XsdDateTimeVisitor;

        impl<'de> Visitor<'de> for XsdDateTimeVisitor {
            type Value = XsdDateTime;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a valid XSD DateTime string")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                XsdDateTime::new(value).map_err(de::Error::custom)
            }
        }

        deserializer.deserialize_str(XsdDateTimeVisitor)
    }
}

impl fmt::Display for XsdDateTime {
    /// Formats the `XsdDateTime` using the inner `DateTime`'s `Display` impl.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

/// Error type for XSD DateTime parsing failures.
#[derive(Error, Debug)]
pub enum XsdDateTimeParseError {
    #[error("Invalid XSD DateTime: {0}")]
    InvalidDateTime(#[from] oxsdatatypes::ParseDateTimeError),
}