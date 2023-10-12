use crate::manifest::Manifest;
use crate::manifest_error::ManifestError;
use std::collections::HashMap;
use std::fmt;
use std::path::Path;

use crate::context_entry_value::ContextEntryValue;
use log::debug;
use serde::de::{self};
use serde::{Deserialize, Deserializer};
use serde_derive::{Deserialize, Serialize};
use shex_ast::ast::Schema as SchemaJson;

#[derive(Deserialize, Debug)]
#[serde(from = "ManifestSchemasJson")]
pub struct ManifestSchemas {
    entry_names: Vec<String>,
    map: HashMap<String, SchemasEntry>,
}

impl<'a> From<ManifestSchemasJson> for ManifestSchemas {
    fn from(m: ManifestSchemasJson) -> Self {
        let entries = &m.graph[0].entries;
        let names = entries.iter().map(|e| e.name.clone()).collect();
        let mut map = HashMap::new();
        for entry in entries {
            map.insert(entry.name.clone(), entry.clone());
        }
        ManifestSchemas {
            entry_names: names,
            map: map,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ManifestSchemasJson {
    #[serde(rename = "@context")]
    context: Vec<ContextEntryValue>,

    #[serde(rename = "@graph")]
    pub graph: Vec<ManifestSchemasGraph>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ManifestSchemasGraph {
    #[serde(rename = "@id")]
    id: String,

    #[serde(rename = "@type")]
    type_: String,

    #[serde(rename = "rdfs:comment")]
    comment: String,
    entries: Vec<SchemasEntry>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SchemasEntry {
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

impl ManifestSchemas {
    pub fn run(&self, base: &Path, debug: u8) -> Result<(), ManifestError> {
        for entry in self.map.values() {
            entry.run(base)?
        }
        Ok(())
    }
}

impl SchemasEntry {
    pub fn run(&self, base: &Path) -> Result<(), ManifestError> {
        debug!("Runnnig entry: {} with json: {}", self.id, self.json);
        let schema_parsed = SchemaJson::parse_schema_name(&self.json, base).map_err(|e| {
            ManifestError::SchemaJsonError {
                error: e,
                entry_name: self.name.to_string(),
            }
        })?;

        let schema_serialized = serde_json::to_string_pretty(&schema_parsed).map_err(|e| {
            ManifestError::SchemaSerializationError {
                schema_parsed: schema_parsed.clone(),
                error: e,
            }
        })?;

        let schema_parsed_after_serialization =
            serde_json::from_str::<shex_ast::ast::Schema>(&schema_serialized).map_err(|e| {
                ManifestError::SchemaParsingAfterSerialization {
                    schema_serialized,
                    error: e,
                }
            })?;
        debug!("Entry run: {} - {}", self.id, schema_parsed.type_);

        if (schema_parsed == schema_parsed_after_serialization) {
            Ok(())
        } else {
            Err(ManifestError::SchemasDifferent {
                schema_parsed,
                schema_parsed_after_serialization,
            })
        }
    }
}

impl Manifest for ManifestSchemas {
    fn len(&self) -> usize {
        self.entry_names.len()
    }

    fn entry_names(&self) -> Vec<String> {
        self.entry_names.clone() // iter().map(|n| n.clone()).collect()
    }

    fn run_entry(&self, name: &str, base: &Path) -> Result<(), ManifestError> {
        match self.map.get(name) {
            None => Err(ManifestError::NotFoundEntry {
                name: name.to_string(),
            }),
            Some(entry) => entry.run(base),
        }
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
        assert_eq!(manifest.len(), 2);
    }

    #[test]
    fn count_schema_entries() {
        let manifest_path = Path::new("shexTest/schemas/manifest.jsonld");
        let manifest = {
            let manifest_str = fs::read_to_string(&manifest_path).unwrap();
            serde_json::from_str::<ManifestSchemas>(&manifest_str).unwrap()
        };
        assert_eq!(manifest.len(), 433);
    }
}

/* This code is just for testing iterators...
impl IntoIterator for ManifestSchemas {
    type Item = SchemasEntry;
    type IntoIter = ManifestSchemaIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        ManifestSchemaIntoIterator {
            manifest: self,
            index: 0,
        }
    }
}

pub struct ManifestSchemaIntoIterator {
    manifest: ManifestSchemas,
    index: usize,
}

impl Iterator for ManifestSchemaIntoIterator {
    type Item = SchemasEntry;
    fn next(&mut self) -> Option<SchemasEntry> {
        // self.manifest.map.into_iter().next() //
        todo!()
    }
}
*/
