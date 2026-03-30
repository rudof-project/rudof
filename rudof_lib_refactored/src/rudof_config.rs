use crate::errors::ConfigError;
use dctap::TapConfig;
use rudof_rdf::rdf_core::RdfDataConfig;
use serde::{Deserialize, Serialize};
use shapes_comparator::ComparatorConfig;
use shapes_converter::{ShEx2HtmlConfig, ShEx2SparqlConfig, ShEx2UmlConfig, Shacl2ShExConfig, Tap2ShExConfig};
use shex_validation::{ShExConfig, ValidatorConfig};
use sparql_service::ServiceConfig;
use std::env;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::str::FromStr;

/// Embedded default configuration in TOML format.
const DEFAULT_CONFIG: &str = include_str!("default_config.toml");

/// Main configuration structure for Rudof.
///
/// This structure encapsulates all configuration options for Rudof operations,
/// including RDF data handling, schema validation (ShEx and SHACL), conversions,
/// and visualization settings.
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct RudofConfig {
    pub(crate) rdf_data: Option<RdfDataConfig>,
    pub(crate) shex: Option<ShExConfig>,
    pub(crate) shex_validator: Option<ValidatorConfig>,
    pub(crate) shex2uml: Option<ShEx2UmlConfig>,
    pub(crate) shex2html: Option<ShEx2HtmlConfig>,
    pub(crate) shacl2shex: Option<Shacl2ShExConfig>,
    pub(crate) tap: Option<TapConfig>,
    pub(crate) tap2shex: Option<Tap2ShExConfig>,
    pub(crate) shex2sparql: Option<ShEx2SparqlConfig>,
    pub(crate) service: Option<ServiceConfig>,
    pub(crate) plantuml_path: Option<PathBuf>,
    pub(crate) comparator: Option<ComparatorConfig>,
}

impl RudofConfig {
    /// Creates a new `RudofConfig` with default settings.
    pub fn new() -> Self {
        RudofConfig::from_str(DEFAULT_CONFIG).unwrap()
    }

    /// Loads a `RudofConfig` from a TOML file.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the TOML configuration file
    ///
    /// # Errors
    ///
    /// * [`ConfigError::ReadFromPath`] - If the file cannot be opened or read
    /// * [`ConfigError::TomlParseFromPath`] - If the TOML content is invalid
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let path_name = path.as_ref().display().to_string();
        let mut f = std::fs::File::open(path).map_err(|e| ConfigError::ReadFromPath {
            path: path_name.clone(),
            error: e,
        })?;
        let mut s = String::new();
        f.read_to_string(&mut s).map_err(|e| ConfigError::ReadFromPath {
            path: path_name.clone(),
            error: e,
        })?;

        let config: RudofConfig = toml::from_str(s.as_str()).map_err(|e| ConfigError::TomlParseFromPath {
            path: path_name.clone(),
            error: e,
        })?;
        Ok(config)
    }

    // ---------------------------------------------------------------------------
    // Dctap configuration
    // ---------------------------------------------------------------------------

    pub fn dctap_config(&self) -> TapConfig {
        self.tap.clone().unwrap_or_default()
    }

    // ---------------------------------------------------------------------------
    // ShEx configuration
    // ---------------------------------------------------------------------------

    /// Returns whether to show shape extends in ShEx schemas.
    ///
    /// Defaults to `false` if not configured.
    pub fn show_extends(&self) -> bool {
        self.shex_config().show_extends.unwrap_or(false)
    }

    /// Returns whether to show imports in ShEx schemas.
    ///
    /// Defaults to `false` if not configured.
    pub fn show_imports(&self) -> bool {
        self.shex_config().show_extends.unwrap_or(false)
    }

    /// Returns whether to show shapes in ShEx schemas.
    ///
    /// Defaults to `false` if not configured.
    pub fn show_shapes(&self) -> bool {
        self.shex_config().show_shapes.unwrap_or(false)
    }

    /// Returns whether to show dependencies in ShEx schemas.
    ///
    /// Defaults to `false` if not configured.
    pub fn show_dependencies(&self) -> bool {
        self.shex_config().show_dependencies.unwrap_or(false)
    }

    /// Returns whether to show the internal representation (IR) of  ShEx schemas.
    ///
    /// Defaults to `true` if not configured.
    pub fn show_ir(&self) -> bool {
        self.shex_config().show_ir.unwrap_or(true)
    }

    /// Disables statistics display in ShEx operations.
    ///
    /// If no ShEx configuration exists, creates one with statistics disabled.
    pub fn shex_without_showing_stats(&mut self) {
        if let Some(shex_config) = &mut self.shex {
            shex_config.without_showing_stats();
        } else {
            let mut shex_config = ShExConfig::default();
            shex_config.without_showing_stats();
            self.shex = Some(shex_config);
        }
    }

    /// Returns the ShEx validator configuration.
    ///
    /// Returns a default configuration if none was specified.
    pub fn validator_config(&self) -> ValidatorConfig {
        match &self.shex_validator {
            None => ValidatorConfig::default(),
            Some(cfg) => cfg.clone(),
        }
    }

    // ---------------------------------------------------------------------------
    // RDF data configuration
    // ---------------------------------------------------------------------------

    /// Returns the base IRI for RDF data, if configured.
    ///
    /// Returns `None` if no base IRI is set in the configuration.
    pub fn rdf_data_base(&self) -> Option<&str> {
        match &self.rdf_data {
            None => None,
            Some(rdf_data_config) => rdf_data_config.base.as_ref().map(|i| i.as_str()),
        }
    }

    /// Returns whether automatic base IRI detection is enabled.
    ///
    /// Defaults to `true` if not configured.
    pub fn automatic_base(&self) -> bool {
        match &self.rdf_data {
            None => true,
            Some(rdf_data_config) => rdf_data_config.automatic_base.unwrap_or(true),
        }
    }

    /// Sets the PlantUML executable path using the builder pattern.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the PlantUML executable or JAR file
    pub fn with_plantuml_path<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.plantuml_path = Some(path.as_ref().to_owned());
        self
    }

    /// Returns the path to the PlantUML executable.
    ///
    /// The path is determined in the following order:
    /// 1. The explicitly configured path via [`with_plantuml_path`](Self::with_plantuml_path)
    /// 2. The `PLANTUML` environment variable
    /// 3. The current working directory
    pub fn plantuml_path(&self) -> PathBuf {
        if let Some(path) = &self.plantuml_path {
            path.to_owned()
        } else {
            match env::var("PLANTUML") {
                Ok(value) => Path::new(value.as_str()).to_path_buf(),
                Err(_) => env::current_dir().unwrap(),
            }
        }
    }

    // ----------------------------------------------------------------------------
    // Helper methods to access specific configurations with defaults
    // ----------------------------------------------------------------------------

    pub(crate) fn rdf_data_config(&self) -> RdfDataConfig {
        self.rdf_data.clone().unwrap_or_default()
    }

    /// Returns the ShEx schema configuration.
    ///
    /// Returns a default configuration if none was specified.
    pub(crate) fn shex_config(&self) -> ShExConfig {
        match &self.shex {
            None => ShExConfig::default(),
            Some(cfg) => cfg.clone(),
        }
    }
}

impl Default for RudofConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for RudofConfig {
    type Err = ConfigError;

    /// Parses a `RudofConfig` from a TOML string.
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError::TomlParseFromString`] if the TOML content is invalid.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        toml::from_str(s).map_err(|e| ConfigError::TomlParseFromString {
            content: s.to_string(),
            error: e,
        })
    }
}
