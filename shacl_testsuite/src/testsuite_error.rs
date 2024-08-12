use shacl_validation::validate_error::ValidateError;
use thiserror::Error;

use crate::manifest_error::ManifestError;

#[derive(Error, Debug)]
pub enum TestSuiteError {
    #[error("Error during the parsing of the Manifest")]
    Manifest(#[from] ManifestError),
    #[error("Error during the validation")]
    Validate(#[from] ValidateError),
}
