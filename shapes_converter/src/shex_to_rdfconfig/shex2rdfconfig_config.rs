use rudof_config::TomlConfig;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
#[serde(default)]
pub struct ShEx2RdfConfigConfig {}

impl ShEx2RdfConfigConfig {
    pub fn new() -> Self {
        Self::default()
    }
}

impl TomlConfig for ShEx2RdfConfigConfig {}

#[cfg(test)]
mod tests {
    use super::ShEx2RdfConfigConfig;
    use rudof_config::TomlConfig;

    #[test]
    fn defaults() {
        assert_eq!(ShEx2RdfConfigConfig::new(), ShEx2RdfConfigConfig::default());
    }

    #[test]
    fn empty_toml_is_default() {
        let c = ShEx2RdfConfigConfig::from_toml_str("").unwrap();
        assert_eq!(c, ShEx2RdfConfigConfig::default());
    }

    #[test]
    fn toml_round_trip() {
        let c = ShEx2RdfConfigConfig::default();
        let s = c.to_toml_string().unwrap();
        let d = ShEx2RdfConfigConfig::from_toml_str(&s).unwrap();
        assert_eq!(c, d);
    }
}