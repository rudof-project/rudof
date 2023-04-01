use std::error::Error;
use std::fmt::Display;
use std::path::{Path, PathBuf};
use std::{fmt, fs, io};

use crate::context_entry_value::ContextEntryValue;
use serde::de::{self};
use serde::{Deserialize, Deserializer};
use serde_derive::{Deserialize, Serialize};
use shex_ast::schema_json::SchemaJson;

#[derive(Deserialize, Serialize, Debug)]
pub struct ManifestSchemas {
    #[serde(rename = "@context")]
    context: Vec<ContextEntryValue>,

    #[serde(rename = "@graph")]
    pub graph: Vec<ManifestSchemasGraph>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ManifestSchemasGraph {
    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@type")]
    pub type_: String,

    #[serde(rename = "rdfs:comment")]
    pub comment: String,
    pub entries: Vec<SchemasEntry>,
}

#[derive(Deserialize, Serialize, Debug)]
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
    pub fn run(&self, base: &Path) -> Result<(), Box<(dyn Error + 'static)>> {
        for entry in &self.graph[0].entries {
            entry.run(base)?
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct IOError {
    path: PathBuf,
    error: io::Error,
}

impl IOError {
    pub fn new(path_buf: PathBuf, error: io::Error) -> IOError {
        IOError {
            path: path_buf,
            error: error,
        }
    }
}

impl std::error::Error for IOError {}

impl fmt::Display for IOError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "IO error at path {}: {}",
            self.path.display(),
            self.error
        )
    }
}

impl SchemasEntry {
    pub fn run(&self, base: &Path) -> Result<(), Box<(dyn Error + 'static)>> {
        println!("Runnnig entry: {} with json: {}", self.id, self.json);
        let json_path = Path::new(&self.json);
        let mut attempt = PathBuf::from(base);
        attempt.push(json_path);
        let schema = {
            println!("Attempt on: {}", attempt.display());
            let schema_str = fs::read_to_string(&attempt.as_path()).map_err(|e| IOError {
                path: attempt,
                error: e,
            })?;
            serde_json::from_str::<SchemaJson>(&schema_str)?
        };
        println!("Runnnig entry: {} - {}", self.id, schema.type_);
        Ok(())
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
}
