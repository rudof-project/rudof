use std::path::Path;

use dctap::TapConfig;
use serde_derive::{Deserialize, Serialize};
use shapes_converter::{
    ShEx2HtmlConfig, ShEx2SparqlConfig, ShEx2UmlConfig, Shacl2ShExConfig, Tap2ShExConfig,
};
use shex_validation::{ShExConfig, ValidatorConfig};
use srdf::RdfDataConfig;

use crate::RudofError;

/// `rudof_config` describes the configuration of Rudof
///
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone, Default)]
pub struct RudofConfig {
    rdf_data: Option<RdfDataConfig>,
    shex: Option<ShExConfig>,
    shex_validator: Option<ValidatorConfig>,
    shex2uml: Option<ShEx2UmlConfig>,
    shex2html: Option<ShEx2HtmlConfig>,
    shacl2shex: Option<Shacl2ShExConfig>,
    tap2shex: Option<Tap2ShExConfig>,
    tap: Option<TapConfig>,
    shex2sparql: Option<ShEx2SparqlConfig>,
}

impl RudofConfig {
    pub fn new() -> RudofConfig {
        Self::default()
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
        match &self.shex_validator {
            None => ValidatorConfig::default(),
            Some(cfg) => cfg.clone(),
        }
    }

    pub fn shex_config(&self) -> ShExConfig {
        match &self.shex {
            None => ShExConfig::default(),
            Some(cfg) => cfg.clone(),
        }
    }

    pub fn show_extends(&self) -> bool {
        self.shex_config().show_extends.unwrap_or(true)
    }

    pub fn show_imports(&self) -> bool {
        self.shex_config().show_extends.unwrap_or(true)
    }

    pub fn show_shapes(&self) -> bool {
        self.shex_config().show_shapes.unwrap_or(true)
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

    pub fn shex2sparql_config(&self) -> ShEx2SparqlConfig {
        self.shex2sparql.clone().unwrap_or_default()
    }

    pub fn shacl2shex_config(&self) -> Shacl2ShExConfig {
        self.shacl2shex.clone().unwrap_or_default()
    }
}
