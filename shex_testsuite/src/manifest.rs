use crate::manifest_error::ManifestError;
use crate::manifest_run_mode::ManifestRunMode;
use crate::manifest_run_result::ManifestRunResult;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::Path;

pub trait Manifest {
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool;

    fn entry_names(&self) -> Vec<String>;

    fn run_entry(&self, name: &str, base: &Path) -> Result<(), Box<ManifestError>>;

    fn should_run_entry_name(
        &self,
        single_entries: &Option<Vec<String>>,
        entry_name: &String,
    ) -> bool {
        match single_entries {
            None => true,
            Some(es) => es.contains(entry_name),
        }
    }

    fn run(
        &self,
        base: &Path,
        mode: ManifestRunMode,
        excluded_entries: Vec<String>,
        single_entries: Option<Vec<String>>,
        _single_traits: Option<Vec<String>>,
    ) -> ManifestRunResult {
        let mut result: ManifestRunResult = ManifestRunResult::new();
        for entry_name in &self.entry_names() {
            if excluded_entries.contains(entry_name) {
                result.add_skipped(entry_name.to_string());
            } else if Self::should_run_entry_name(self, &single_entries, entry_name) {
                let safe_result =
                    catch_unwind(AssertUnwindSafe(move || self.run_entry(entry_name, base)));
                match safe_result {
                    Ok(Ok(())) => {
                        result.add_passed(entry_name.to_string());
                    }
                    Ok(Err(e)) => {
                        result.add_failed(entry_name.to_string(), *e);
                        if mode == ManifestRunMode::FailFirstError {
                            return result;
                        }
                    }
                    Err(err) => {
                        result.add_panicked(entry_name.to_string(), err);
                        if mode == ManifestRunMode::FailFirstError {
                            return result;
                        }
                    }
                }
            }
        }
        result
    }
}
