use std::{collections::HashMap, path::Path};

use crate::{
    context_entry_value::ContextEntryValue, manifest::Manifest, manifest_error::ManifestError,
    manifest_mode::ManifestShExSyntaxMode,
};
use serde::{Deserialize, Serialize};
#[cfg(not(target_family = "wasm"))]
use shex_ast::ir::{map_state::MapState, schema_ir::SchemaIR, semantic_actions_registry::SemanticActionsRegistry};
#[cfg(not(target_family = "wasm"))]
use shex_ast::{ResolveMethod, ShExParser, ir::ast2ir::AST2IR};
#[cfg(not(target_family = "wasm"))]
use shex_validation::{Validator, ValidatorConfig};
use tracing::debug;

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

impl From<ManifestNegativeStructureJson> for ManifestNegativeStructure {
    fn from(m: ManifestNegativeStructureJson) -> Self {
        let entries = &m.graph[0].entries;
        let names = entries.iter().map(|e| e.name.clone()).collect();
        let mut map: HashMap<String, NegativeStructureEntry> = HashMap::new();
        for entry in entries {
            map.insert(entry.name.clone(), entry.clone());
        }
        ManifestNegativeStructure {
            entry_names: names,
            map,
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
    shex: String,
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
    /// The schema of the entry follows the ShExC grammar, but it is not a valid
    /// schema (e.g. it refers to a shape that doesn't exist, or has a cycle
    /// through a negation). The entry passes when rudof rejects the schema: when
    /// it parses it (for what the AST can't hold, e.g. a cardinality on an
    /// inclusion), when it compiles it, or when it creates a validator for it
    /// (which checks the negation cycles).
    #[cfg(not(target_family = "wasm"))]
    pub fn run(&self, base: &Path) -> Result<(), Box<ManifestError>> {
        let path = base.join(&self.shex);
        let base_iri = crate::manifest_validation::path_to_iri(&path)?;
        match Self::rejection(&path, &base_iri) {
            Some(error) => {
                debug!("Negative structure entry {} rejected as expected: {error}", self.name);
                Ok(())
            },
            None => Err(Box::new(ManifestError::NegativeStructureAccepted {
                entry_name: self.name.clone(),
            })),
        }
    }

    /// Why rudof rejects the schema at `path`, or `None` if it accepts it
    #[cfg(not(target_family = "wasm"))]
    fn rejection(path: &Path, base_iri: &rudof_iri::IriS) -> Option<String> {
        let schema = match ShExParser::parse_buf(path, Some(base_iri.clone())) {
            Ok(schema) => schema,
            Err(e) => return Some(format!("parse: {e}")),
        };
        let config = ValidatorConfig::default();
        let mut map_state = MapState::default();
        let registry = SemanticActionsRegistry::default();
        registry.set_map_state(&mut map_state);
        let mut compiler = AST2IR::new(&ResolveMethod::default(), map_state);
        let mut compiled_schema = SchemaIR::new(registry);
        if let Err(e) = compiler.compile(
            &schema,
            base_iri,
            &Some(base_iri.clone()),
            &mut compiled_schema,
            config.external_resolvers(),
        ) {
            return Some(format!("compile: {e}"));
        }
        Validator::new(&compiled_schema, &config)
            .err()
            .map(|e| format!("validator: {e}"))
    }

    #[cfg(target_family = "wasm")]
    pub fn run(&self, _base: &Path) -> Result<(), Box<ManifestError>> {
        debug!("Running negative structure entry: {}...not implemented", self.id);
        Ok(())
    }
}

impl Manifest for ManifestNegativeStructure {
    fn len(&self) -> usize {
        self.entry_names.len()
    }

    fn is_empty(&self) -> bool {
        self.entry_names.is_empty()
    }

    fn entry_names(&self) -> Vec<String> {
        self.entry_names.clone() // iter().map(|n| n.clone()).collect()
    }

    fn run_entry(
        &self,
        name: &str,
        base: &Path,
        _manifest_shex_syntax_mode: ManifestShExSyntaxMode,
    ) -> Result<(), Box<ManifestError>> {
        match self.map.get(name) {
            None => Err(Box::new(ManifestError::NotFoundEntry { name: name.to_string() })),
            Some(entry) => entry.run(base),
        }
    }

    fn has_traits(&self, _name: &str) -> Result<Vec<std::string::String>, Box<ManifestError>> {
        Ok(Vec::new())
    }
}

/* This test contains a hardcoded number and is flaky
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn count_negative_structure_entries() {
        let manifest_path = Path::new("shexTest/negativeStructure/manifest.jsonld");
        let manifest = {
            let manifest_str = fs::read_to_string(manifest_path).unwrap();
            serde_json::from_str::<ManifestNegativeStructure>(&manifest_str).unwrap()
        };
        assert_eq!(manifest.entry_names.len(), 14);
    }
}
*/
