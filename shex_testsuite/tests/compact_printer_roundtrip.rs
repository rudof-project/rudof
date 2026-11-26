#[cfg(not(target_family = "wasm"))]
use shex_testsuite::{manifest::Manifest, manifest_schemas::ManifestSchemas};
#[cfg(not(target_family = "wasm"))]
use std::panic::{AssertUnwindSafe, catch_unwind};
#[cfg(not(target_family = "wasm"))]
use std::{collections::BTreeSet, fs, path::Path};

#[cfg(not(target_family = "wasm"))]
const MANIFEST: &str = "shexTest/schemas/manifest.jsonld";
#[cfg(not(target_family = "wasm"))]
const BASELINE: &str = "tests/baseline_pretty_print_failing.txt";

/// Regression test for the ShEx compact printer, using the ShExJ schemas from
/// the ShEx test suite.
///
/// For every schema in `shexTest/schemas/manifest.jsonld`, this:
/// 1. Loads the ShExJ (JSON) schema.
/// 2. Pretty prints it with the ShEx compact printer.
/// 3. Parses the pretty printed ShExC text back into a schema.
/// 4. Checks that the re-parsed schema is equivalent to the original one.
///
/// Like `validation_regression`, this compares the current set of failing
/// entries against a stored baseline so that improvements and regressions in
/// the compact printer are both visible without having to fix every known
/// failure at once.
#[cfg(not(target_family = "wasm"))]
#[test]
fn compact_printer_roundtrip_regression() {
    let pkg = Path::new(env!("CARGO_MANIFEST_DIR"));
    let manifest_path = pkg.join(MANIFEST);
    let baseline_path = pkg.join(BASELINE);

    let manifest_str = fs::read_to_string(&manifest_path).expect("Failed to read manifest.jsonld");
    let manifest: ManifestSchemas = serde_json::from_str(&manifest_str).expect("Failed to parse manifest");

    let base = manifest_path.parent().unwrap();

    let mut current_failing: BTreeSet<String> = BTreeSet::new();
    for name in manifest.entry_names() {
        let result = catch_unwind(AssertUnwindSafe(|| {
            manifest.run_pretty_print_roundtrip_entry(&name, base)
        }));
        match result {
            Ok(Ok(())) => {},
            Ok(Err(_)) | Err(_) => {
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
