use crate::TestSuiteError;
use crate::manifest::{NodeExprManifest, ValidateManifest};
use crate::runner::validate::mismatch;
use shacl::validator::ShaclValidationMode;
use std::path::Path;

pub enum TestOutcome {
    Passed,
    Failed(String),
}

/// Runs every `sht:Validate` entry of the manifest at `path` under both validation modes
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

/// Runs every `sht:EvalNodeExpr` entry of the manifest at `path`
pub fn node_expr_outcome(path: &Path) -> TestOutcome {
    node_expr_outcome_inner(path).unwrap_or_else(|e| TestOutcome::Failed(e.to_string()))
}

fn node_expr_outcome_inner(path: &Path) -> Result<TestOutcome, TestSuiteError> {
    let mut manifest = NodeExprManifest::new(path)?;
    let tests = manifest.collect_tests()?;

    for test in tests {
        let actual = manifest.evaluate(&test)?;

        let equals = if test.ignore_order {
            let mut actual = actual.iter().map(ToString::to_string).collect::<Vec<_>>();
            let mut expected = test.expected.iter().map(ToString::to_string).collect::<Vec<_>>();
            actual.sort();
            expected.sort();
            actual == expected
        } else {
            actual == test.expected
        };

        if !equals {
            return Ok(TestOutcome::Failed(format!(
                "{}: expected {:#?}, got {:#?}",
                test.label, test.expected, actual
            )));
        }
    }

    Ok(TestOutcome::Passed)
}
