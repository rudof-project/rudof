use crate::context_entry_value::ContextEntryValue;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct ManifestNegativeStructure {
    #[serde(rename = "@context")]
    context: Vec<ContextEntryValue>,

    #[serde(rename = "@graph")]
    graph: Vec<ManifestNegativeStructureGraph>,
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
        assert_eq!(manifest.graph[0].entries.len(), 14);
    }
}
