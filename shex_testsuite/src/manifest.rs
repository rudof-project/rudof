use crate::manifest_error::ManifestError;
use crate::manifest_run_mode::ManifestRunMode;
use std::path::Path;

pub trait Manifest {
    fn len(&self) -> usize;

    fn entry_names(&self) -> Vec<String>;

    fn run_entry(&self, name: &str, base: &Path, debug: u8) -> Result<(), ManifestError>;

    fn run(
        &self,
        base: &Path,
        debug: u8,
        mode: ManifestRunMode,
    ) -> Result<u32, Vec<ManifestError>> {
        let mut count_passed = 0;
        let mut failed: Vec<ManifestError> = Vec::new();
        for entry_name in &self.entry_names() {
            match self.run_entry(entry_name, base, debug) {
                Ok(_) => count_passed += 1,
                Err(e) => {
                    failed.push(e);
                    match mode {
                        ManifestRunMode::FailFirstError => return Err(failed),
                        _ => (),
                    }
                }
            }
        }
        if failed.is_empty() {
            Ok(count_passed)
        } else {
            Err(failed)
        }
    }
}
