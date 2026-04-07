use crate::manifest_error::ManifestError;
use crate::manifest_mode::ManifestShExSyntaxMode;
use crate::manifest_run_mode::ManifestRunMode;
use crate::manifest_run_result::ManifestRunResult;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::Path;

pub trait Manifest {
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool;

    fn entry_names(&self) -> Vec<String>;

    fn has_traits(&self, name: &str) -> Result<Vec<String>, Box<ManifestError>>;

    fn run_entry(
        &self,
        name: &str,
        base: &Path,
        manifest_shex_syntax_mode: ManifestShExSyntaxMode,
    ) -> Result<(), Box<ManifestError>>;

    fn should_run_entry_name(
        &self,
        single_entries: &Option<Vec<String>>,
        entry_name: &String,
        allowed_traits: &Option<Vec<String>>,
    ) -> Result<bool, Box<ManifestError>> {
        let check_entry_name = match single_entries {
            None => true,
            Some(es) => es.contains(entry_name),
        };
        let check_traits = match allowed_traits {
            None => Ok::<bool, Box<ManifestError>>(true),
            Some(ts) => {
                let traits = self.has_traits(entry_name)?;
                for t in traits {
                    if ts.contains(&t) {
                        return Ok(true);
                    }
                }
                Ok(false)
            },
        }?;
        Ok(check_entry_name && check_traits)
    }

    fn conditional_run(
        &self,
        entry_name: &String,
        base: &Path,
        single_entries: &Option<Vec<String>>,
        allowed_traits: &Option<Vec<String>>,
        manifest_shex_syntax_mode: ManifestShExSyntaxMode,
    ) -> Result<bool, Box<ManifestError>> {
        let condition = Self::should_run_entry_name(self, single_entries, entry_name, allowed_traits)?;
        if condition {
            self.run_entry(entry_name, base, manifest_shex_syntax_mode)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn run(
        &self,
        base: &Path,
        mode: ManifestRunMode,
        excluded_entries: Vec<String>,
        single_entries: Option<Vec<String>>,
        allowed_traits: Option<Vec<String>>,
        manifest_shex_syntax_mode: ManifestShExSyntaxMode,
    ) -> ManifestRunResult {
        /*trace!(
            "Running manifest with mode: {mode:?}, excluded_entries: {:?}, single_entries: {:?}, allowed_traits: {:?}",
            excluded_entries, single_entries, allowed_traits
        );*/
        let mut result: ManifestRunResult = ManifestRunResult::new();
        for entry_name in &self.entry_names() {
            let entry_traits = self.has_traits(entry_name).unwrap_or_default();
            if excluded_entries.contains(entry_name) {
                result.add_skipped(entry_name.to_string(), entry_traits);
            } else {
                // We clone these here to avoid borrowing issues in the closure passed to catch_unwind
                let entries = single_entries.clone();
                let traits = allowed_traits.clone();
                // We use catch_unwind to catch any panics that occur during the execution of the entry, and treat them as test failures
                let safe_result = catch_unwind(AssertUnwindSafe(move || {
                    self.conditional_run(entry_name, base, &entries, &traits, manifest_shex_syntax_mode)
                }));
                match safe_result {
                    Ok(Ok(true)) => {
                        result.add_passed(entry_name.to_string(), entry_traits);
                    },
                    Ok(Ok(false)) => {
                        result.add_skipped(entry_name.to_string(), entry_traits);
                    },
                    Ok(Err(e)) => {
                        result.add_failed(entry_name.to_string(), *e, entry_traits);
                        if mode == ManifestRunMode::FailFirstError {
                            return result;
                        }
                    },
                    Err(err) => {
                        result.add_panicked(entry_name.to_string(), err, entry_traits);
                        if mode == ManifestRunMode::FailFirstError {
                            return result;
                        }
                    },
                }
            }
        }
        result
    }
}
