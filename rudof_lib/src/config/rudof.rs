use std::fs::File;
use std::io::Read;
use std::path::Path;
use crate::errors::RudofConfigError;
use dctap::TapConfig;
use rudof_rdf::rdf_core::RdfDataConfig;
use serde::{Deserialize, Deserializer};
use shapes_comparator::ComparatorConfig;
use shapes_converter::{ShEx2HtmlConfig, ShEx2SparqlConfig, ShEx2UmlConfig, Shacl2ShExConfig, Tap2ShExConfig};
use shex_validation::{ShExConfig, ValidatorConfig};
use sparql_service::ServiceConfig;
use std::str::FromStr;
use semver::Version;
use crate::config::CommonConfig;

/// Main configuration structure for Rudof.
///
/// This structure encapsulates all configuration options for Rudof operations,
/// including RDF data handling, schema validation (ShEx and SHACL), conversions,
/// and visualization settings.
#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct RudofConfig {
    #[serde(rename = "version", default = "RudofConfig::default_version")]
    pub(crate) version: Option<Version>,

    #[serde(flatten, default = "RudofConfig::default_common_config")]
    pub(crate) common: CommonConfig,

    // ---
    #[serde(rename = "rdf", default = "RudofConfig::default_rdf_data_config")]
    pub(crate) rdf_data: RdfDataConfig,
    #[serde(rename = "shex", default = "RudofConfig::default_shex_config")]
    pub(crate) shex: ShExConfig,
    #[serde(rename = "shex_validator", default = "RudofConfig::default_shex_validator_config")]
    pub(crate) shex_validator: ValidatorConfig,
    #[serde(rename = "shex2uml", default = "RudofConfig::default_shex2uml_config")]
    pub(crate) shex2uml: ShEx2UmlConfig,
    #[serde(rename = "shex2html", default = "RudofConfig::default_shex2html_config")]
    pub(crate) shex2html: ShEx2HtmlConfig,
    #[serde(rename = "shacl2shex", default = "RudofConfig::default_shacl2shex_config")]
    pub(crate) shacl2shex: Shacl2ShExConfig,
    #[serde(rename = "tap", default = "RudofConfig::default_tap_config")]
    pub(crate) tap: TapConfig,
    #[serde(rename = "tap2shex", default = "RudofConfig::default_tap2shex_config")]
    pub(crate) tap2shex: Tap2ShExConfig,
    #[serde(rename = "shex2sparql", default = "RudofConfig::default_shex2sparql_config")]
    pub(crate) shex2sparql: ShEx2SparqlConfig,
    #[serde(rename = "service", default = "RudofConfig::default_service_config")]
    pub(crate) service: ServiceConfig,
    #[serde(rename = "comparator", default = "RudofConfig::default_comparator_config")]
    pub(crate) comparator: ComparatorConfig,
}

impl RudofConfig {
    /// Creates a new [`RudofConfig`] with default settings.
    pub fn new() -> Self {
        let mut cfg = Self {
            version: Self::default_version(),
            common: Self::default_common_config(),
            service: Self::default_service_config(),
            rdf_data: Self::default_rdf_data_config(),
            shex: Self::default_shex_config(),
            shex_validator: Self::default_shex_validator_config(),
            shex2uml: Self::default_shex2uml_config(),
            shex2html: Self::default_shex2html_config(),
            shacl2shex: Self::default_shacl2shex_config(),
            tap: Self::default_tap_config(),
            tap2shex: Self::default_tap2shex_config(),
            shex2sparql: Self::default_shex2sparql_config(),
            comparator: Self::default_comparator_config(),
        };
        cfg.fixup();
        cfg
    }

    /// Loads a [`RudofConfig`] from a TOML file.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the TOML configuration file
    ///
    /// # Errors
    ///
    /// * [`RudofConfigError::ReadError`] - If the file cannot be opened or read
    /// * [`RudofConfigError::TomlPathError`] - If the TOML content is invalid
    #[cfg(not(target_family = "wasm"))]
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, RudofConfigError> {
        let path_name = path.as_ref().display().to_string();
        let mut f = File::open(path).map_err(|e| RudofConfigError::ReadError {
            error: e.to_string(),
            path: path_name.to_string(),
        })?;
        let mut s = String::new();
        f.read_to_string(&mut s).map_err(|e| RudofConfigError::ReadError {
            error: e.to_string(),
            path: path_name.to_string(),
        })?;
        let mut config: RudofConfig = toml::from_str(s.as_str()).map_err(|e| RudofConfigError::TomlPathError {
            error: e.to_string(),
            path: path_name.to_string(),
        })?;
        config.fixup();
        Ok(config)
    }

    pub fn with_version(mut self, version: Option<Version>) -> Self {
        self.version = version;
        self
    }

    pub fn with_common(mut self, cfg: CommonConfig) -> Self {
        self.common = cfg;
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

    pub fn with_shex_validator(mut self, cfg: ValidatorConfig) -> Self {
        self.shex_validator = cfg;
        self
    }

    pub fn with_shex2uml(mut self, cfg: ShEx2UmlConfig) -> Self {
        self.shex2uml = cfg;
        self
    }

    pub fn with_shex2html(mut self, cfg: ShEx2HtmlConfig) -> Self {
        self.shex2html = cfg;
        self
    }

    pub fn with_shacl2shex(mut self, cfg: Shacl2ShExConfig) -> Self {
        self.shacl2shex = cfg;
        self
    }

    pub fn with_tap(mut self, cfg: TapConfig) -> Self {
        self.tap = cfg;
        self
    }

    pub fn with_tap2shex(mut self, cfg: Tap2ShExConfig) -> Self {
        self.tap2shex = cfg;
        self
    }

    pub fn with_shex2sparql(mut self, cfg: ShEx2SparqlConfig) -> Self {
        self.shex2sparql = cfg;
        self
    }

    pub fn with_service(mut self, cfg: ServiceConfig) -> Self {
        self.service = cfg;
        self
    }

    pub fn with_comparator(mut self, cfg: ComparatorConfig) -> Self {
        self.comparator = cfg;
        self
    }
}

impl RudofConfig {
    pub fn version(&self) -> Option<&Version> {
        self.version.as_ref()
    }

    pub fn common(&self) -> &CommonConfig {
        &self.common
    }

    pub fn service(&self) -> &ServiceConfig {
        &self.service
    }

    pub fn rdf_data(&self) -> &RdfDataConfig {
        &self.rdf_data
    }

    pub fn shex(&self) -> &ShExConfig {
        &self.shex
    }

    pub fn shex_validator(&self) -> &ValidatorConfig {
        &self.shex_validator
    }

    pub fn shex2uml(&self) -> &ShEx2UmlConfig {
        &self.shex2uml
    }

    pub fn shex2html(&self) -> &ShEx2HtmlConfig {
        &self.shex2html
    }

    pub fn shacl2shex(&self) -> &Shacl2ShExConfig {
        &self.shacl2shex
    }

    pub fn tap(&self) -> &TapConfig {
        &self.tap
    }

    pub fn tap2shex(&self) -> &Tap2ShExConfig {
        &self.tap2shex
    }

    pub fn shex2sparql(&self) -> &ShEx2SparqlConfig {
        &self.shex2sparql
    }

    pub fn comparator(&self) -> &ComparatorConfig {
        &self.comparator
    }
}

/// Serde stuff
#[allow(dead_code)]
#[cfg_attr(rustfmt, rustfmt_skip)]
impl RudofConfig {
    #[inline] fn default_version() -> Option<Version> { None }
    #[inline] fn default_common_config() -> CommonConfig { CommonConfig::default() }
    #[inline] fn default_service_config() -> ServiceConfig { ServiceConfig::default() }
    #[inline] fn default_rdf_data_config() -> RdfDataConfig { RdfDataConfig::default() }
    #[inline] fn default_shex_config() -> ShExConfig { ShExConfig::default() }
    #[inline] fn default_shex_validator_config() -> ValidatorConfig { ValidatorConfig::default() }
    #[inline] fn default_shex2uml_config() -> ShEx2UmlConfig { ShEx2UmlConfig::default() }
    #[inline] fn default_shex2html_config() -> ShEx2HtmlConfig { ShEx2HtmlConfig::default() }
    #[inline] fn default_shacl2shex_config() -> Shacl2ShExConfig { Shacl2ShExConfig::default() }
    #[inline] fn default_tap_config() -> TapConfig { TapConfig::default() }
    #[inline] fn default_tap2shex_config() -> Tap2ShExConfig { Tap2ShExConfig::default() }
    #[inline] fn default_shex2sparql_config() -> ShEx2SparqlConfig { ShEx2SparqlConfig::default() }
    #[inline] fn default_comparator_config() -> ComparatorConfig { ComparatorConfig::default() }

    pub fn fixup(&mut self) {
        self.service.fixup(self.common.base.clone());
        self.rdf_data.fixup(self.common.base.clone());
        self.shex.fixup(self.rdf_data.clone(),
            self.common.base.clone());
        self.shex_validator.fixup(self.rdf_data.clone(),
            self.shex.clone());
        self.shex2uml.fixup(self.shex.clone());
        self.shex2html.fixup(self.shex.clone(),
            self.shex2uml.clone());
        self.tap2shex.fixup(self.common.base.clone(), self.tap.clone());
        self.shex2sparql.fixup(self.shex.clone());
    }
}

impl Default for RudofConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for RudofConfig {
    type Err = RudofConfigError;

    /// Parses a `RudofConfig` from a TOML string.
    ///
    /// # Errors
    ///
    /// Returns [`RudofConfigError::TomlParseFromString`] if the TOML content is invalid.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut config: RudofConfig = toml::from_str(s).map_err(|e| RudofConfigError::TomlStringError {
            string: s.to_string(),
            error: e.to_string(),
        })?;
        config.fixup();
        Ok(config)
    }
}
