use crate::common::TestSuiteError;
use crate::common::manifest::ValidateManifest;
use shacl::error::IRError;
use shacl::validator::processor::{DataValidation, ShaclProcessor};
use shacl::validator::{ShaclConfig, ShaclValidationMode};
use std::path::Path;

/// Runs a `sht:Validate` manifest with both [`Native`](ShaclValidationMode::Native) and
/// [`Sparql`](ShaclValidationMode::Sparql)
pub(crate) fn validate_runner(path: String) -> Result<(), TestSuiteError> {
    test(path.clone(), ShaclValidationMode::Native)?;
    test(path, ShaclValidationMode::Sparql)
}

fn test(path: String, mode: ShaclValidationMode) -> Result<(), TestSuiteError> {
    let mut manifest = ValidateManifest::new(Path::new(&path))?;
    let tests = manifest.collect_tests()?;

    for test in tests {
        let mut validator: DataValidation = test.data.into();
        let test_shapes = test
            .shapes
            .try_into()
            .map_err(|e: IRError| TestSuiteError::TestShapesCompilation(e.to_string()))?;

        // The test suite compares reports without nested results (`sh:detail`).
        let report = validator
            .validate(&test_shapes, &mode, &ShaclConfig::default())
            .map_err(|e| TestSuiteError::Validation(e.to_string()))?
            .without_details();

        if report != test.report.clone().without_details() {
            println!("❌ Test failed");
            println!("Expected report:\n{:#?}", test.report.results());
            println!("Actual report:\n{:#?}", report.results());
            return Err(TestSuiteError::NotEquals);
        }
    }

    Ok(())
}
