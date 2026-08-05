use dctap::TapConfig;
use rudof_config::TomlConfig;
use serde::{Deserialize, Serialize};

use crate::{ShEx2HtmlConfig, ShEx2SparqlConfig, ShEx2UmlConfig, Shacl2ShExConfig, Tap2ShExConfig};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct ConverterConfig {
    dctap: Option<TapConfig>,
    shex2html: Option<ShEx2HtmlConfig>,
    tap2shex: Option<Tap2ShExConfig>,
    shex2sparql: Option<ShEx2SparqlConfig>,
    shacl2shex: Option<Shacl2ShExConfig>,
    shex2uml: Option<ShEx2UmlConfig>,
}

impl TomlConfig for ConverterConfig {}

impl ConverterConfig {
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
