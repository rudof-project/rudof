use rudof_rdf::rdf_core::RdfDataConfig;
use serde::{Deserialize, Deserializer, Serialize};
use shex_ast::shapemap::ShapemapConfig;
use std::io::Read;
use std::path::Path;

use crate::{MAX_STEPS, ShExConfig, ValidatorError};

/// This struct can be used to customize the behavour of ShEx validators
#[derive(Debug, PartialEq, Clone)]
pub struct ValidatorConfig {
    /// Maximum numbers of validation steps
    pub(crate) max_steps: Option<usize>,

    /// Configuration of RDF data readers
    pub(crate) rdf_data: RdfDataConfig,
    rdf_data_needs_fixup: bool,

    /// Configuration of ShEx schemas
    pub(crate) shex: ShExConfig,
    shex_needs_fixup: bool,

    /// Configuration of Shapemaps
    pub(crate) shapemap: ShapemapConfig,

    /// Whether to check the negation requirement (default: true)
    pub(crate) check_negation_requirement: bool,

    /// Width for pretty printing
    // TODO - This should be in rudof_lib
    pub(crate) width: usize,
}

impl ValidatorConfig {
    pub fn new() -> Self {
        Self {
            max_steps: Self::default_max_steps(),
            shex_needs_fixup: false,
            width: Self::default_width(),
            rdf_data: Self::default_rdf_data(),
            shex: Self::default_shex(),
            shapemap: Self::default_shapemap(),
            rdf_data_needs_fixup: false,
            check_negation_requirement: Self::default_check_negation_requirement(),
        }
    }

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

    pub fn with_max_steps(mut self, steps: Option<usize>) -> Self {
        self.max_steps = steps;
        self
    }

    pub fn with_rdf_data(mut self, cfg: RdfDataConfig) -> Self {
        self.rdf_data = cfg;
        self
    }

    pub fn with_shex(mut self, cfg: ShExConfig) -> Self {
        self.shex = cfg;
        self
    }

    pub fn with_shapemap(mut self, cfg: ShapemapConfig) -> Self {
        self.shapemap = cfg;
        self
    }

    pub fn with_check_negation_requirement(mut self, flag: bool) -> Self {
        self.check_negation_requirement = flag;
        self
    }

    pub fn with_width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }
}

impl ValidatorConfig {
    pub fn max_steps(&self) -> Option<usize> {
        self.max_steps
    }

    pub fn rdf_data(&self) -> &RdfDataConfig {
        &self.rdf_data
    }

    pub fn shex(&self) -> &ShExConfig {
        &self.shex
    }

    pub fn shapemap(&self) -> &ShapemapConfig {
        &self.shapemap
    }

    pub fn check_negation_requirement(&self) -> bool {
        self.check_negation_requirement
    }

    pub fn width(&self) -> usize {
        self.width
    }
}

/// Serde stuff
#[allow(dead_code)]
#[cfg_attr(rustfmt, rustfmt_skip)]
impl ValidatorConfig {
    #[inline] fn default_max_steps() -> Option<usize> { None }
    #[inline] fn default_rdf_data() -> RdfDataConfig { RdfDataConfig::new() }
    #[inline] fn default_shex() -> ShExConfig { ShExConfig::new() }
    #[inline] fn default_shapemap() -> ShapemapConfig { ShapemapConfig::new() }
    #[inline] fn default_check_negation_requirement() -> bool { true }
    #[inline] fn default_width() -> usize { 80 }

    pub fn fixup(&mut self, rdf: RdfDataConfig, shex: ShExConfig) {
        if self.shex_needs_fixup {
            self.shex_needs_fixup = false;
            self.shex = shex;
        }

        if self.rdf_data_needs_fixup {
            self.rdf_data_needs_fixup = false;
            self.rdf_data = rdf;
        }
    }
}

impl Default for ValidatorConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ValidatorConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        #[derive(Deserialize)]
        struct Raw {
            #[serde(rename = "max_steps", default = "ValidatorConfig::default_max_steps")]
            max_steps: Option<usize>,
            #[serde(rename = "rdf", default)]
            rdf_data: Option<RdfDataConfig>,
            #[serde(rename = "shex", default)]
            shex: Option<ShExConfig>,
            #[serde(rename = "shapemap", default = "ValidatorConfig::default_shapemap")]
            shapemap: ShapemapConfig,
            #[serde(rename = "check_negation", default = "ValidatorConfig::default_check_negation_requirement")]
            check_negation_requirement: bool,
            #[serde(rename = "width", default = "ValidatorConfig::default_width")]
            width: usize,
        }

        let raw = Raw::deserialize(deserializer)?;

        Ok(Self {
            max_steps: raw.max_steps,
            rdf_data_needs_fixup: raw.rdf_data.is_none(),
            rdf_data: raw.rdf_data.unwrap_or(Self::default_rdf_data()),
            shex_needs_fixup: raw.shex.is_none(),
            shex: raw.shex.unwrap_or(Self::default_shex()),
            width: raw.width,
            shapemap: raw.shapemap,
            check_negation_requirement: raw.check_negation_requirement,
        })
    }
}
