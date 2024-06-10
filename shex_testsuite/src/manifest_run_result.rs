use std::any::Any;

use crate::manifest_error::ManifestError;

#[derive(Debug)]
pub struct ManifestRunResult {
    pub passed: Vec<String>,
    pub skipped: Vec<String>,
    pub failed: Vec<(String, ManifestError)>,
    pub panicked: Vec<(String, Box<dyn Any + Send + 'static>)>,
}

impl Default for ManifestRunResult {
    fn default() -> Self {
        Self::new()
    }
}

impl ManifestRunResult {
    pub fn new() -> ManifestRunResult {
        ManifestRunResult {
            passed: Vec::new(),
            skipped: Vec::new(),
            failed: Vec::new(),
            panicked: Vec::new(),
        }
    }

    pub fn add_passed(&mut self, name: String) -> &Self {
        self.passed.push(name);
        self
    }

    pub fn add_skipped(&mut self, name: String) -> &Self {
        self.skipped.push(name);
        self
    }

    pub fn add_failed(&mut self, name: String, err: ManifestError) -> &Self {
        self.failed.push((name, err));
        self
    }

    pub fn add_panicked(&mut self, name: String, err: Box<dyn Any + Send + 'static>) -> &Self {
        self.panicked.push((name, err));
        self
    }
}
