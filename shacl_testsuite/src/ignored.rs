use crate::TestSuiteError;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// An `ignored.toml` entry
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IgnoredEntry {
    pub test: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue: Option<u32>,
}

/// The known-failing entries of a test suite, keyed by test IRI
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct IgnoredList {
    #[serde(default)]
    pub ignored: Vec<IgnoredEntry>,
}

impl IgnoredList {
    pub fn load(path: &Path) -> Result<Self, TestSuiteError> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let contents = fs::read_to_string(path)?;
        toml::from_str(&contents).map_err(TestSuiteError::IgnoredTomlRead)
    }

    pub fn save(&self, path: &Path) -> Result<(), TestSuiteError> {
        let mut sorted = self.ignored.clone();
        sorted.sort_by(|a, b| a.test.cmp(&b.test));
        let contents = toml::to_string_pretty(&Self { ignored: sorted }).map_err(TestSuiteError::IgnoredTomlWrite)?;
        fs::write(path, contents)?;
        Ok(())
    }

    pub fn is_ignored(&self, test: &str) -> bool {
        self.ignored.iter().any(|entry| entry.test == test)
    }

    pub fn reason_for(&self, test: &str) -> Option<&str> {
        self.ignored
            .iter()
            .find(|entry| entry.test == test)
            .and_then(|entry| entry.reason.as_deref())
    }
}
