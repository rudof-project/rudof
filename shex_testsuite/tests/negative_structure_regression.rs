#[cfg(not(target_family = "wasm"))]
use shex_testsuite::{
    manifest::Manifest, manifest_mode::ManifestShExSyntaxMode, manifest_negative_structure::ManifestNegativeStructure,
};
#[cfg(not(target_family = "wasm"))]
use std::panic::{AssertUnwindSafe, catch_unwind};
#[cfg(not(target_family = "wasm"))]
use std::{collections::BTreeSet, fs, path::Path};

#[cfg(not(target_family = "wasm"))]
const MANIFEST: &str = "shexTest/negativeStructure/manifest.jsonld";
#[cfg(not(target_family = "wasm"))]
const BASELINE: &str = "tests/baseline_negative_structure_failing.txt";

/// Entries that are not run, because rudof accepts their schemas on purpose
#[cfg(not(target_family = "wasm"))]
const SKIPPED: &[(&str, &str)] = &[
    (
        "1focusRefANDSelfdot",
        "a shape that refers to itself through AND without a triple constraint is \
         accepted, with a greatest fixpoint for the recursive reference",
    ),
    (
        "Cycle2Extra",
        "a shape that refers to itself through an EXTRA predicate is accepted: \
         EXTRA adds no negative edge to the dependency graph, so that recursive \
         schemas like `:C [ :a ] OR EXTRA :p { :p @:C + }` are valid",
    ),
];

/// Regression test for the negative structure tests of the ShEx test suite.
///
/// Each schema in `shexTest/negativeStructure/manifest.jsonld` is valid ShExC,
/// but not a valid schema (a missing reference, a cycle through a negation or
/// through EXTENDS, ...). An entry passes when rudof rejects its schema. The
/// entries in `SKIPPED` are not run.
///
/// Like `validation_regression`, this compares the current set of failing
/// entries against a stored baseline so that improvements and regressions are
/// both visible without having to fix every known failure at once.
#[cfg(not(target_family = "wasm"))]
#[test]
fn negative_structure_regression() {
    let pkg = Path::new(env!("CARGO_MANIFEST_DIR"));
    let manifest_path = pkg.join(MANIFEST);
    let baseline_path = pkg.join(BASELINE);

    let manifest_str = fs::read_to_string(&manifest_path).expect("Failed to read manifest.jsonld");
    let manifest: ManifestNegativeStructure = serde_json::from_str(&manifest_str).expect("Failed to parse manifest");

    let base = manifest_path.parent().unwrap();

    let names = manifest.entry_names();
    for (skipped, _) in SKIPPED {
        assert!(
            names.iter().any(|n| n == skipped),
            "skipped entry {skipped} is not in the manifest"
        );
    }

    let mut current_failing: BTreeSet<String> = BTreeSet::new();
    for name in names {
        if let Some((_, reason)) = SKIPPED.iter().find(|(skipped, _)| *skipped == name) {
            eprintln!("SKIP {name}: {reason}");
            continue;
        }
        let result = catch_unwind(AssertUnwindSafe(|| {
            manifest.run_entry(&name, base, ManifestShExSyntaxMode::ShExC)
        }));
        match result {
            Ok(Ok(())) => {},
            Ok(Err(e)) => {
                eprintln!("FAIL {name}: {e}");
                current_failing.insert(name);
            },
            Err(_) => {
                eprintln!("FAIL {name}: panicked");
                current_failing.insert(name);
            },
        }
    }

    if !baseline_path.exists() {
        let content = current_failing.iter().cloned().collect::<Vec<_>>().join("\n");
        fs::write(&baseline_path, &content).expect("Failed to write baseline");
        panic!(
            "Baseline created with {} failing tests at {}. Please commit this file.",
            current_failing.len(),
            BASELINE
        );
    }

    let baseline_content = fs::read_to_string(&baseline_path).expect("Failed to read baseline");
    let baseline_failing: BTreeSet<String> = baseline_content
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(str::to_string)
        .collect();

    let regressions: Vec<&String> = current_failing.difference(&baseline_failing).collect();
    let improvements: Vec<&String> = baseline_failing.difference(&current_failing).collect();

    let mut messages: Vec<String> = Vec::new();

    if !improvements.is_empty() {
        let new_content = current_failing.iter().cloned().collect::<Vec<_>>().join("\n");
        fs::write(&baseline_path, new_content).expect("Failed to update baseline");

        let names = improvements.iter().map(|s| s.as_str()).collect::<Vec<_>>().join("\n  ");
        messages.push(format!(
            "PROGRESS: {} test(s) newly passing — baseline updated, please commit {}:\n  {}",
            improvements.len(),
            BASELINE,
            names
        ));
    }

    if !regressions.is_empty() {
        let names = regressions.iter().map(|s| s.as_str()).collect::<Vec<_>>().join("\n  ");
        messages.push(format!(
            "REGRESSION: {} new test(s) failing:\n  {}",
            regressions.len(),
            names
        ));
    }

    if !messages.is_empty() {
        panic!("{}", messages.join("\n\n"));
    }
}
