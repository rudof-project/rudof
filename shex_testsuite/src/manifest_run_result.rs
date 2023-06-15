use crate::manifest_error::ManifestError;

#[derive(Debug)]
pub struct ManifestRunResult {
    pub passed: Vec<String>,
    pub skipped: Vec<String>,
    pub failed: Vec<ManifestError>,
}

impl ManifestRunResult {
    pub fn new() -> ManifestRunResult {
        ManifestRunResult {
            passed: Vec::new(),
            skipped: Vec::new(),
            failed: Vec::new(),
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

    pub fn add_failed(&mut self, err: ManifestError) -> &Self {
        self.failed.push(err);
        self
    }
}
