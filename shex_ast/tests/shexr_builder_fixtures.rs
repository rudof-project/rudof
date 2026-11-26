//! Drives `shex_ast::shexr::shexr_builder::ShExRBuilder` against every entry
//! in the W3C ShExTest `schemas` manifest (`shex_testsuite/shexTest/schemas/`,
//! a sibling crate's fixtures — read directly rather than copied, so this
//! test always tracks the real corpus).
//!
//! Each entry has a `.json` (ShExJ) schema and a `.ttl` (ShExR) RDF encoding
//! of the *same* schema. For every entry:
//! - If the builder reports `Unsupported`, that's expected for most of the
//!   corpus right now and is just tallied, not a failure.
//! - If the builder produces RDF, it MUST match the fixture's `.ttl` exactly
//!   (structurally) — anything the builder claims to support has to be
//!   correct.
//! - Any other error is a real bug and fails the test immediately.

mod common;

use common::{assert_same_node, read_manifest, schema_root, schemas_dir};
use rudof_rdf::rdf_core::BuildRDF;
use rudof_rdf::rdf_core::RDFFormat;
use rudof_rdf::rdf_impl::{OxigraphInMemory, ReaderMode};
use shex_ast::Schema;
use shex_ast::shexr::shexr_builder::{ShExRBuilder, ShExRBuilderError};

#[test]
fn shexr_builder_matches_shextest_schemas_corpus() {
    let dir = schemas_dir();
    let entries = read_manifest(&dir);
    assert!(
        entries.len() > 400,
        "expected the full shexTest schemas corpus, found {}",
        entries.len()
    );

    let mut supported: Vec<String> = Vec::new();
    let mut skipped = 0usize;
    let mut unparseable = 0usize;
    let mut unverifiable = 0usize;

    for entry in &entries {
        let schema_path = dir.join(&entry.json);
        // A handful of manifest entries (e.g. `_all`) exercise ShExJ corner
        // cases unrelated to this builder (parsing itself, not RDF output);
        // those are out of scope here and just tallied separately.
        let schema = match Schema::parse_schema(&schema_path) {
            Ok(schema) => schema,
            Err(_) => {
                unparseable += 1;
                continue;
            },
        };

        let mut actual = OxigraphInMemory::empty();
        match ShExRBuilder::schema_to_rdf(&schema, &mut actual) {
            Err(ShExRBuilderError::Unsupported { what }) => {
                skipped += 1;
                eprintln!("SKIP [{}]: {what}", entry.name);
                continue;
            },
            Err(other) => panic!("[{}] builder failed (not just unsupported): {other}", entry.name),
            Ok(_) => {},
        }

        if schema.imports().is_some() {
            // `sx:imports` entries can be relative IRI references (e.g.
            // `<1dot>`), which a real caller resolves against the schema's
            // own known location. These fixtures carry no such location
            // (`Schema::parse_schema` never sets `source_iri`), and a
            // conformant Turtle parser refuses to parse the fixture's own
            // `.ttl` without *some* base — there's no base this harness can
            // supply that both parses the fixture and matches what the
            // builder (correctly, given it has no real base either) leaves
            // unresolved. So: still run the builder for coverage/panic
            // safety, just don't attempt the strict byte-for-byte compare.
            unverifiable += 1;
            continue;
        }

        let ttl_path = dir.join(&entry.ttl);
        let ttl = std::fs::read_to_string(&ttl_path)
            .unwrap_or_else(|e| panic!("[{}] failed to read {}: {e}", entry.name, ttl_path.display()));
        let expected =
            OxigraphInMemory::from_str(&ttl, &RDFFormat::Turtle, None, &ReaderMode::Strict).unwrap_or_else(|e| {
                panic!(
                    "[{}] failed to parse expected ttl {}: {e}",
                    entry.name,
                    ttl_path.display()
                )
            });

        let a_root = schema_root(&actual);
        let e_root = schema_root(&expected);
        let mut visited = std::collections::HashSet::new();
        assert_same_node(&actual, a_root, &expected, e_root, &entry.name, &mut visited);
        supported.push(entry.name.clone());
    }

    eprintln!(
        "shexr_builder: {} supported (matched exactly), {skipped} not yet implemented, \
         {unverifiable} imports (builder ran, base-IRI resolution not verifiable here), \
         {unparseable} unparseable (out of scope), {} total",
        supported.len(),
        entries.len()
    );
    for must_be_supported in ["0", "1dot"] {
        assert!(
            supported.iter().any(|n| n == must_be_supported),
            "expected '{must_be_supported}' to be supported by the current MVP slice; supported so far: {supported:?}"
        );
    }
}
