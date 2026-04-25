use crate::IriS;
use crate::error::IriSError;
use std::path::Path;

impl TryFrom<&Path> for IriS {
    type Error = IriSError;

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        Err(IriSError::ConvertingPathToIri {
            path: value.to_string_lossy().to_string(),
            error: String::from("Converting path to IRI is not supported in WASM target"),
        })
    }
}
