use crate::manifest_error::ManifestError;
use crate::manifest_schemas::SchemasEntry;
use iri_s::IriS;
use std::path::Path;

pub(super) fn path_to_iri(_: &Path) -> Result<IriS, Box<ManifestError>> {
    Err(Box::new(ManifestError::WASMError(
        "Unable to convert path to IRI".to_string(),
    )))
}

impl SchemasEntry {
    pub(super) fn run(&self, _: &Path) -> Result<(), Box<ManifestError>> {
        Err(Box::new(ManifestError::WASMError(
            "Url cannot be generated from file path".to_string(),
        )))
    }
}
