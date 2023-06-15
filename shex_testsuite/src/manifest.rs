use crate::manifest_error::ManifestError;
use crate::manifest_run_mode::ManifestRunMode;
use crate::manifest_run_result::ManifestRunResult;
use std::path::Path;

pub trait Manifest {
    fn len(&self) -> usize;

    fn entry_names(&self) -> Vec<String>;
    
    fn run_entry(&self, name: &str, base: &Path, debug: u8) -> Result<(), ManifestError>;

    fn should_run(&self, single_entries: &Option<Vec<String>>, entry_name: &String) -> bool {
        match single_entries {
            None => true,
            Some(es) => es.contains(entry_name),
        }
    }

    fn run(
        &self,
        base: &Path,
        debug: u8,
        mode: ManifestRunMode,
        excluded_entries: Vec<String>,
        single_entries: Option<Vec<String>>,
    ) -> ManifestRunResult {
        let mut result: ManifestRunResult = ManifestRunResult::new();
        for entry_name in &self.entry_names() {
            if excluded_entries.contains(entry_name) {
                result.add_skipped(entry_name.to_string());
                ()
            } else if Self::should_run(&self, &single_entries, entry_name) {
                match self.run_entry(entry_name, base, debug) {
                    Ok(_) => {
                        result.add_passed(entry_name.to_string());
                        ()
                    }
                    Err(e) => {
                        result.add_failed(e);
                        match mode {
                            ManifestRunMode::FailFirstError => return result,
                            _ => (),
                        }
                    }
                }
            } else {
                ()
            }
        }
        result
    }
}
