use std::path::Path;
use shacl::error::IRError;
use shacl::validator::processor::{DataValidation, ShaclProcessor};
use shacl::validator::{ShaclConfig, ShaclValidationMode};
use crate::common::manifest::ValidateManifest;
use crate::common::TestSuiteError;

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

        let report = validator
            .validate(&test_shapes, &mode, &ShaclConfig::default())
            .map_err(|e| TestSuiteError::Validation(e.to_string()))?;

        if report != test.report {
            println!("❌ Test failed");
            println!("Expected report:\n{:#?}", test.report.results());
            println!("Actual report:\n{:#?}", report.results());
            return Err(TestSuiteError::NotEquals);
        }
    }

    Ok(())
}
