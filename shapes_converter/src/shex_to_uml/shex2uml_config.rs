use rudof_config::TomlConfig;
use rudof_iri::IriS;
use rudof_rdf::rdf_core::vocabs::RdfsVocab;
use serde::{Deserialize, Serialize};
use shex_validation::ShExConfig;
use std::{
    env,
    path::{Path, PathBuf},
};

use crate::shex_to_uml::{Direction, LineType};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ShEx2UmlConfig {
    #[serde(rename = "plantuml_path")]
    pub(crate) plantuml_path: PathBuf,

    /// A list of IRIs to use as annotation labels in the generated PlantUML diagram. If empty, the default is `rdfs:label`.
    #[serde(rename = "annotation_label", skip_serializing_if = "Vec::is_empty")]
    pub(crate) annotation_label: Vec<IriS>,

    /// Whether to replace IRIs by their labels in the generated PlantUML diagram. If `None`, the default is `true`.
    #[serde(rename = "replace_iri")]
    pub(crate) replace_iri_by_label: bool,

    /// Whether to use shadowing in the generated PlantUML diagram. If `None`, the default is `true`.
    #[serde(rename = "shadowing")]
    pub(crate) shadowing: bool,

    /// The line type to use in the generated PlantUML diagram. If `None`, the default is `LineType::Polyline`.
    #[serde(rename = "line_type")]
    pub(crate) line_type: LineType,

    /// The direction of the generated PlantUML diagram. If `None`, the default is `Direction::TopToBottom`.
    #[serde(rename = "direction")]
    pub(crate) direction: Direction,

    /// Configuration for ShEx. If `None`, the default configuration is used.
    #[serde(rename = "shex", skip_serializing)]
    pub(crate) shex: ShExConfig,
}

impl ShEx2UmlConfig {
    pub fn new() -> Self {
        Self {
            plantuml_path: Self::default_plantuml_path(),
            annotation_label: Self::default_annotation_label(),
            replace_iri_by_label: Self::default_replace_iri_by_label(),
            shadowing: Self::default_shadowing(),
            line_type: Self::default_line_type(),
            direction: Self::default_direction(),
            shex: Self::default_shex(),
        }
    }

    pub fn with_plantuml_path<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.plantuml_path = path.as_ref().to_path_buf();
        self
    }

    pub fn with_annotation_label(mut self, label: Vec<IriS>) -> Self {
        self.annotation_label = label;
        self
    }

    pub fn with_replace_iri_by_label(mut self, flag: bool) -> Self {
        self.replace_iri_by_label = flag;
        self
    }

    pub fn with_shadowing(mut self, flag: bool) -> Self {
        self.shadowing = flag;
        self
    }

    pub fn with_line_type(mut self, line_type: LineType) -> Self {
        self.line_type = line_type;
        self
    }

    pub fn with_direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    pub fn with_shex(mut self, cfg: ShExConfig) -> Self {
        self.shex = cfg;
        self
    }
}

impl ShEx2UmlConfig {
    pub fn plantuml_path(&self) -> &PathBuf {
        &self.plantuml_path
    }
    pub fn annotation_label(&self) -> Vec<&IriS> {
        self.annotation_label.iter().collect()
    }
    pub fn replace_iri_by_label(&self) -> bool {
        self.replace_iri_by_label
    }
    pub fn shadowing(&self) -> bool {
        self.shadowing
    }
    pub fn line_type(&self) -> &LineType {
        &self.line_type
    }
    pub fn direction(&self) -> &Direction {
        &self.direction
    }
    pub fn shex(&self) -> &ShExConfig {
        &self.shex
    }
}

/// Serde stuff
#[allow(dead_code)]
#[rustfmt::skip]
impl ShEx2UmlConfig {
    #[inline] fn default_plantuml_path() -> PathBuf { discover_puml_path(None) }
    #[inline] fn default_annotation_label() -> Vec<IriS> { vec![RdfsVocab::rdfs_label()] }
    #[inline] fn default_replace_iri_by_label() -> bool { false }
    #[inline] fn default_shadowing() -> bool { true }
    #[inline] fn default_line_type() -> LineType { LineType::default() }
    #[inline] fn default_direction() -> Direction { Direction::default() }
    #[inline] fn default_shex() -> ShExConfig { ShExConfig::default() }
}

impl Default for ShEx2UmlConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl TomlConfig for ShEx2UmlConfig {}

fn discover_puml_path(path: Option<PathBuf>) -> PathBuf {
    path.unwrap_or_else(|| match env::var("PLANTUML") {
        Ok(value) => Path::new(value.as_str()).to_path_buf(),
        Err(_) => Path::new("plantuml.jar").to_path_buf(),
    })
}

#[cfg(test)]
mod tests {
    use super::ShEx2UmlConfig;
    use rudof_config::TomlConfig;

    #[test]
    fn defaults() {
        let c = ShEx2UmlConfig::default();
        assert_eq!(c.replace_iri_by_label(), ShEx2UmlConfig::default_replace_iri_by_label());
        assert_eq!(c.shadowing(), ShEx2UmlConfig::default_shadowing());
    }

    #[test]
    fn partial_toml_fills_remaining_defaults() {
        let c = ShEx2UmlConfig::from_toml_str(r#"replace_iri = true"#).unwrap();
        assert!(c.replace_iri_by_label());
        assert_eq!(c.shadowing(), ShEx2UmlConfig::default_shadowing());
    }

    #[test]
    fn toml_round_trip() {
        let c = ShEx2UmlConfig::default().with_replace_iri_by_label(true);
        let s = c.to_toml_string().unwrap();
        let d = ShEx2UmlConfig::from_toml_str(&s).unwrap();
        assert_eq!(c, d);
    }

    /// The default `plantuml_path` must come from the `PLANTUML` env var:
    /// that's the variable documented for users (README, `docs/`, MCP setup)
    /// and referenced by the CLI's own error messages when the jar can't be
    /// found. It previously read a stale `RUDOF_PUML` var that no docs or
    /// error message ever mentioned, so setting `PLANTUML` was silently
    /// ignored.
    ///
    /// Both assertions live in one test (rather than two) so they can't race
    /// on the shared `PLANTUML` process env var if run in parallel with each
    /// other; no other test in this crate touches that var.
    #[test]
    fn plantuml_path_is_read_from_the_plantuml_env_var() {
        // SAFETY: test-only mutation of the process environment, scoped to
        // this single test.
        unsafe {
            std::env::remove_var("PLANTUML");
        }
        assert_eq!(
            super::discover_puml_path(None),
            std::path::PathBuf::from("plantuml.jar")
        );

        // SAFETY: test-only mutation of the process environment, scoped to
        // this single test.
        unsafe {
            std::env::set_var("PLANTUML", "/tmp/my-plantuml.jar");
        }
        assert_eq!(
            super::discover_puml_path(None),
            std::path::PathBuf::from("/tmp/my-plantuml.jar")
        );

        // SAFETY: test-only mutation of the process environment, scoped to
        // this single test.
        unsafe {
            std::env::remove_var("PLANTUML");
        }
    }
}
