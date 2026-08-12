use rudof_config::TomlConfig;
use rudof_rdf::rdf_core::RdfDataConfig;
use serde::{Deserialize, Serialize};

/// This struct can be used to define the configuration of SHACL
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ShaclConfig {
    #[serde(rename = "rdf", skip_serializing)]
    pub(crate) data: RdfDataConfig,
}

impl ShaclConfig {
    pub fn new() -> Self {
        Self {
            data: Self::default_data_config(),
        }
    }

    pub fn with_rdf_data(mut self, data: RdfDataConfig) -> Self {
        self.data = data;
        self
    }
}

impl ShaclConfig {
    pub fn rdf_data(&self) -> &RdfDataConfig {
        &self.data
    }
}

/// Serde stuff
#[allow(dead_code)]
#[rustfmt::skip]
impl ShaclConfig {
    #[inline] fn default_data_config() -> RdfDataConfig { RdfDataConfig::default() }
}

impl Default for ShaclConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl TomlConfig for ShaclConfig {}

#[cfg(test)]
mod tests {
    use super::ShaclConfig;
    use rudof_config::TomlConfig;

    #[test]
    fn defaults() {
        assert_eq!(ShaclConfig::default().rdf_data(), &ShaclConfig::default_data_config());
    }

    #[test]
    fn partial_toml_sets_rdf_base() {
        let c = ShaclConfig::from_toml_str(
            r#"
            [rdf]
            base_iri = "http://ex/"
        "#,
        )
        .unwrap();
        assert_eq!(c.rdf_data().base().map(|i| i.as_str()), Some("http://ex/"));
    }

    #[test]
    fn toml_round_trip() {
        let c = ShaclConfig::default();
        let s = c.to_toml_string().unwrap();
        let d = ShaclConfig::from_toml_str(&s).unwrap();
        assert_eq!(c, d);
    }
}
