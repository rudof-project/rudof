use std::path::Path;

use serde_derive::{Deserialize, Serialize};
use srdf::RdfDataConfig;

use crate::{ValidatorError, MAX_STEPS};

/// This struct can be used to customize the behavour of ShEx validators
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]

pub struct ValidatorConfig {
    /// Maximum numbers of validation steps
    pub max_steps: usize,

    /// Configuration of RDF data readers
    pub data_config: Option<RdfDataConfig>,
}

impl Default for ValidatorConfig {
    fn default() -> Self {
        Self {
            max_steps: MAX_STEPS,
            data_config: Some(RdfDataConfig::default()),
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
