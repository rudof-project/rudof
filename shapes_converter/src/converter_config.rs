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
