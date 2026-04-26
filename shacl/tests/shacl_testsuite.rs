#[cfg(not(target_family = "wasm"))]
use crate::common::{Manifest, TestSuiteError};
#[cfg(not(target_family = "wasm"))]
use shacl::error::IRError;
#[cfg(not(target_family = "wasm"))]
use shacl::validator::ShaclValidationMode;
#[cfg(not(target_family = "wasm"))]
use shacl::validator::processor::{DataValidation, ShaclProcessor};
#[cfg(not(target_family = "wasm"))]
use std::path::Path;

#[cfg(not(target_family = "wasm"))]
mod common;
#[cfg(not(target_family = "wasm"))]
mod core;

#[cfg(not(target_family = "wasm"))]
fn test(path: String, mode: ShaclValidationMode) -> Result<(), TestSuiteError> {
    println!("Running test: {path}");
    let mut manifest = Manifest::new(Path::new(&path))?;
    println!("Manifest loaded successfully");
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
