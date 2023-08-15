use crate::context_entry_value::ContextEntryValue;
use crate::manifest::Manifest;
use crate::manifest_error::ManifestError;
use serde::de::{self};
use serde::{Deserialize, Deserializer};
use serde_derive::{Deserialize, Serialize};
use shex_ast::SchemaJson;
use srdf_graph::SRDFGraph;
use std::collections::HashMap;
use std::fmt;
use std::path::Path;

#[derive(Deserialize, Debug)]
#[serde(from = "ManifestValidationJson")]
pub struct ManifestValidation {
    entry_names: Vec<String>,
    map: HashMap<String, ValidationEntry>,
}

#[derive(Deserialize, Serialize, Debug)]
struct ManifestValidationJson {
    #[serde(rename = "@context")]
    context: Vec<ContextEntryValue>,

    #[serde(rename = "@graph")]
    graph: Vec<ManifestValidationGraph>,
}

impl<'a> From<ManifestValidationJson> for ManifestValidation {
    fn from(m: ManifestValidationJson) -> Self {
        let entries = &m.graph[0].entries;
        let names = entries.iter().map(|e| e.name.clone()).collect();
        let mut map: HashMap<String, ValidationEntry> = HashMap::new();
        for entry in entries {
            map.insert(entry.name.clone(), entry.clone());
        }
        ManifestValidation {
            entry_names: names,
            map: map,
        }
    }
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

#[derive(Deserialize, Serialize, Debug, Clone)]
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

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Action {
    schema: String,
    shape: Option<String>,
    data: String,
    focus: Option<Focus>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct ExtensionResult {
    extension: String,
    prints: String,
}

#[derive(Serialize, Debug, Clone)]
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

fn change_extension(name: String, old_extension: String, new_extension: String) -> String {
    if name.ends_with(&old_extension) {
        let (first, _) = name.split_at(name.len() - old_extension.len());
        format!("{}{}", first, new_extension)
    } else {
        name
    }
}

fn parse_schema(
    schema: &String,
    base: &Path,
    entry_name: &String,
    debug: u8,
) -> Result<SchemaJson, ManifestError> {
    let new_schema_name =
        change_extension(schema.to_string(), ".shex".to_string(), ".json".to_string());

    if debug > 0 {
        println!("schema: {}, new_schema_name: {}", schema, new_schema_name);
    }
    SchemaJson::parse_schema_name(&new_schema_name, base, debug).map_err(|e| {
        ManifestError::SchemaJsonError {
            error: e,
            entry_name: entry_name.to_string(),
        }
    })
}

impl ValidationEntry {
    pub fn run(&self, base: &Path, debug: u8) -> Result<(), ManifestError> {
        let graph = SRDFGraph::parse_data(&self.action.data, base, debug)?;
        let schema = parse_schema(&self.action.schema, base, &self.name, debug)?;

        if debug > 0 {
            println!(
                "Runnnig entry: {} with schema: {}, data: {}, #triples: {}",
                self.id,
                self.action.schema,
                self.action.data,
                graph.len()
            );
        }
        Ok(())
    }
}

impl Manifest for ManifestValidation {
    fn len(&self) -> usize {
        self.entry_names.len()
    }

    fn entry_names(&self) -> Vec<String> {
        self.entry_names.clone() // iter().map(|n| n.clone()).collect()
    }

    fn run_entry(&self, name: &str, base: &Path, debug: u8) -> Result<(), ManifestError> {
        match self.map.get(name) {
            None => Err(ManifestError::NotFoundEntry {
                name: name.to_string(),
            }),
            Some(entry) => entry.run(base, debug),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn count_validation_entries() {
        let manifest_path = Path::new("shexTest/validation/manifest.jsonld");
        let manifest = {
            let manifest_str = fs::read_to_string(&manifest_path).unwrap();
            serde_json::from_str::<ManifestValidation>(&manifest_str).unwrap()
        };
        assert_eq!(manifest.entry_names.len(), 1166);
    }
}
