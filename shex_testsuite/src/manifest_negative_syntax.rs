use crate::context_entry_value::ContextEntryValue;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct ManifestNegativeSyntax {
    #[serde(rename = "@context")]
    context: Vec<ContextEntryValue>,

    #[serde(rename = "@graph")]
    graph: Vec<ManifestNegativeSyntaxGraph>,
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

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
