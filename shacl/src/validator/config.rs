use crate::validator::RecursionSemantics;
use rudof_config::TomlConfig;
use rudof_rdf::rdf_core::RdfDataConfig;
use serde::{Deserialize, Serialize};

/// This struct can be used to define the configuration of SHACL
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ShaclConfig {
    #[serde(rename = "rdf", skip_serializing)]
    pub(crate) data: RdfDataConfig,

    /// Whether the final `ValidationReport` retains violations (default: true,
    /// today's behavior). The cache always computes them internally regardless
    /// of this flag — it only controls what's exposed in the report.
    #[serde(rename = "store_errors")]
    pub(crate) store_errors: bool,

    /// Whether the final `ValidationReport` retains evidence for why
    /// `(node, shape)` pairs conform (default: false).
    #[serde(rename = "store_evidences")]
    pub(crate) store_evidences: bool,

    /// Fixpoint semantics assumed when a recursive shape reference (a
    /// cycle) is encountered during validation (default: cautious/LFP).
    /// See [`RecursionSemantics`].
    #[serde(rename = "recursion_semantics")]
    pub(crate) recursion_semantics: RecursionSemantics,
}

impl ShaclConfig {
    pub fn new() -> Self {
        Self {
            data: Self::default_data_config(),
            store_errors: Self::default_store_errors(),
            store_evidences: Self::default_store_evidences(),
            recursion_semantics: Self::default_recursion_semantics(),
        }
    }

    pub fn with_rdf_data(mut self, data: RdfDataConfig) -> Self {
        self.data = data;
        self
    }

    pub fn with_store_errors(mut self, flag: bool) -> Self {
        self.store_errors = flag;
        self
    }

    pub fn with_store_evidences(mut self, flag: bool) -> Self {
        self.store_evidences = flag;
        self
    }

    pub fn with_recursion_semantics(mut self, semantics: RecursionSemantics) -> Self {
        self.recursion_semantics = semantics;
        self
    }
}

impl ShaclConfig {
    pub fn rdf_data(&self) -> &RdfDataConfig {
        &self.data
    }

    pub fn store_errors(&self) -> bool {
        self.store_errors
    }

    pub fn store_evidences(&self) -> bool {
        self.store_evidences
    }

    pub fn recursion_semantics(&self) -> RecursionSemantics {
        self.recursion_semantics
    }
}

/// Serde stuff
#[allow(dead_code)]
#[rustfmt::skip]
impl ShaclConfig {
    #[inline] fn default_data_config() -> RdfDataConfig { RdfDataConfig::default() }
    #[inline] fn default_store_errors() -> bool { true }
    #[inline] fn default_store_evidences() -> bool { false }
    #[inline] fn default_recursion_semantics() -> RecursionSemantics { RecursionSemantics::default() }
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
        let c = ShaclConfig::default();
        assert_eq!(c.rdf_data(), &ShaclConfig::default_data_config());
        assert!(c.store_errors());
        assert!(!c.store_evidences());
        assert_eq!(c.recursion_semantics(), crate::validator::RecursionSemantics::Cautious);
    }

    #[test]
    fn builder_sets_recursion_semantics() {
        let c = ShaclConfig::default().with_recursion_semantics(crate::validator::RecursionSemantics::Brave);
        assert_eq!(c.recursion_semantics(), crate::validator::RecursionSemantics::Brave);
    }

    #[test]
    fn toml_configures_recursion_semantics() {
        let c: ShaclConfig = toml::from_str(r#"recursion_semantics = "brave""#).unwrap();
        assert_eq!(c.recursion_semantics(), crate::validator::RecursionSemantics::Brave);
    }

    #[test]
    fn builders_toggle_store_flags() {
        let c = ShaclConfig::default()
            .with_store_errors(false)
            .with_store_evidences(true);
        assert!(!c.store_errors());
        assert!(c.store_evidences());
    }

    #[test]
    fn toml_configures_store_flags() {
        let c: ShaclConfig = toml::from_str(
            r#"
            store_errors = false
            store_evidences = true
        "#,
        )
        .unwrap();
        assert!(!c.store_errors());
        assert!(c.store_evidences());
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
