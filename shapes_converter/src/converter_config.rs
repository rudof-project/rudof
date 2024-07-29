use std::path::Path;

use dctap::TapConfig;
use serde_derive::{Deserialize, Serialize};

use crate::{ConverterError, ShEx2HtmlConfig, ShEx2SparqlConfig, ShEx2UmlConfig, Tap2ShExConfig};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone, Default)]
pub struct ConverterConfig {
    dctap_config: Option<TapConfig>,
    shex2html_config: Option<ShEx2HtmlConfig>,
    tap2shex_config: Option<Tap2ShExConfig>,
    shex2sparql_config: Option<ShEx2SparqlConfig>,
    shex2uml_config: Option<ShEx2UmlConfig>,
}

impl ConverterConfig {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<ConverterConfig, ConverterError> {
        let path_name = path.as_ref().display().to_string();
        let f = std::fs::File::open(path).map_err(|e| {
            ConverterError::ConverterConfigFromPathError {
                path: path_name.clone(),
                error: e,
            }
        })?;
        let config: ConverterConfig = serde_yml::from_reader(f).map_err(|e| {
            ConverterError::ConverterConfigFromYAMLError {
                path: path_name.clone(),
                error: e,
            }
        })?;
        Ok(config)
    }

    pub fn tap_config(&self) -> TapConfig {
        match &self.dctap_config {
            Some(tc) => tc.clone(),
            None => TapConfig::default(),
        }
    }

    pub fn tap2shex_config(&self) -> Tap2ShExConfig {
        match &self.tap2shex_config {
            Some(c) => c.clone(),
            None => Tap2ShExConfig::default(),
        }
    }

    pub fn shex2html_config(&self) -> ShEx2HtmlConfig {
        match &self.shex2html_config {
            Some(c) => c.clone(),
            None => ShEx2HtmlConfig::default(),
        }
    }

    pub fn shex2uml_config(&self) -> ShEx2UmlConfig {
        match &self.shex2uml_config {
            Some(c) => c.clone(),
            None => ShEx2UmlConfig::default(),
        }
    }
    pub fn shex2sparql_config(&self) -> ShEx2SparqlConfig {
        match &self.shex2sparql_config {
            Some(c) => c.clone(),
            None => ShEx2SparqlConfig::default(),
        }
    }
}
