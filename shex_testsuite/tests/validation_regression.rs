use shex_testsuite::{
    config::Config, manifest::Manifest, manifest_mode::ManifestShExSyntaxMode, manifest_run_mode::ManifestRunMode,
    manifest_validation::ManifestValidation,
};
use std::{collections::BTreeSet, fs, path::Path};

const MANIFEST: &str = "shexTest/validation/manifest.jsonld";
const CONFIG: &str = "config.toml";
const BASELINE: &str = "tests/baseline_failing.txt";

/// Regression test for the ShEx validation test suite.
///
/// Compares the set of currently failing tests against a stored baseline:
/// - If new tests are now failing (regressions), the test fails and lists them.
/// - If previously failing tests now pass (improvements), the baseline is updated
///   automatically and the test fails asking you to commit the updated baseline.
/// - If nothing changed, the test passes.
///
/// To initialise the baseline for the first time, simply run the test — it will
/// create `tests/baseline_failing.txt` and then fail asking you to commit the file.
#[test]
fn validation_regression() {
    let pkg = Path::new(env!("CARGO_MANIFEST_DIR"));
    let manifest_path = pkg.join(MANIFEST);
    let config_path = pkg.join(CONFIG);
    let baseline_path = pkg.join(BASELINE);

    let config = Config::from_file(config_path.to_str().unwrap()).expect("Failed to load config.toml");

    let manifest_str = fs::read_to_string(&manifest_path).expect("Failed to read manifest.jsonld");
    let manifest: ManifestValidation = serde_json::from_str(&manifest_str).expect("Failed to parse manifest");

    let base = manifest_path.parent().unwrap();

    let result = manifest.run(
        base,
        ManifestRunMode::CollectErrors,
        config.excluded_entries,
        None,
        None,
        ManifestShExSyntaxMode::ShExC,
    );

    let current_failing: BTreeSet<String> = result.failed.iter().map(|(name, _)| name.clone()).collect();

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
