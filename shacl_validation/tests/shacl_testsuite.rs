mod common;

use crate::common::manifest::Manifest;
use common::testsuite_error::TestSuiteError;
use shacl_ir::compiled_shacl_error::CompiledShaclError;
use shacl_validation::shacl_processor::RdfDataValidation;
use shacl_validation::shacl_processor::ShaclProcessor;
use shacl_validation::shacl_processor::ShaclValidationMode;
use std::path::Path;

mod core;

fn test(
    path: String,
    mode: ShaclValidationMode,
    // subsetting: Subsetting,
) -> Result<(), Box<TestSuiteError>> {
    let mut manifest = Manifest::new(Path::new(&path))?;
    let tests = manifest.collect_tests()?;

    for test in tests {
        let validator = RdfDataValidation::from_rdf_data(test.data, mode);
        let test_shapes = test
            .shapes
            .try_into()
            .map_err(
                |e: Box<CompiledShaclError>| TestSuiteError::TestShapesCompilation {
                    error: (*e).to_string(),
                },
            )?;
        let report = validator
            .validate(&test_shapes)
            .map_err(|e| TestSuiteError::Validation {
                error: e.to_string(),
            })?;
        if report != test.report {
            return Err(Box::new(TestSuiteError::NotEquals));
        }
    }

    Ok(())
}
