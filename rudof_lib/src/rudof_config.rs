use crate::RudofError;
use dctap::TapConfig;
use serde::{Deserialize, Serialize};
use shapes_comparator::ComparatorConfig;
use shapes_converter::{ShEx2HtmlConfig, ShEx2SparqlConfig, ShEx2UmlConfig, Shacl2ShExConfig, Tap2ShExConfig};
use shex_validation::{ShExConfig, ValidatorConfig};
use sparql_service::ServiceConfig;
use srdf::{PLANTUML, RdfDataConfig};
use std::env;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::str::FromStr;

const DEFAULT_CONFIG: &str = include_str!("default_config.toml");

/// `rudof_config` describes the configuration of Rudof
///
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct RudofConfig {
    rdf_data: Option<RdfDataConfig>,
    shex: Option<ShExConfig>,
    shex_validator: Option<ValidatorConfig>,
    shex2uml: Option<ShEx2UmlConfig>,
    shex2html: Option<ShEx2HtmlConfig>,
    shacl2shex: Option<Shacl2ShExConfig>,
    tap: Option<TapConfig>,
    tap2shex: Option<Tap2ShExConfig>,
    shex2sparql: Option<ShEx2SparqlConfig>,
    service: Option<ServiceConfig>,
    plantuml_path: Option<PathBuf>,
    comparator: Option<ComparatorConfig>,
}

impl RudofConfig {
    pub fn new() -> Result<RudofConfig, RudofError> {
        RudofConfig::default_config()
    }

    /// Returns the default config which is read from the embedded default_config.toml
    pub fn default_config() -> Result<RudofConfig, RudofError> {
        RudofConfig::from_str(DEFAULT_CONFIG)
    }

    pub fn with_rdf_data_config(mut self, rdf_data_config: RdfDataConfig) -> Self {
        self.rdf_data = Some(rdf_data_config);
        self
    }

    pub fn with_shex_validator_config(mut self, shex_validator_config: ValidatorConfig) -> Self {
        self.shex_validator = Some(shex_validator_config);
        self
    }

    pub fn with_shex_config(mut self, shex_config: ShExConfig) -> Self {
        self.shex = Some(shex_config);
        self
    }

    /// Obtain a RudofConfig from a path file in TOML
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<RudofConfig, RudofError> {
        let path_name = path.as_ref().display().to_string();
        let mut f = std::fs::File::open(path).map_err(|e| RudofError::RudofConfigFromPathError {
            path: path_name.clone(),
            error: e,
        })?;
        let mut s = String::new();
        f.read_to_string(&mut s)
            .map_err(|e| RudofError::RudofConfigFromPathError {
                path: path_name.clone(),
                error: e,
            })?;

        let config: RudofConfig = toml::from_str(s.as_str()).map_err(|e| RudofError::RudofConfigTomlError {
            path: path_name.clone(),
            error: e,
        })?;
        Ok(config)
    }

    pub fn validator_config(&self) -> ValidatorConfig {
        match &self.shex_validator {
            None => ValidatorConfig::default(),
            Some(cfg) => cfg.clone(),
        }
    }

    pub fn comparator_config(&self) -> ComparatorConfig {
        match self.comparator {
            None => ComparatorConfig::new(),
            Some(ref cfg) => cfg.clone(),
        }
    }

    pub fn shex_config(&self) -> ShExConfig {
        match &self.shex {
            None => ShExConfig::default(),
            Some(cfg) => cfg.clone(),
        }
    }

    pub fn show_extends(&self) -> bool {
        self.shex_config().show_extends.unwrap_or(false)
    }

    pub fn show_imports(&self) -> bool {
        self.shex_config().show_extends.unwrap_or(false)
    }

    pub fn show_shapes(&self) -> bool {
        self.shex_config().show_shapes.unwrap_or(false)
    }

    pub fn show_dependencies(&self) -> bool {
        self.shex_config().show_dependencies.unwrap_or(false)
    }

    pub fn show_ir(&self) -> bool {
        self.shex_config().show_ir.unwrap_or(true)
    }

    pub fn rdf_data_config(&self) -> RdfDataConfig {
        self.rdf_data.clone().unwrap_or_default()
    }

    pub fn tap_config(&self) -> TapConfig {
        self.tap.clone().unwrap_or_default()
    }

    pub fn tap2shex_config(&self) -> Tap2ShExConfig {
        self.tap2shex.clone().unwrap_or_default()
    }

    pub fn shex2uml_config(&self) -> ShEx2UmlConfig {
        self.shex2uml.clone().unwrap_or_default()
    }

    pub fn shex2html_config(&self) -> ShEx2HtmlConfig {
        self.shex2html.clone().unwrap_or_default()
    }

    pub fn service_config(&self) -> ServiceConfig {
        self.service.clone().unwrap_or_default()
    }

    pub fn shex2sparql_config(&self) -> ShEx2SparqlConfig {
        self.shex2sparql.clone().unwrap_or_default()
    }

    pub fn shacl2shex_config(&self) -> Shacl2ShExConfig {
        self.shacl2shex.clone().unwrap_or_default()
    }

    pub fn rdf_data_base(&self) -> Option<&str> {
        match &self.rdf_data {
            None => None,
            Some(rdf_data_config) => rdf_data_config.base.as_ref().map(|i| i.as_str()),
        }
    }

    pub fn automatic_base(&self) -> bool {
        match &self.rdf_data {
            None => true,
            Some(rdf_data_config) => rdf_data_config.automatic_base.unwrap_or(true),
        }
    }

    pub fn shex_without_showing_stats(&mut self) {
        if let Some(shex_config) = &mut self.shex {
            shex_config.without_showing_stats();
        } else {
            let mut shex_config = ShExConfig::default();
            shex_config.without_showing_stats();
            self.shex = Some(shex_config);
        }
    }

    pub fn with_plantuml_path<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.plantuml_path = Some(path.as_ref().to_owned());
        self
    }

    pub fn plantuml_path(&self) -> PathBuf {
        if let Some(path) = &self.plantuml_path {
            path.to_owned()
        } else {
            match env::var(PLANTUML) {
                Ok(value) => Path::new(value.as_str()).to_path_buf(),
                Err(_) => env::current_dir().unwrap(),
            }
        }
    }
}

impl FromStr for RudofConfig {
    type Err = RudofError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        toml::from_str(s).map_err(|e| RudofError::RudofConfigFromStrError {
            str: s.to_string(),
            error: e.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use iri_s::iri;

    use super::*;

    #[test]
    fn test_rudof_config() {
        let s = r#"[tap2shex]
base_iri = "http://example.org/"

[tap2shex.prefixmap]
dct = "http://purl.org/dc/terms/"
rdf = "http://www.w3.org/1999/02/22-rdf-syntax-ns#"
foaf = "http://xmlns.com/foaf/0.1/"
xsd = "http://www.w3.org/2001/XMLSchema#"
sdo = "https://schema.org/"
ex = "http://example.org/"
"#;
        let config = RudofConfig::from_str(s).unwrap();
        assert_eq!(config.tap2shex_config().base_iri.unwrap(), iri!("http://example.org/"));
        assert_eq!(
            config.tap2shex_config().prefixmap().find("sdo").unwrap().clone(),
            iri!("https://schema.org/")
        );
    }
}
