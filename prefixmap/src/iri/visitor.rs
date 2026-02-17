use crate::IriRef;
use serde::Deserialize;
use serde::de::Visitor;
use std::str::FromStr;

struct IriRefVisitor;

impl<'de> Visitor<'de> for IriRefVisitor {
    type Value = IriRef;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid IRI or a prefixed name")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match IriRef::from_str(v) {
            Ok(iri_ref) => Ok(iri_ref),
            Err(_) => {
                // Try to parse as prefixed name
                let parts: Vec<&str> = v.splitn(2, ':').collect();
                if parts.len() == 2 {
                    let prefix = parts[0].to_string();
                    let local = parts[1].to_string();
                    Ok(IriRef::Prefixed { prefix, local })
                } else {
                    Err(E::custom(format!("Invalid IRI or prefixed name: {}", v)))
                }
            },
        }
    }
}

impl<'de> Deserialize<'de> for IriRef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(IriRefVisitor)
    }
}
