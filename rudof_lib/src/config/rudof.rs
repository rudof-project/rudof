use crate::config::CommonConfig;
use dctap::TapConfig;
use rudof_config::{ConfigError, TomlConfig, find_config_files_from, merge_tables, read_toml_table, user_config_file};
use rudof_rdf::rdf_core::RdfDataConfig;
use semver::Version;
use serde::{Deserialize, Serialize};
#[cfg(not(target_family = "wasm"))]
use shacl::validator::ShaclConfig;
use shapes_comparator::ComparatorConfig;
use shapes_converter::{ShEx2HtmlConfig, ShEx2SparqlConfig, ShEx2UmlConfig, Shacl2ShExConfig, Tap2ShExConfig};
use shex_validation::{ShExConfig, ValidatorConfig};
use sparql_service::ServiceConfig;
use std::path::Path;
use std::str::FromStr;

/// Main configuration structure for Rudof.
///
/// This structure encapsulates all configuration options for Rudof operations,
/// including RDF data handling, schema validation (ShEx and SHACL), conversions,
/// and visualization settings.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(default)]
pub struct RudofConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) version: Option<Version>,

    #[serde(flatten)]
    pub(crate) common: CommonConfig,

    // ---
    #[serde(rename = "rdf")]
    pub(crate) rdf_data: RdfDataConfig,
    #[serde(rename = "shex")]
    pub(crate) shex: ShExConfig,
    #[serde(rename = "shex_validator")]
    pub(crate) shex_validator: ValidatorConfig,
    #[cfg(not(target_family = "wasm"))]
    #[serde(rename = "shacl")]
    pub(crate) shacl: ShaclConfig,
    #[serde(rename = "shex2uml")]
    pub(crate) shex2uml: ShEx2UmlConfig,
    #[serde(rename = "shex2html")]
    pub(crate) shex2html: ShEx2HtmlConfig,
    #[serde(rename = "shacl2shex")]
    pub(crate) shacl2shex: Shacl2ShExConfig,
    #[serde(rename = "tap")]
    pub(crate) tap: TapConfig,
    #[serde(rename = "tap2shex")]
    pub(crate) tap2shex: Tap2ShExConfig,
    #[serde(rename = "shex2sparql")]
    pub(crate) shex2sparql: ShEx2SparqlConfig,
    #[serde(rename = "service")]
    pub(crate) service: ServiceConfig,
    #[serde(rename = "comparator")]
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
            #[cfg(not(target_family = "wasm"))]
            shacl: Self::default_shacl_config(),
            shex2uml: Self::default_shex2uml_config(),
            shex2html: Self::default_shex2html_config(),
            shacl2shex: Self::default_shacl2shex_config(),
            tap: Self::default_tap_config(),
            tap2shex: Self::default_tap2shex_config(),
            shex2sparql: Self::default_shex2sparql_config(),
            comparator: Self::default_comparator_config(),
        };
        cfg.resolve();
        cfg
    }

    /// Loads the configuration, either from an explicit file or by discovering
    /// and merging the standard locations.
    ///
    /// When no explicit file is given, sources are merged per-key, from lowest to
    /// highest precedence:
    /// 1. The built-in defaults ([`RudofConfig::default`]).
    /// 2. The platform-specific user config file:
    ///    - Linux: `~/.config/rudof/config.toml`
    ///    - Windows: `%LOCALAPPDATA%\rudof\config.toml`
    ///    - macOS: `~/Library/Application Support/rudof/config.toml`
    /// 3. Every `rudof.toml` found by walking from the filesystem root down to the
    ///    current working directory.
    ///
    /// # Errors
    ///
    /// Returns a [`ConfigError`] if any read file cannot be read or contains invalid
    /// TOML. Missing discovered files are silently skipped.
    #[cfg(not(target_family = "wasm"))]
    pub fn discover(explicit: Option<&Path>) -> Result<Self, ConfigError> {
        // Explicit file
        if let Some(path) = explicit {
            return Self::from_path(path);
        }

        let mut merged = toml::Table::new();

        // Platform-specific user config directory
        if let Some(path) = user_config_file("rudof", "config.toml")
            && path.is_file()
        {
            merge_tables(&mut merged, read_toml_table(&path)?);
        }

        // `rudof.toml` files from the filesystem root down to the CWD
        if let Ok(cwd) = std::env::current_dir() {
            for path in find_config_files_from(&cwd, "rudof.toml") {
                merge_tables(&mut merged, read_toml_table(&path)?);
            }
        }

        Self::from_table(merged)
    }

    #[cfg(not(target_family = "wasm"))]
    fn from_table(table: toml::Table) -> Result<Self, ConfigError> {
        let mut config: RudofConfig =
            toml::Value::Table(table)
                .try_into()
                .map_err(|e: toml::de::Error| ConfigError::Parse {
                    location: "<merged config>".to_string(),
                    error: e.to_string(),
                })?;
        config.check_version()?;
        config.resolve();
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

    #[cfg(not(target_family = "wasm"))]
    pub fn with_shacl(mut self, cfg: ShaclConfig) -> Self {
        self.shacl = cfg;
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

    #[cfg(not(target_family = "wasm"))]
    pub fn shacl(&self) -> &ShaclConfig {
        &self.shacl
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

impl RudofConfig {
    /// Returns config version, paired with rudof version
    fn current_version() -> Version {
        Version::parse(env!("CARGO_PKG_VERSION")).expect("CARGO_PKG_VERSION is a valid semver")
    }

    /// Checks the config's declared version against this build
    ///
    /// # Errors
    ///
    /// ([`ConfigError::IncompatibleVersion`]) if the config's **major** version
    /// is newer than this build's, Warns on any other version mismatch
    fn check_version(&self) -> Result<(), ConfigError> {
        let Some(config_version) = self.version.as_ref() else {
            tracing::warn!("config file does not declare a version");
            return Ok(());
        };
        let rudof_version = Self::current_version();

        if config_version.major > rudof_version.major {
            return Err(ConfigError::IncompatibleVersion {
                config: config_version.to_string(),
                rudof: rudof_version.to_string(),
            });
        }

        if *config_version != rudof_version {
            tracing::warn!(
                "config file targets rudof {config_version}, but this is rudof {rudof_version}; \
                 some settings may be ignored or behave differently"
            );
        }
        Ok(())
    }
}

/// Serde stuff
#[allow(dead_code)]
#[rustfmt::skip]
impl RudofConfig {
    #[inline] fn default_version() -> Option<Version> { Some(Self::current_version()) }
    #[inline] fn default_common_config() -> CommonConfig { CommonConfig::default() }
    #[inline] fn default_service_config() -> ServiceConfig { ServiceConfig::default() }
    #[inline] fn default_rdf_data_config() -> RdfDataConfig { RdfDataConfig::default() }
    #[inline] fn default_shex_config() -> ShExConfig { ShExConfig::default() }
    #[inline] fn default_shex_validator_config() -> ValidatorConfig { ValidatorConfig::default() }
    #[cfg(not(target_family = "wasm"))]
    #[inline] fn default_shacl_config() -> ShaclConfig { ShaclConfig::default() }
    #[inline] fn default_shex2uml_config() -> ShEx2UmlConfig { ShEx2UmlConfig::default() }
    #[inline] fn default_shex2html_config() -> ShEx2HtmlConfig { ShEx2HtmlConfig::default() }
    #[inline] fn default_shacl2shex_config() -> Shacl2ShExConfig { Shacl2ShExConfig::default() }
    #[inline] fn default_tap_config() -> TapConfig { TapConfig::default() }
    #[inline] fn default_tap2shex_config() -> Tap2ShExConfig { Tap2ShExConfig::default() }
    #[inline] fn default_shex2sparql_config() -> ShEx2SparqlConfig { ShEx2SparqlConfig::default() }
    #[inline] fn default_comparator_config() -> ComparatorConfig { ComparatorConfig::default() }

    /// Resolves cross-section inheritance after all config layers have been merged
    pub fn resolve(&mut self) {
        let base = self.common.base().cloned();

        // Base propagation
        if self.rdf_data.base().is_none() {
            self.rdf_data = self.rdf_data.clone().with_base(base.clone());
        }
        if self.service.base().is_none() {
            self.service = self.service.clone().with_base(base.clone());
        }
        if self.shex.base().is_none() {
            self.shex = self.shex.clone().with_base(base.clone());
        }
        if self.tap2shex.base_iri().is_none() {
            self.tap2shex = self.tap2shex.clone().with_base_iri(base.clone());
        }

        // RDF propagation
        self.shex = self.shex.clone().with_rdf_config_shex(self.rdf_data.clone());

        #[cfg(not(target_family = "wasm"))]
        {
            self.shacl = self.shacl.clone().with_rdf_data(self.rdf_data.clone());
        }

        // Tap propagation
        self.tap2shex = self.tap2shex.clone().with_dctap(self.tap.clone());

        // ShEx and RDF propagation
        self.shex_validator = self
            .shex_validator
            .clone()
            .with_shex(self.shex.clone())
            .with_rdf_data(self.rdf_data.clone());

        // ShEx propagation
        self.shex2uml = self.shex2uml.clone().with_shex(self.shex.clone());
        self.shex2html = self
            .shex2html
            .clone()
            .with_shex(self.shex.clone())
            .with_shex2uml(self.shex2uml.clone());
        self.shex2sparql = self.shex2sparql.clone().with_shex(self.shex.clone());
    }
}

impl Default for RudofConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl TomlConfig for RudofConfig {
    fn from_toml_str(s: &str) -> Result<Self, ConfigError> {
        let mut config: RudofConfig = toml::from_str(s).map_err(|e| ConfigError::Parse {
            location: "<string>".to_string(),
            error: e.to_string(),
        })?;
        config.check_version()?;
        config.resolve();
        Ok(config)
    }
}

impl FromStr for RudofConfig {
    type Err = ConfigError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        <Self as TomlConfig>::from_toml_str(s)
    }
}

#[cfg(all(test, not(target_family = "wasm")))]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn closest_rudof_toml_wins() {
        let tmp = std::env::temp_dir().join(format!("rudof_discover_{}", std::process::id()));
        let nested = tmp.join("a").join("b");
        fs::create_dir_all(&nested).unwrap();

        fs::write(tmp.join("rudof.toml"), r#"base_iri = "http://root/""#).unwrap();
        fs::write(nested.join("rudof.toml"), r#"base_iri = "http://nested/""#).unwrap();

        let files = find_config_files_from(&nested, "rudof.toml");

        assert_eq!(files.first().unwrap(), &tmp.join("rudof.toml"));
        assert_eq!(files.last().unwrap(), &nested.join("rudof.toml"));

        let mut merged = toml::Table::new();
        for path in &files {
            merge_tables(&mut merged, read_toml_table(path).unwrap());
        }
        assert_eq!(merged["base_iri"].as_str(), Some("http://nested/"));

        fs::remove_dir_all(&tmp).ok();
    }

    #[test]
    fn from_table_empty_yields_defaults() {
        let from_table = RudofConfig::from_table(toml::Table::new()).unwrap();
        assert_eq!(from_table, RudofConfig::default());
    }

    #[test]
    fn version_incompatible_major_is_rejected() {
        let result = RudofConfig::from_str(r#"version = "999.0.0""#);
        assert!(matches!(result, Err(ConfigError::IncompatibleVersion { .. })));
    }

    #[test]
    fn version_matching_current_is_accepted() {
        let v = env!("CARGO_PKG_VERSION");
        let cfg = RudofConfig::from_str(&format!(r#"version = "{v}""#)).unwrap();
        assert_eq!(cfg.version(), Some(&Version::parse(v).unwrap()));
    }

    #[test]
    fn version_absent_is_stamped_with_current() {
        let cfg = RudofConfig::from_str(r#"base_iri = "http://x/""#).unwrap();
        assert_eq!(cfg.version(), RudofConfig::default_version().as_ref());
    }

    #[test]
    fn resolve_propagates_common_base_to_sections() {
        let cfg = RudofConfig::from_str("base_iri = \"http://common/\"\n").unwrap();
        let base = Some("http://common/");
        assert_eq!(cfg.rdf_data().base().map(|i| i.as_str()), base);
        assert_eq!(cfg.shex().base().map(|i| i.as_str()), base);
        assert_eq!(cfg.service().base().map(|i| i.as_str()), base);
        assert_eq!(cfg.tap2shex().base_iri().map(|i| i.as_str()), base);
    }

    #[test]
    fn resolve_preserves_per_section_base_override() {
        let cfg = RudofConfig::from_str(
            r#"
            base_iri = "http://common/"
            [rdf]
            base_iri = "http://rdf-specific/"
            "#,
        )
        .unwrap();
        assert_eq!(cfg.rdf_data().base().map(|i| i.as_str()), Some("http://rdf-specific/"));
        assert_eq!(cfg.shex().base().map(|i| i.as_str()), Some("http://common/"));
    }

    #[test]
    fn resolve_injects_canonical_sections() {
        let cfg = RudofConfig::from_str(
            r#"
            [shex]
            show_imports = false
            [tap]
            delimiter = ";"
            "#,
        )
        .unwrap();
        assert!(!cfg.shex().show_imports());
        assert_eq!(cfg.tap().delimiter(), ';');
        assert_eq!(cfg.shex_validator().shex(), cfg.shex());
        assert_eq!(cfg.shex_validator().rdf_data(), cfg.rdf_data());
        assert_eq!(cfg.shex2uml().shex(), cfg.shex());
        assert_eq!(cfg.shex2html().shex(), cfg.shex());
        assert_eq!(cfg.shex2html().shex2uml(), cfg.shex2uml());
        assert_eq!(cfg.shex2sparql().shex(), cfg.shex());
        assert_eq!(cfg.tap2shex().dctap(), cfg.tap());
        assert_eq!(cfg.shex().rdf_config_shex(), cfg.rdf_data());
        assert_eq!(cfg.shacl().rdf_data(), cfg.rdf_data());
    }

    #[test]
    fn rudof_config_toml_round_trip() {
        let original = RudofConfig::from_str(
            r#"
            base_iri = "http://ex/"
            [shex]
            show_imports = false
            [shex_validator]
            max_steps = 42
            "#,
        )
        .unwrap();
        let serialized = toml::to_string(&original).unwrap();
        let reparsed = RudofConfig::from_str(&serialized).unwrap();
        assert_eq!(original, reparsed);
    }

    #[test]
    fn default_rudof_config_toml_round_trip() {
        let original = RudofConfig::default();
        let serialized = toml::to_string(&original).unwrap();
        let reparsed = RudofConfig::from_str(&serialized).unwrap();
        assert_eq!(original, reparsed);
    }

    #[test]
    fn discover_layers_partial_override_keeps_defaults() {
        let mut merged = toml::Table::new();
        merge_tables(&mut merged, toml::from_str(r#"base_iri = "http://layered/""#).unwrap());
        let layered = RudofConfig::from_table(merged).unwrap();
        let defaults = RudofConfig::default();

        assert_eq!(layered.shex().show_imports(), defaults.shex().show_imports());
        assert_eq!(
            layered.shex_validator().max_steps(),
            defaults.shex_validator().max_steps()
        );
        assert_eq!(layered.rdf_data().base().map(|i| i.as_str()), Some("http://layered/"));
    }
}
