use std::path::Path;

use serde_derive::{Deserialize, Serialize};

use crate::{ValidatorError, MAX_STEPS};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]

pub struct ValidatorConfig {
    max_steps: usize,
}

impl Default for ValidatorConfig {
    fn default() -> Self {
        Self {
            max_steps: MAX_STEPS,
        }
    }
}

impl ValidatorConfig {
    /// Obtain a `ValidatorConfig` from a path file in YAML format
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<ValidatorConfig, ValidatorError> {
        let path_name = path.as_ref().display().to_string();
        let f = std::fs::File::open(path).map_err(|e| {
            ValidatorError::ValidatorConfigFromPathError {
                path: path_name.clone(),
                error: e.to_string(),
            }
        })?;
        let config: ValidatorConfig =
            serde_yml::from_reader(f).map_err(|e| ValidatorError::ValidatorConfigYamlError {
                path: path_name.clone(),
                error: e.to_string(),
            })?;
        Ok(config)
    }

    pub fn set_max_steps(&mut self, max_steps: usize) {
        self.max_steps = max_steps;
    }

    pub fn max_steps(&self) -> usize {
        self.max_steps
    }
}
