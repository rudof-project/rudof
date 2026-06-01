use std::{
    env::{self, VarError},
    fs, io,
    path::PathBuf,
};
use std::io::Read;
use std::path::Path;
use rudof_iri::IriS;
use rudof_rdf::rdf_core::vocabs::RdfsVocab;
use serde::{Deserialize, Deserializer};
use shex_validation::ShExConfig;
use thiserror::Error;

use crate::shex_to_uml::{Direction, LineType};

#[derive(PartialEq, Debug, Clone)]
pub struct ShEx2UmlConfig {
    pub(crate) plantuml_path: Option<PathBuf>,

    /// A list of IRIs to use as annotation labels in the generated PlantUML diagram. If empty, the default is `rdfs:label`.
    pub(crate) annotation_label: Vec<IriS>,

    /// Whether to replace IRIs by their labels in the generated PlantUML diagram. If `None`, the default is `true`.
    pub(crate) replace_iri_by_label: bool,

    /// Whether to use shadowing in the generated PlantUML diagram. If `None`, the default is `true`.
    pub(crate) shadowing: bool,

    /// The line type to use in the generated PlantUML diagram. If `None`, the default is `LineType::Polyline`.
    pub(crate) line_type: LineType,

    /// The direction of the generated PlantUML diagram. If `None`, the default is `Direction::TopToBottom`.
    pub(crate) direction: Direction,

    /// Configuration for ShEx. If `None`, the default configuration is used.
    pub(crate) shex: ShExConfig,
    shex_needs_fixup: bool,
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
            shex_needs_fixup: false,
            shex: Self::default_shex()
        }
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<ShEx2UmlConfig, ShEx2UmlConfigError> {
        let path_name = path.as_ref().display().to_string();
        let mut f = fs::File::open(path).map_err(|e| ShEx2UmlConfigError::ReadingConfig {
            path_name: path_name.clone(),
            error: e,
        })?;
        let mut s = String::new();
        f.read_to_string(&mut s)
            .map_err(|e| ShEx2UmlConfigError::ReadingConfig {
                path_name: path_name.clone(),
                error: e,
            })?;

        let config: ShEx2UmlConfig =
            toml::from_str(s.as_str()).map_err(|e| ShEx2UmlConfigError::Toml {
                path_name: path_name.clone(),
                error: e,
            })?;
        Ok(config)
    }

    pub fn with_plantuml_path(mut self, path: Option<PathBuf>) -> Self {
        self.plantuml_path = path;
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
    pub fn plantuml_path(&self) -> Option<&PathBuf> {
        self.plantuml_path.as_ref()
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
impl ShEx2UmlConfig {
    #[inline] fn default_plantuml_path() -> Option<PathBuf> { None }
    #[inline] fn default_annotation_label() -> Vec<IriS> { Vec::default() }
    #[inline] fn default_replace_iri_by_label() -> bool { true }
    #[inline] fn default_shadowing() -> bool { true }
    #[inline] fn default_line_type() -> LineType { LineType::default() }
    #[inline] fn default_direction() -> Direction { Direction::default() }
    #[inline] fn default_shex() -> ShExConfig { ShExConfig::default() }

    pub fn fixup(&mut self, cfg: ShExConfig) {
        if self.shex_needs_fixup {
            self.shex = cfg;
        }
    }
}

impl<'de> Deserialize<'de> for ShEx2UmlConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        #[derive(Deserialize)]
        struct Raw {
            #[serde(rename = "plantuml_path", default = "ShEx2UmlConfig::default_plantuml_path")]
            plantuml_path: Option<PathBuf>,
            #[serde(rename = "annotation_label", default = "ShEx2UmlConfig::default_annotation_label")]
            annotation_label: Vec<IriS>,
            #[serde(rename = "replace_iri", default = "ShEx2UmlConfig::default_replace_iri_by_label")]
            replace_iri_by_label: bool,
            #[serde(rename = "shadowing", default = "ShEx2UmlConfig::default_shadowing")]
            shadowing: bool,
            #[serde(rename = "line_type", default = "ShEx2UmlConfig::default_line_type")]
            line_type: LineType,
            #[serde(rename = "direction", default = "ShEx2UmlConfig::default_direction")]
            direction: Direction,
            #[serde(rename = "shex", default)]
            shex: Option<ShExConfig>,
        }

        let raw = Raw::deserialize(deserializer)?;

        Ok(Self {
            plantuml_path: raw.plantuml_path,
            annotation_label: raw.annotation_label,
            replace_iri_by_label: raw.replace_iri_by_label,
            shadowing: raw.shadowing,
            line_type: raw.line_type,
            direction: raw.direction,
            shex_needs_fixup: raw.shex.is_none(),
            shex: raw.shex.unwrap_or(Self::default_shex()),
        })
    }
}

impl Default for ShEx2UmlConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Error, Debug)]
pub enum ShEx2UmlConfigError {
    #[error("Reading path {path_name:?} error: {error:?}")]
    ReadingConfig { path_name: String, error: io::Error },

    #[error("Reading TOML from {path_name:?}. Error: {error:?}")]
    Toml { path_name: String, error: toml::de::Error },

    #[error("Accessing environment variable {var_name}: {error}")]
    EnvVar { var_name: String, error: VarError },
}
