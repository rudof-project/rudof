use rudof_config::TomlConfig;
use rudof_rdf::rdf_core::RdfDataConfig;
use serde::{Deserialize, Serialize};
use shex_ast::ir::external_resolver::{ExternalShapeResolver, ExternalShapeResolverRegistry};
use shex_ast::shapemap::ShapemapConfig;
use std::sync::Arc;

use crate::ShExConfig;
use crate::typing::TypingObserver;

/// This struct can be used to customize the behavour of ShEx validators
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ValidatorConfig {
    /// Maximum numbers of validation steps
    #[serde(rename = "max_steps", skip_serializing_if = "Option::is_none")]
    pub(crate) max_steps: Option<usize>,

    /// Configuration of RDF data readers
    #[serde(rename = "rdf", skip_serializing)]
    pub(crate) rdf_data: RdfDataConfig,

    /// Configuration of ShEx schemas
    #[serde(rename = "shex", skip_serializing)]
    pub(crate) shex: ShExConfig,

    /// Configuration of Shapemaps
    #[serde(rename = "shapemap")]
    pub(crate) shapemap: ShapemapConfig,

    /// Whether to check the negation requirement (default: true)
    #[serde(rename = "check_negation")]
    pub(crate) check_negation_requirement: bool,

    /// Width for pretty printing
    // TODO - This should be in rudof_lib
    #[serde(rename = "width")]
    pub(crate) width: usize,

    /// Resolvers consulted for EXTERNAL shape expressions. Defaults to a
    /// registry containing only `RejectAllExternalResolver`. Resolvers cannot
    /// be loaded from TOML — they must be installed programmatically via
    /// [`Self::with_external_resolver`].
    #[serde(skip)]
    pub(crate) external_resolvers: ExternalShapeResolverRegistry,

    /// Observer notified whenever the ShEx engine caches a newly-proved
    /// `(node, shape)` result, for surfacing intermediate validation
    /// progress to the caller. Cannot be loaded from TOML — install it
    /// programmatically via [`Self::with_typing_observer`].
    #[serde(skip)]
    pub(crate) typing_observer: Option<Arc<TypingObserver>>,

    /// Whether to print each `(node, shape)` result as soon as it's cached,
    /// instead of only showing the final report (default: false). When set
    /// and no [`Self::with_typing_observer`] was installed explicitly, the
    /// validator installs a default console-printing observer.
    #[serde(rename = "show_intermediate_results")]
    pub(crate) show_intermediate_results: bool,

    /// Color used to print "conformant" in intermediate results (default:
    /// "green"). An unrecognized color name falls back to the default.
    #[serde(rename = "conformant_color")]
    pub(crate) conformant_color: String,

    /// Color used to print "non-conformant" in intermediate results
    /// (default: "red"). An unrecognized color name falls back to the
    /// default.
    #[serde(rename = "non_conformant_color")]
    pub(crate) non_conformant_color: String,
}

impl PartialEq for ValidatorConfig {
    fn eq(&self, other: &Self) -> bool {
        self.max_steps == other.max_steps
            && self.rdf_data == other.rdf_data
            && self.shex == other.shex
            && self.shapemap == other.shapemap
            && self.check_negation_requirement == other.check_negation_requirement
            && self.width == other.width
            && self.show_intermediate_results == other.show_intermediate_results
            && self.conformant_color == other.conformant_color
            && self.non_conformant_color == other.non_conformant_color
    }
}

impl ValidatorConfig {
    pub fn new() -> Self {
        Self {
            max_steps: Self::default_max_steps(),
            width: Self::default_width(),
            rdf_data: Self::default_rdf_data(),
            shex: Self::default_shex(),
            shapemap: Self::default_shapemap(),
            check_negation_requirement: Self::default_check_negation_requirement(),
            external_resolvers: ExternalShapeResolverRegistry::default(),
            typing_observer: None,
            show_intermediate_results: Self::default_show_intermediate_results(),
            conformant_color: Self::default_conformant_color(),
            non_conformant_color: Self::default_non_conformant_color(),
        }
    }

    pub fn with_max_steps(mut self, steps: Option<usize>) -> Self {
        self.max_steps = steps;
        self
    }

    pub fn with_rdf_data(mut self, cfg: RdfDataConfig) -> Self {
        self.rdf_data = cfg;
        self
    }

    pub fn with_shex(mut self, cfg: ShExConfig) -> Self {
        self.shex = cfg;
        self
    }

    pub fn with_shapemap(mut self, cfg: ShapemapConfig) -> Self {
        self.shapemap = cfg;
        self
    }

    pub fn with_check_negation_requirement(mut self, flag: bool) -> Self {
        self.check_negation_requirement = flag;
        self
    }

    pub fn with_width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }

    pub fn with_external_shape_resolver_registry(mut self, r: ExternalShapeResolverRegistry) -> Self {
        self.external_resolvers = r;
        self
    }

    /// Prepend a resolver to the EXTERNAL-shape resolver chain. Returns the
    /// updated config for builder-style chaining.
    pub fn with_external_resolver<R: ExternalShapeResolver + 'static>(mut self, r: R) -> Self {
        self.external_resolvers = std::mem::take(&mut self.external_resolvers).with_resolver(r);
        self
    }

    pub fn with_external_resolver_arc(mut self, r: Arc<dyn ExternalShapeResolver>) -> Self {
        self.external_resolvers = std::mem::take(&mut self.external_resolvers).with_resolver_arc(r);
        self
    }

    /// Install an observer notified on every newly-cached `(node, shape)`
    /// validation result. Returns the updated config for builder-style
    /// chaining.
    pub fn with_typing_observer(mut self, observer: Arc<TypingObserver>) -> Self {
        self.typing_observer = Some(observer);
        self
    }

    pub fn with_show_intermediate_results(mut self, flag: bool) -> Self {
        self.show_intermediate_results = flag;
        self
    }

    pub fn with_conformant_color(mut self, color: impl Into<String>) -> Self {
        self.conformant_color = color.into();
        self
    }

    pub fn with_non_conformant_color(mut self, color: impl Into<String>) -> Self {
        self.non_conformant_color = color.into();
        self
    }
}

impl ValidatorConfig {
    pub fn max_steps(&self) -> Option<usize> {
        self.max_steps
    }

    pub fn rdf_data(&self) -> &RdfDataConfig {
        &self.rdf_data
    }

    pub fn shex(&self) -> &ShExConfig {
        &self.shex
    }

    pub fn shapemap(&self) -> &ShapemapConfig {
        &self.shapemap
    }

    pub fn check_negation_requirement(&self) -> bool {
        self.check_negation_requirement
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn external_resolvers(&self) -> &ExternalShapeResolverRegistry {
        &self.external_resolvers
    }

    pub fn typing_observer(&self) -> Option<Arc<TypingObserver>> {
        self.typing_observer.clone()
    }

    pub fn show_intermediate_results(&self) -> bool {
        self.show_intermediate_results
    }

    pub fn conformant_color(&self) -> &str {
        &self.conformant_color
    }

    pub fn non_conformant_color(&self) -> &str {
        &self.non_conformant_color
    }
}

/// Serde stuff
#[allow(dead_code)]
#[rustfmt::skip]
impl ValidatorConfig {
    #[inline] fn default_max_steps() -> Option<usize> { None }
    #[inline] fn default_rdf_data() -> RdfDataConfig { RdfDataConfig::new() }
    #[inline] fn default_shex() -> ShExConfig { ShExConfig::new() }
    #[inline] fn default_shapemap() -> ShapemapConfig { ShapemapConfig::new() }
    #[inline] fn default_check_negation_requirement() -> bool { true }
    #[inline] fn default_width() -> usize { 80 }
    #[inline] fn default_show_intermediate_results() -> bool { false }
    #[inline] fn default_conformant_color() -> String { "green".to_string() }
    #[inline] fn default_non_conformant_color() -> String { "red".to_string() }
}

impl Default for ValidatorConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl TomlConfig for ValidatorConfig {}

#[cfg(test)]
mod tests {
    use super::ValidatorConfig;

    #[test]
    fn defaults() {
        let c = ValidatorConfig::default();
        assert_eq!(c.max_steps(), ValidatorConfig::default_max_steps());
        assert_eq!(
            c.check_negation_requirement(),
            ValidatorConfig::default_check_negation_requirement()
        );
        assert_eq!(c.width(), ValidatorConfig::default_width());
        assert!(!c.show_intermediate_results());
        assert_eq!(c.conformant_color(), "green");
        assert_eq!(c.non_conformant_color(), "red");
    }

    #[test]
    fn toml_configures_intermediate_results_and_colors() {
        let c: ValidatorConfig = toml::from_str(
            r#"
            show_intermediate_results = true
            conformant_color = "cyan"
            non_conformant_color = "magenta"
        "#,
        )
        .unwrap();
        assert!(c.show_intermediate_results());
        assert_eq!(c.conformant_color(), "cyan");
        assert_eq!(c.non_conformant_color(), "magenta");
    }

    #[test]
    fn partial_toml_fills_remaining_defaults() {
        let c: ValidatorConfig = toml::from_str(
            r#"
            max_steps = 7
            check_negation = false
        "#,
        )
        .unwrap();
        assert_eq!(c.max_steps(), Some(7));
        assert!(!c.check_negation_requirement());
        assert_eq!(c.width(), ValidatorConfig::default_width());
    }

    #[test]
    fn toml_round_trip() {
        let c = ValidatorConfig::default().with_max_steps(Some(42)).with_width(120);
        let s = toml::to_string(&c).unwrap();
        let d: ValidatorConfig = toml::from_str(&s).unwrap();
        assert_eq!(c, d);
    }
}
