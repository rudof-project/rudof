use std::path::Path;

use serde_derive::{Deserialize, Serialize};
use shex_validation::ValidatorConfig;
use srdf::RdfDataConfig;

use crate::RudofError;

/// `rudof_config` describes the configuration of Rudof
///
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone, Default)]
pub struct RudofConfig {
    rdf_data_config: Option<RdfDataConfig>,
    shex_validator_config: Option<ValidatorConfig>,
}

impl RudofConfig {
    pub fn new() -> RudofConfig {
        Self::default()
    }

    pub fn with_rdf_data_config(mut self, rdf_data_config: RdfDataConfig) -> Self {
        self.rdf_data_config = Some(rdf_data_config);
        self
    }

    pub fn with_shex_validator_config(mut self, shex_validator_config: ValidatorConfig) -> Self {
        self.shex_validator_config = Some(shex_validator_config);
        self
    }

    /// Obtain a DCTapConfig from a path file in YAML
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<RudofConfig, RudofError> {
        let path_name = path.as_ref().display().to_string();
        let f = std::fs::File::open(path).map_err(|e| RudofError::RudofConfigFromPathError {
            path: path_name.clone(),
            error: e,
        })?;
        let config: RudofConfig =
            serde_yml::from_reader(f).map_err(|e| RudofError::RudofConfigYamlError {
                path: path_name.clone(),
                error: e,
            })?;
        Ok(config)
    }

    pub fn validator_config(&self) -> ValidatorConfig {
        match &self.shex_validator_config {
            None => ValidatorConfig::default(),
            Some(cfg) => cfg.clone(),
        }
    }
}
