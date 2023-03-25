use serde::{Deserialize, Deserializer};
use serde_derive::{Deserialize, Serialize};
use std::fmt;

use serde::de::{self};

#[derive(Deserialize, Serialize, Debug)]
struct Manifest {

    #[serde(rename = "@context")] 
    context: Vec<ContextEntryValue>,

    #[serde(rename = "@graph")] 
    graph: Vec<ManifestGraph>,
}


#[derive(Serialize, Debug)]
enum ContextEntryValue {
    Base(String),
    Plain(String)
}


#[derive(Deserialize, Serialize, Debug)]
struct Value {
    value: String
}

#[derive(Deserialize, Serialize, Debug)]
struct ManifestGraph {

    #[serde(rename = "@id")] 
    id: String,

    #[serde(rename = "@type")] 
    type_: String, 

    #[serde(rename = "rdfs:comment")] 
    comment: String,
    
    entries: Vec<Entry>
}

#[derive(Deserialize, Serialize, Debug)]
struct Entry {

    #[serde(rename = "@id")] 
    id: String,

    #[serde(rename = "@type")] 
    type_: String,

    name: String,
    status: String,
    shex: String,
    json: String,
    ttl: String
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use std::fs;

    #[test]
    fn count_local_manifest_entries() {
        let manifest_path = Path::new("localTest/schemas/manifest.jsonld");
        let manifest = {
            let manifest_str = fs::read_to_string(&manifest_path).unwrap();
            serde_json::from_str::<Manifest>(&manifest_str).unwrap()
        };
        assert_eq!(manifest.graph[0].entries.len(), 2);
    }

    #[test]
    fn count_manifest_entries() {
        let manifest_path = Path::new("shexTest/schemas/manifest.jsonld");
        let manifest = {
            let manifest_str = fs::read_to_string(&manifest_path).unwrap();
            serde_json::from_str::<Manifest>(&manifest_str).unwrap()
        };
        assert_eq!(manifest.graph[0].entries.len(), 433);
    }

}
