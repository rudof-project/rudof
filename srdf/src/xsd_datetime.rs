use core::fmt;
use oxsdatatypes::DateTime;
use serde::de::Visitor;
use serde::{Deserialize, Serialize, Serializer};
use std::fmt::Display;
use std::hash::Hash;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub struct XsdDateTime {
    value: DateTime,
}

impl XsdDateTime {
    pub fn new(value: &str) -> Result<Self, String> {
        DateTime::from_str(value)
            .map(|dt| XsdDateTime { value: dt })
            .map_err(|e| e.to_string())
    }

    pub fn value(&self) -> &DateTime {
        &self.value
    }
}

impl Eq for XsdDateTime {}

impl Hash for XsdDateTime {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Use the value's hash directly
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
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct XsdDateTimeVisitor;

        impl Visitor<'_> for XsdDateTimeVisitor {
            type Value = XsdDateTime;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("XsdDateTime")
            }
        }

        deserializer.deserialize_any(XsdDateTimeVisitor)
    }
}

impl Display for XsdDateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
mod tests {}
