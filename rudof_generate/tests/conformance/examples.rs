//! Every schema shipped in `examples/schemas` must generate data that is valid against
//! it, checked by the validator of its own language: ShEx (`.shex`) against the
//! generator's ShapeMap, SHACL (`.ttl`) through its targets, PG-Schema (`.pgs`) against
//! the generator's type map. The examples are what users and the notebooks start from,
//! so a schema that stops generating valid data is a broken example.

use crate::common::*;
use rudof_generate::GeneratorConfig;
use rudof_generate::config::OutputFormat;
use std::path::{Path, PathBuf};

fn shipped_schemas() -> Vec<PathBuf> {
    fn walk(dir: &Path, found: &mut Vec<PathBuf>) {
        for entry in std::fs::read_dir(dir).unwrap().flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk(&path, found);
            } else if path
                .extension()
                .is_some_and(|e| e == "shex" || e == "ttl" || e == "pgs")
            {
                found.push(path);
            }
        }
    }
    let mut found = Vec::new();
    walk(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/schemas"),
        &mut found,
    );
    found.sort();
    found
}

async fn assert_generates_valid_data(schema_path: &Path) {
    let schema = std::fs::read_to_string(schema_path).unwrap();
    let name = schema_path.display();
    match schema_path.extension().and_then(|e| e.to_str()) {
        Some("pgs") => {
            let fixture = Fixture::pgschema(&schema);
            let dir = fixture.dir.path();
            let mut cfg = GeneratorConfig::default();
            cfg.generation.entity_count = 40;
            cfg.generation.seed = Some(3);
            cfg.output.path = dir.join("out.pg");
            cfg.output.format = OutputFormat::Pg;
            cfg.output.write_stats = false;
            cfg.output.shapemap = Some(dir.join("out.map"));
            let graph = generate(&fixture, cfg).await;
            let map = std::fs::read_to_string(dir.join("out.map")).unwrap();
            let v = validate_pgschema(&graph, &schema, &map);
            assert!(v.failing.is_empty() && v.conformant > 0, "{name}: {}", v.report);
        },
        Some("shex") => {
            let fixture = Fixture::shex(&schema);
            let map_path = fixture.dir.path().join("out.shapemap");
            let mut cfg = config(&fixture.output(), 40, 3);
            cfg.output.shapemap = Some(map_path.clone());
            let data = generate(&fixture, cfg).await;
            let map = std::fs::read_to_string(map_path).unwrap();
            let v = validate_shex(&data, &schema, &map);
            assert!(v.failing.is_empty() && v.conformant > 0, "{name}: {}", v.report);
        },
        _ => {
            let fixture = Fixture::shacl(&schema);
            let data = generate(&fixture, config(&fixture.output(), 40, 3)).await;
            let v = validate_shacl(&data, &schema);
            assert!(v.failing.is_empty(), "{name}: {}", v.report);
        },
    }
}

#[tokio::test]
async fn every_shipped_schema_generates_valid_data() {
    let schemas = shipped_schemas();
    assert!(
        schemas.len() >= 30,
        "expected the shipped schemas, found {}",
        schemas.len()
    );
    for schema in &schemas {
        assert_generates_valid_data(schema).await;
    }
}
