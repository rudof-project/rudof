use rdf::rdf_core::RdfDataConfig;
use serde::{Deserialize, Serialize};
use shex_ast::shapemap::ShapemapConfig;
use std::io::Read;
use std::path::Path;

use crate::{MAX_STEPS, ShExConfig, ValidatorError};

const DEFAULT_WIDTH: usize = 80;

/// This struct can be used to customize the behavour of ShEx validators
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]

pub struct ValidatorConfig {
    /// Maximum numbers of validation steps
    pub max_steps: usize,

    /// Configuration of RDF data readers
    pub rdf_data: Option<RdfDataConfig>,

    /// Configuration of ShEx schemas
    pub shex: Option<ShExConfig>,

    /// Configuration of Shapemaps
    pub shapemap: Option<ShapemapConfig>,

    /// Whether to check the negation requirement (default: true)
    pub check_negation_requirement: Option<bool>,

    /// Width for pretty printing
    pub width: Option<usize>,
}

impl Default for ValidatorConfig {
    fn default() -> Self {
        Self {
            max_steps: MAX_STEPS,
            rdf_data: Some(RdfDataConfig::default()),
            shex: Some(ShExConfig::default()),
            shapemap: Some(ShapemapConfig::default()),
            check_negation_requirement: Some(true),
            width: Some(80),
        }
    }
}

impl ValidatorConfig {
    /// Obtain a `ValidatorConfig` from a path file in TOML format
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<ValidatorConfig, ValidatorError> {
        let path_name = path.as_ref().display().to_string();
        let mut f = std::fs::File::open(path).map_err(|e| ValidatorError::ValidatorConfigFromPathError {
            path: path_name.clone(),
            error: e.to_string(),
        })?;
        let mut s = String::new();
        f.read_to_string(&mut s)
            .map_err(|e| ValidatorError::ValidatorConfigFromPathError {
                path: path_name.clone(),
                error: e.to_string(),
            })?;

        let config: ValidatorConfig =
            toml::from_str(s.as_str()).map_err(|e| ValidatorError::ValidatorConfigTomlError {
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

    pub fn rdf_data_config(&self) -> RdfDataConfig {
        match &self.rdf_data {
            None => RdfDataConfig::default(),
            Some(sc) => sc.clone(),
        }
    }

    pub fn shex_config(&self) -> ShExConfig {
        match &self.shex {
            None => ShExConfig::default(),
            Some(sc) => sc.clone(),
        }
    }

    pub fn shapemap_config(&self) -> ShapemapConfig {
        match &self.shapemap {
            None => ShapemapConfig::default(),
            Some(sc) => sc.clone(),
        }
    }

    pub fn width(&self) -> usize {
        match self.width {
            None => DEFAULT_WIDTH,
            Some(w) => w,
        }
    }
}
