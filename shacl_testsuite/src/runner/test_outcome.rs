use crate::TestSuiteError;
use crate::manifest::ValidateManifest;
use crate::runner::validate::mismatch;
use shacl::validator::ShaclValidationMode;
use std::path::Path;

pub enum TestOutcome {
    Passed,
    Failed(String),
}

/// Runs every entry of the manifest at `path` under both validation modes
pub fn run_outcome(path: &Path) -> TestOutcome {
    run_outcome_inner(path).unwrap_or_else(|e| TestOutcome::Failed(e.to_string()))
}

fn run_outcome_inner(path: &Path) -> Result<TestOutcome, TestSuiteError> {
    let mut manifest = ValidateManifest::new(path)?;
    let tests = manifest.collect_tests()?;

    for test in tests {
        for mode in [ShaclValidationMode::Native, ShaclValidationMode::Sparql] {
            if let Some(message) = mismatch(&test, &mode)? {
                return Ok(TestOutcome::Failed(message));
            }
        }
    }

    Ok(TestOutcome::Passed)
}
