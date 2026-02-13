use std::{collections::HashMap, path::Path};
use tracing::debug;

use crate::{context_entry_value::ContextEntryValue, manifest::Manifest, manifest_error::ManifestError};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
#[serde(from = "ManifestNegativeSyntaxJson")]
pub struct ManifestNegativeSyntax {
    entry_names: Vec<String>,
    map: HashMap<String, NegativeSyntaxEntry>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ManifestNegativeSyntaxJson {
    #[serde(rename = "@context")]
    context: Vec<ContextEntryValue>,

    #[serde(rename = "@graph")]
    graph: Vec<ManifestNegativeSyntaxGraph>,
}

impl From<ManifestNegativeSyntaxJson> for ManifestNegativeSyntax {
    fn from(m: ManifestNegativeSyntaxJson) -> Self {
        let entries = &m.graph[0].entries;
        let names = entries.iter().map(|e| e.name.clone()).collect();
        let mut map: HashMap<String, NegativeSyntaxEntry> = HashMap::new();
        for entry in entries {
            map.insert(entry.name.clone(), entry.clone());
        }
        ManifestNegativeSyntax {
            entry_names: names,
            map,
        }
    }
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

impl Manifest for ManifestNegativeSyntax {
    fn len(&self) -> usize {
        self.entry_names.len()
    }
    fn is_empty(&self) -> bool {
        self.entry_names.is_empty()
    }

    fn entry_names(&self) -> Vec<String> {
        self.entry_names.clone() // iter().map(|n| n.clone()).collect()
    }

    fn run_entry(&self, name: &str, base: &Path) -> Result<(), Box<ManifestError>> {
        match self.map.get(name) {
            None => Err(Box::new(ManifestError::NotFoundEntry { name: name.to_string() })),
            Some(entry) => entry.run(base),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
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

impl NegativeSyntaxEntry {
    pub fn run(&self, _base: &Path) -> Result<(), Box<ManifestError>> {
        debug!("Running negative syntax entry: {}...not implemented", self.id);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn count_negative_syntax_entries() {
        let manifest_path = Path::new("shexTest/negativeSyntax/manifest.jsonld");
        let manifest = {
            let manifest_str = fs::read_to_string(manifest_path).unwrap();
            serde_json::from_str::<ManifestNegativeSyntax>(&manifest_str).unwrap()
        };
        assert_eq!(manifest.entry_names.len(), 100);
    }
}
