use crate::validator::{FetchStrategy, RecursionSemantics};
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

    /// When `store_evidences` is on, whether to keep only the per-shape
    /// summary evidence (one per conforming `(node, shape)` pair with its
    /// own targets) and drop the finer-grained per-constraint-component
    /// evidence (default: false — keep everything). Has no effect when
    /// `store_evidences` is off. See [`crate::validator::report::EvidenceKind`].
    #[serde(rename = "evidence_shapes_only")]
    pub(crate) evidence_shapes_only: bool,

    /// Whether a shapes graph with a cyclic shape reference is accepted
    /// and, if so, how the cycle is resolved during validation (default:
    /// `lfp`/LFP). See [`RecursionSemantics`].
    #[serde(rename = "recursion_semantics")]
    pub(crate) recursion_semantics: RecursionSemantics,

    /// How data held by remote SPARQL endpoints is fetched (default:
    /// `on-demand`). See [`FetchStrategy`].
    #[serde(rename = "fetch_strategy")]
    pub(crate) fetch_strategy: FetchStrategy,

    /// With the `by-shape` fetch strategy, how many nodes one query asks
    /// about (default: 50).
    #[serde(rename = "prefetch_batch_size")]
    pub(crate) prefetch_batch_size: usize,

    /// With the `by-shape` fetch strategy, how many shape references deep
    /// the data is fetched in advance from the target shapes (default: 16).
    /// Deeper data is fetched on demand.
    #[serde(rename = "prefetch_max_rounds")]
    pub(crate) prefetch_max_rounds: usize,
}

impl ShaclConfig {
    pub fn new() -> Self {
        Self {
            data: Self::default_data_config(),
            store_errors: Self::default_store_errors(),
            store_evidences: Self::default_store_evidences(),
            evidence_shapes_only: Self::default_evidence_shapes_only(),
            recursion_semantics: Self::default_recursion_semantics(),
            fetch_strategy: Self::default_fetch_strategy(),
            prefetch_batch_size: Self::default_prefetch_batch_size(),
            prefetch_max_rounds: Self::default_prefetch_max_rounds(),
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

    pub fn with_evidence_shapes_only(mut self, flag: bool) -> Self {
        self.evidence_shapes_only = flag;
        self
    }

    pub fn with_recursion_semantics(mut self, semantics: RecursionSemantics) -> Self {
        self.recursion_semantics = semantics;
        self
    }

    pub fn with_fetch_strategy(mut self, strategy: FetchStrategy) -> Self {
        self.fetch_strategy = strategy;
        self
    }

    pub fn with_prefetch_batch_size(mut self, size: usize) -> Self {
        self.prefetch_batch_size = size.max(1);
        self
    }

    pub fn with_prefetch_max_rounds(mut self, rounds: usize) -> Self {
        self.prefetch_max_rounds = rounds;
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

    pub fn evidence_shapes_only(&self) -> bool {
        self.evidence_shapes_only
    }

    pub fn recursion_semantics(&self) -> RecursionSemantics {
        self.recursion_semantics
    }

    pub fn fetch_strategy(&self) -> FetchStrategy {
        self.fetch_strategy
    }

    pub fn prefetch_batch_size(&self) -> usize {
        self.prefetch_batch_size.max(1)
    }

    pub fn prefetch_max_rounds(&self) -> usize {
        self.prefetch_max_rounds
    }
}

/// Serde stuff
#[allow(dead_code)]
#[rustfmt::skip]
impl ShaclConfig {
    #[inline] fn default_data_config() -> RdfDataConfig { RdfDataConfig::default() }
    #[inline] fn default_store_errors() -> bool { true }
    #[inline] fn default_store_evidences() -> bool { false }
    #[inline] fn default_evidence_shapes_only() -> bool { false }
    #[inline] fn default_recursion_semantics() -> RecursionSemantics { RecursionSemantics::default() }
    #[inline] fn default_fetch_strategy() -> FetchStrategy { FetchStrategy::default() }
    #[inline] fn default_prefetch_batch_size() -> usize { 50 }
    #[inline] fn default_prefetch_max_rounds() -> usize { 16 }
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
        assert!(!c.evidence_shapes_only());
        assert_eq!(c.recursion_semantics(), crate::validator::RecursionSemantics::Lfp);
    }

    #[test]
    fn builder_sets_recursion_semantics() {
        let c = ShaclConfig::default().with_recursion_semantics(crate::validator::RecursionSemantics::Gfp);
        assert_eq!(c.recursion_semantics(), crate::validator::RecursionSemantics::Gfp);
    }

    #[test]
    fn toml_configures_recursion_semantics() {
        let c: ShaclConfig = toml::from_str(r#"recursion_semantics = "gfp""#).unwrap();
        assert_eq!(c.recursion_semantics(), crate::validator::RecursionSemantics::Gfp);
    }

    #[test]
    fn toml_configures_fetch_strategy() {
        let c: ShaclConfig = toml::from_str(
            r#"
            fetch_strategy = "by-shape"
            prefetch_batch_size = 10
        "#,
        )
        .unwrap();
        assert_eq!(c.fetch_strategy(), crate::validator::FetchStrategy::ByShape);
        assert_eq!(c.prefetch_batch_size(), 10);
        assert_eq!(c.prefetch_max_rounds(), 16);
        assert_eq!(
            ShaclConfig::default().fetch_strategy(),
            crate::validator::FetchStrategy::OnDemand
        );
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
    fn builder_sets_evidence_shapes_only() {
        let c = ShaclConfig::default().with_evidence_shapes_only(true);
        assert!(c.evidence_shapes_only());
    }

    #[test]
    fn toml_configures_evidence_shapes_only() {
        let c: ShaclConfig = toml::from_str(r#"evidence_shapes_only = true"#).unwrap();
        assert!(c.evidence_shapes_only());
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
