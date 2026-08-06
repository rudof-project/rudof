use dctap::TapConfig;
use rudof_config::TomlConfig;
use serde::{Deserialize, Serialize};

use crate::{ShEx2HtmlConfig, ShEx2SparqlConfig, ShEx2UmlConfig, Shacl2ShExConfig, Tap2ShExConfig};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(default)]
pub struct ConverterConfig {
    #[serde(rename = "dctap")]
    pub(crate) dctap: TapConfig,
    #[serde(rename = "shex2html")]
    pub(crate) shex2html: ShEx2HtmlConfig,
    #[serde(rename = "tap2shex")]
    pub(crate) tap2shex: Tap2ShExConfig,
    #[serde(rename = "shex2sparql")]
    pub(crate) shex2sparql: ShEx2SparqlConfig,
    #[serde(rename = "shacl2shex")]
    pub(crate) shacl2shex: Shacl2ShExConfig,
    #[serde(rename = "shex2uml")]
    pub(crate) shex2uml: ShEx2UmlConfig,
}

impl ConverterConfig {
    pub fn new() -> Self {
        Self {
            dctap: Self::default_dctap(),
            shex2html: Self::default_shex2html(),
            tap2shex: Self::default_tap2shex(),
            shex2sparql: Self::default_shex2sparql(),
            shacl2shex: Self::default_shacl2shex(),
            shex2uml: Self::default_shex2uml(),
        }
    }

    pub fn with_dctap(mut self, cfg: TapConfig) -> Self {
        self.dctap = cfg;
        self
    }

    pub fn with_shex2html(mut self, cfg: ShEx2HtmlConfig) -> Self {
        self.shex2html = cfg;
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

    pub fn with_shacl2shex(mut self, cfg: Shacl2ShExConfig) -> Self {
        self.shacl2shex = cfg;
        self
    }

    pub fn with_shex2uml(mut self, cfg: ShEx2UmlConfig) -> Self {
        self.shex2uml = cfg;
        self
    }
}

impl ConverterConfig {
    pub fn tap_config(&self) -> &TapConfig {
        &self.dctap
    }

    pub fn tap2shex_config(&self) -> &Tap2ShExConfig {
        &self.tap2shex
    }

    pub fn shex2html_config(&self) -> &ShEx2HtmlConfig {
        &self.shex2html
    }

    pub fn shex2uml_config(&self) -> &ShEx2UmlConfig {
        &self.shex2uml
    }

    pub fn shacl2shex_config(&self) -> &Shacl2ShExConfig {
        &self.shacl2shex
    }

    pub fn shex2sparql_config(&self) -> &ShEx2SparqlConfig {
        &self.shex2sparql
    }
}

/// Serde stuff
#[allow(dead_code)]
#[rustfmt::skip]
impl ConverterConfig {
    #[inline] fn default_dctap() -> TapConfig { TapConfig::default() }
    #[inline] fn default_shex2html() -> ShEx2HtmlConfig { ShEx2HtmlConfig::default() }
    #[inline] fn default_tap2shex() -> Tap2ShExConfig { Tap2ShExConfig::default() }
    #[inline] fn default_shex2sparql() -> ShEx2SparqlConfig { ShEx2SparqlConfig::default() }
    #[inline] fn default_shacl2shex() -> Shacl2ShExConfig { Shacl2ShExConfig::default() }
    #[inline] fn default_shex2uml() -> ShEx2UmlConfig { ShEx2UmlConfig::default() }
}

impl TomlConfig for ConverterConfig {}

impl Default for ConverterConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::ConverterConfig;
    use rudof_config::TomlConfig;

    #[test]
    fn defaults() {
        let c = ConverterConfig::default();
        assert_eq!(c.tap_config(), &ConverterConfig::default_dctap());
    }

    #[test]
    fn partial_toml_sets_one_subconfig() {
        let c = ConverterConfig::from_toml_str(r#"
            [dctap]
            delimiter = ";"
        "#).unwrap();
        assert_eq!(c.tap_config().delimiter(), ';');
    }

    #[test]
    fn toml_round_trip() {
        let c = ConverterConfig::from_toml_str(r#"
            [dctap]
            delimiter = ";"
        "#).unwrap();
        let s = c.to_toml_string().unwrap();
        let d = ConverterConfig::from_toml_str(&s).unwrap();
        assert_eq!(c, d);
    }
}
