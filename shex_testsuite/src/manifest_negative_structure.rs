use std::{collections::HashMap, path::Path};

use crate::{
    context_entry_value::ContextEntryValue, manifest::Manifest, manifest_error::ManifestError,
};
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
#[serde(from = "ManifestNegativeStructureJson")]
pub struct ManifestNegativeStructure {
    entry_names: Vec<String>,
    map: HashMap<String, NegativeStructureEntry>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ManifestNegativeStructureJson {
    #[serde(rename = "@context")]
    context: Vec<ContextEntryValue>,

    #[serde(rename = "@graph")]
    graph: Vec<ManifestNegativeStructureGraph>,
}

impl<'a> From<ManifestNegativeStructureJson> for ManifestNegativeStructure {
    fn from(m: ManifestNegativeStructureJson) -> Self {
        let entries = &m.graph[0].entries;
        let names = entries.iter().map(|e| e.name.clone()).collect();
        let mut map: HashMap<String, NegativeStructureEntry> = HashMap::new();
        for entry in entries {
            map.insert(entry.name.clone(), entry.clone());
        }
        ManifestNegativeStructure {
            entry_names: names,
            map: map,
        }
    }
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
#[derive(Deserialize, Serialize, Debug, Clone)]
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

impl NegativeStructureEntry {
    pub fn run(&self, base: &Path, debug: u8) -> Result<(), ManifestError> {
        if debug > 0 {
            println!("Runnnig entry: {}...not implemented", self.id);
        }
        Ok(())
    }
}

impl Manifest for ManifestNegativeStructure {
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
    fn count_negative_structure_entries() {
        let manifest_path = Path::new("shexTest/negativeStructure/manifest.jsonld");
        let manifest = {
            let manifest_str = fs::read_to_string(&manifest_path).unwrap();
            serde_json::from_str::<ManifestNegativeStructure>(&manifest_str).unwrap()
        };
        assert_eq!(manifest.entry_names.len(), 14);
    }
}
