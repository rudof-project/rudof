use crate::common::{Manifest, TestSuiteError};
use shacl::error::IRError;
use shacl::validator::ShaclValidationMode;
use shacl::validator::processor::{DataValidation, ShaclProcessor};
use std::path::Path;

mod common;
mod core;

fn test(path: String, mode: ShaclValidationMode) -> Result<(), TestSuiteError> {
    let mut manifest = Manifest::new(Path::new(&path))?;
    let tests = manifest.collect_tests()?;

    for test in tests {
        let mut validator: DataValidation = test.data.into();
        let test_shapes = test
            .shapes
            .try_into()
            .map_err(|e: IRError| TestSuiteError::TestShapesCompilation(e.to_string()))?;

        let report = validator
            .validate(&test_shapes, &mode)
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
