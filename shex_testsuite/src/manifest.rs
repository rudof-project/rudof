use std::path::Path;

use crate::manifest_error::ManifestError;

pub trait Manifest {
    fn len(&self) -> usize;
    fn entry_names(&self) -> Vec<String>;
    fn run_entry(&self, name: &str, base: &Path, debug: u8) -> Result<(), ManifestError>;
    fn run(&self, base: &Path, debug: u8) -> Result<(), ManifestError> {
        for entry_name in &self.entry_names() {
            self.run_entry(entry_name, base, debug)?
        }
        Ok(())
    }
}
