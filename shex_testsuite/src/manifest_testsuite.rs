use std::fmt;

use serde::{Deserialize, Deserializer};
use serde_derive::{Deserialize, Serialize};

use serde::de::{self};

#[derive(Deserialize, Serialize, Debug)]
struct ManifestSchemas {
    #[serde(rename = "@context")]
    context: Vec<ContextEntryValue>,

    #[serde(rename = "@graph")]
    graph: Vec<ManifestSchemasGraph>,
}

#[derive(Deserialize, Serialize, Debug)]
struct ManifestValidation {
    #[serde(rename = "@context")]
    context: Vec<ContextEntryValue>,

    #[serde(rename = "@graph")]
    graph: Vec<ManifestValidationGraph>,
}

#[derive(Deserialize, Serialize, Debug)]
struct ManifestNegativeStructure {
    #[serde(rename = "@context")]
    context: Vec<ContextEntryValue>,

    #[serde(rename = "@graph")]
    graph: Vec<ManifestNegativeStructureGraph>,
}

#[derive(Deserialize, Serialize, Debug)]
struct ManifestNegativeSyntax {
    #[serde(rename = "@context")]
    context: Vec<ContextEntryValue>,

    #[serde(rename = "@graph")]
    graph: Vec<ManifestNegativeSyntaxGraph>,
}

#[derive(Serialize, Debug)]
enum ContextEntryValue {
    Base(String),
    Plain(String),
}

#[derive(Deserialize, Serialize, Debug)]
struct Value {
    value: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct ManifestSchemasGraph {
    #[serde(rename = "@id")]
    id: String,

    #[serde(rename = "@type")]
    type_: String,

    #[serde(rename = "rdfs:comment")]
    comment: String,
    entries: Vec<SchemasEntry>,
}

#[derive(Deserialize, Serialize, Debug)]
struct ManifestValidationGraph {
    #[serde(rename = "@id")]
    id: String,

    #[serde(rename = "@type")]
    type_: String,

    #[serde(rename = "rdfs:comment")]
    comment: String,

    entries: Vec<ValidationEntry>,
}

#[derive(Deserialize, Serialize, Debug)]
struct ManifestNegativeStructureGraph {
    #[serde(rename = "@id")]
    id: String,

    #[serde(rename = "@type")]
    type_: String,

    #[serde(rename = "rdfs:comment")]
    comment: String,

    entries: Vec<NegativeStructureEntry>,
}

#[derive(Deserialize, Serialize, Debug)]
struct ManifestNegativeSyntaxGraph {
    #[serde(rename = "@id")]
    id: String,

    #[serde(rename = "@type")]
    type_: String,

    #[serde(rename = "rdfs:comment")]
    comment: String,

    entries: Vec<NegativeSyntaxEntry>,
}

#[derive(Deserialize, Serialize, Debug)]
struct SchemasEntry {
    #[serde(rename = "@id")]
    id: String,

    #[serde(rename = "@type")]
    type_: String,
    name: String,
    status: String,
    shex: String,
    json: String,
    ttl: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct ValidationEntry {
    #[serde(rename = "@id")]
    id: String,

    #[serde(rename = "@type")]
    type_: String,
    action: Action,
    #[serde(rename = "extensionResults")]
    extension_results: Vec<ExtensionResult>,
    name: String,
    #[serde(rename = "trait")]
    trait_: Option<Vec<String>>,
    comment: String,
    status: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct NegativeStructureEntry {
    #[serde(rename = "@id")]
    id: String,

    #[serde(rename = "@type")]
    type_: String,
    name: String,
    status: String,
    #[serde(rename = "startRow")]
    start_row: u32,
    #[serde(rename = "startColumn")]
    start_column: u32,
    #[serde(rename = "endRow")]
    end_row: u32,
    #[serde(rename = "endColumn")]
    end_column: u32,
}

#[derive(Deserialize, Serialize, Debug)]
struct NegativeSyntaxEntry {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@type")]
    type_: String,
    name: String,
    status: String,
    shex: String,
    #[serde(rename = "startRow")]
    start_row: Option<u32>,
    #[serde(rename = "startColumn")]
    start_column: Option<u32>,
    #[serde(rename = "endRow")]
    end_row: Option<u32>,
    #[serde(rename = "endColumn")]
    end_column: Option<u32>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Action {
    schema: String,
    shape: Option<String>,
    data: String,
    focus: Option<Focus>,
}

#[derive(Deserialize, Serialize, Debug)]
struct ExtensionResult {
    extension: String,
    prints: String,
}

#[derive(Serialize, Debug)]
enum Focus {
    Single(String),
    Typed(String, String),
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

impl<'de> Deserialize<'de> for Focus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FocusVisitor;

        impl<'de> de::Visitor<'de> for FocusVisitor {
            type Value = Focus;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Focus")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Focus::Single(value.to_string()))
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: de::MapAccess<'de>,
            {
                if let Some("@value") = map.next_key()? {
                    let value: String = map.next_value()?;
                    if let Some("@type") = map.next_key()? {
                        let type_: String = map.next_value()?;
                        Ok(Focus::Typed(value, type_))
                    } else {
                        Err(de::Error::missing_field("@type"))
                    }
                } else {
                    Err(de::Error::missing_field("@value"))
                }
            }
        }
        deserializer.deserialize_any(FocusVisitor {})
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn count_local_manifest_entries() {
        let manifest_path = Path::new("localTest/schemas/manifest.jsonld");
        let manifest = {
            let manifest_str = fs::read_to_string(&manifest_path).unwrap();
            serde_json::from_str::<ManifestSchemas>(&manifest_str).unwrap()
        };
        assert_eq!(manifest.graph[0].entries.len(), 2);
    }

    #[test]
    fn count_schema_entries() {
        let manifest_path = Path::new("shexTest/schemas/manifest.jsonld");
        let manifest = {
            let manifest_str = fs::read_to_string(&manifest_path).unwrap();
            serde_json::from_str::<ManifestSchemas>(&manifest_str).unwrap()
        };
        assert_eq!(manifest.graph[0].entries.len(), 433);
    }

    #[test]
    fn count_validation_entries() {
        let manifest_path = Path::new("shexTest/validation/manifest.jsonld");
        let manifest = {
            let manifest_str = fs::read_to_string(&manifest_path).unwrap();
            serde_json::from_str::<ManifestValidation>(&manifest_str).unwrap()
        };
        assert_eq!(manifest.graph[0].entries.len(), 1166);
    }

    #[test]
    fn count_negative_structure_entries() {
        let manifest_path = Path::new("shexTest/negativeStructure/manifest.jsonld");
        let manifest = {
            let manifest_str = fs::read_to_string(&manifest_path).unwrap();
            serde_json::from_str::<ManifestNegativeStructure>(&manifest_str).unwrap()
        };
        assert_eq!(manifest.graph[0].entries.len(), 14);
    }

    #[test]
    fn count_negative_syntax_entries() {
        let manifest_path = Path::new("shexTest/negativeSyntax/manifest.jsonld");
        let manifest = {
            let manifest_str = fs::read_to_string(&manifest_path).unwrap();
            serde_json::from_str::<ManifestNegativeSyntax>(&manifest_str).unwrap()
        };
        assert_eq!(manifest.graph[0].entries.len(), 100);
    }
}
