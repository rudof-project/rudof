use std::fmt;

use serde::de::{self};
use serde::{Deserialize, Deserializer};
use serde_derive::Serialize;

#[derive(Serialize, Debug)]
pub(crate) enum ContextEntryValue {
    Base(String),
    Plain(String),
}

#[derive(Deserialize, Serialize, Debug)]
struct Value {
    value: String,
}

// I didn't find a way to automatically derive a deserializer for enums which contain
// both a Map and a String
// The following code is inspired by the following:
// https://stackoverflow.com/questions/66135063/rust-custom-deserialize-implementation
// https://serde.rs/impl-deserialize.html
// https://serde.rs/string-or-struct.html
impl<'de> Deserialize<'de> for ContextEntryValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ContextEntryValueVisitor;

        impl<'de> de::Visitor<'de> for ContextEntryValueVisitor {
            type Value = ContextEntryValue;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("ContextEntryValue")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ContextEntryValue::Plain(value.to_string()))
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: de::MapAccess<'de>,
            {
                if let Some("@base") = map.next_key()? {
                    let value: String = map.next_value()?;
                    Ok(ContextEntryValue::Base(value))
                } else {
                    Err(de::Error::missing_field("@base"))
                }
            }
        }
        deserializer.deserialize_any(ContextEntryValueVisitor {})
    }
}
