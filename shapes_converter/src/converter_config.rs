use dctap::TapConfig;
use serde::{Deserialize, Serialize};
use std::io::Read;
use std::path::Path;

use crate::{ConverterError, ShEx2HtmlConfig, ShEx2SparqlConfig, ShEx2UmlConfig, Shacl2ShExConfig, Tap2ShExConfig};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone, Default)]
pub struct ConverterConfig {
    dctap: Option<TapConfig>,
    shex2html: Option<ShEx2HtmlConfig>,
    tap2shex: Option<Tap2ShExConfig>,
    shex2sparql: Option<ShEx2SparqlConfig>,
    shacl2shex: Option<Shacl2ShExConfig>,
    shex2uml: Option<ShEx2UmlConfig>,
}

impl ConverterConfig {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<ConverterConfig, ConverterError> {
        let path_name = path.as_ref().display().to_string();
        let mut f = std::fs::File::open(path).map_err(|e| ConverterError::ConverterConfigFromPathError {
            path: path_name.clone(),
            error: e,
        })?;
        let mut s = String::new();
        f.read_to_string(&mut s)
            .map_err(|e| ConverterError::ConverterConfigFromPathError {
                path: path_name.clone(),
                error: e,
            })?;

        let config: ConverterConfig =
            toml::from_str(s.as_str()).map_err(|e| ConverterError::ConverterConfigFromTomlError {
                path: path_name.clone(),
                error: e,
            })?;
        Ok(config)
    }

    pub fn tap_config(&self) -> TapConfig {
        match &self.dctap {
            Some(tc) => tc.clone(),
            None => TapConfig::default(),
        }
    }

    pub fn tap2shex_config(&self) -> Tap2ShExConfig {
        match &self.tap2shex {
            Some(c) => c.clone(),
            None => Tap2ShExConfig::default(),
        }
    }

    pub fn shex2html_config(&self) -> ShEx2HtmlConfig {
        match &self.shex2html {
            Some(c) => c.clone(),
            None => ShEx2HtmlConfig::default(),
        }
    }

    pub fn shex2uml_config(&self) -> ShEx2UmlConfig {
        match &self.shex2uml {
            Some(c) => c.clone(),
            None => ShEx2UmlConfig::default(),
        }
    }

    pub fn shacl2shex_config(&self) -> Shacl2ShExConfig {
        match &self.shacl2shex {
            Some(c) => c.clone(),
            None => Shacl2ShExConfig::default(),
        }
    }

    pub fn shex2sparql_config(&self) -> ShEx2SparqlConfig {
        match &self.shex2sparql {
            Some(c) => c.clone(),
            None => ShEx2SparqlConfig::default(),
        }
    }
}
