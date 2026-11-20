use crate::TestSuiteError;
use crate::manifest::{TestInstance, ValidateManifest};
use shacl::error::IRError;
use shacl::validator::processor::{DataValidation, ShaclProcessor};
use shacl::validator::{ShaclConfig, ShaclValidationMode};
use sparql_service::RdfData;
use std::path::Path;

/// Runs a `sht:Validate` manifest with both [`Native`](ShaclValidationMode::Native) and
/// [`Sparql`](ShaclValidationMode::Sparql)
pub fn validate_runner(path: String) -> Result<(), TestSuiteError> {
    test(path.clone(), ShaclValidationMode::Native)?;
    test(path, ShaclValidationMode::Sparql)
}

fn test(path: String, mode: ShaclValidationMode) -> Result<(), TestSuiteError> {
    let mut manifest = ValidateManifest::new(Path::new(&path))?;
    let tests = manifest.collect_tests()?;

    for test in tests {
        if let Some(message) = mismatch(&test, &mode)? {
            println!("❌ Test failed");
            println!("{message}");
            return Err(TestSuiteError::NotEquals);
        }
    }

    Ok(())
}

/// Runs one test instance under one mode
pub(crate) fn mismatch(
    test: &TestInstance<RdfData>,
    mode: &ShaclValidationMode,
) -> Result<Option<String>, TestSuiteError> {
    let mut validator: DataValidation = test.data.clone().into();
    let test_shapes = test
        .shapes
        .clone()
        .try_into()
        .map_err(|e: IRError| TestSuiteError::TestShapesCompilation(e.to_string()))?;

    let report = validator
        .validate(&test_shapes, mode, &ShaclConfig::default())
        .map_err(|e| TestSuiteError::Validation(e.to_string()))?
        .without_details();

    let expected = test.report.clone().without_details();

    if report != expected {
        Ok(Some(format!(
            "Expected report:\n{:#?}\nActual report:\n{:#?}",
            expected.results(),
            report.results()
        )))
    } else {
        Ok(None)
    }
}
