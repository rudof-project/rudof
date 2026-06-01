use std::io::Read;
use std::path::Path;

use colored::*;
use prefixmap::PrefixMap;
use serde::{Deserialize, Serialize};
use thiserror::Error;


#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct ShapemapConfig {
    #[serde(rename = "nodes_prefixmap", default = "ShapemapConfig::default_nodes_prefixmap")]
    pub(crate) nodes_prefixmap: PrefixMap,
    #[serde(rename = "shapes_prefixmap", default = "ShapemapConfig::default_shapes_prefixmap")]
    pub(crate) shapes_prefixmap: PrefixMap,

    // TODO - Color stuff should be in rudof_lib

    #[serde(rename = "ok_text", default = "ShapemapConfig::default_ok_text")]
    pub(crate) ok_text: String,
    #[serde(rename = "fail_text", default = "ShapemapConfig::default_fail_text")]
    pub(crate) fail_text: String,

    #[serde(skip)]
    pub(crate) ok_color: Color,
    #[serde(skip)]
    pub(crate) fail_color: Color,
    #[serde(skip)]
    pub(crate) pending_color: Color,
}

impl ShapemapConfig {
    pub fn new() -> Self {
        Self {
            nodes_prefixmap: Self::default_nodes_prefixmap(),
            shapes_prefixmap: Self::default_shapes_prefixmap(),
            ok_text: Self::default_ok_text(),
            fail_text: Self::default_fail_text(),
            ok_color: Self::default_ok_color(),
            fail_color: Self::default_fail_color(),
            pending_color: Self::default_pending_color(),
        }
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, ShapemapConfigError> {
        let path_name = path.as_ref().display().to_string();
        let mut f = std::fs::File::open(path).map_err(|e| ShapemapConfigError::FromPath {
            path: path_name.clone(),
            error: e.to_string(),
        })?;
        let mut s = String::new();
        f.read_to_string(&mut s).map_err(|e| ShapemapConfigError::FromFile {
            file: path_name.clone(),
            error: e.to_string(),
        })?;

        let config: ShapemapConfig = toml::from_str(s.as_str()).map_err(|e| ShapemapConfigError::Toml {
            path: path_name.clone(),
            error: e.to_string(),
        })?;
        Ok(config)
    }

    pub fn with_nodes_prefixmap(mut self, pm: PrefixMap) -> Self {
        self.nodes_prefixmap = pm;
        self
    }

    pub fn with_shapes_prefixmap(mut self, pm: PrefixMap) -> Self {
        self.shapes_prefixmap = pm;
        self
    }

    pub fn with_ok_text(mut self, text: String) -> Self {
        self.ok_text = text;
        self
    }

    pub fn with_fail_text(mut self, text: String) -> Self {
        self.fail_text = text;
        self
    }

    pub fn with_ok_color(mut self, color: Color) -> Self {
        self.ok_color = color;
        self
    }

    pub fn with_fail_color(mut self, color: Color) -> Self {
        self.fail_color = color;
        self
    }

    pub fn with_pending_color(mut self, color: Color) -> Self {
        self.pending_color = color;
        self
    }
}

impl ShapemapConfig {
    pub fn nodes_prefixmap(&self) -> &PrefixMap {
        &self.nodes_prefixmap
    }

    pub fn shapes_prefixmap(&self) -> &PrefixMap {
        &self.shapes_prefixmap
    }

    pub fn ok_text(&self) -> String {
        self.ok_text
    }

    pub fn fail_text(&self) -> String {
        self.fail_text
    }

    pub fn ok_color(&self) -> &Color {
        &self.ok_color
    }

    pub fn fail_color(&self) -> &Color {
        &self.fail_color
    }

    pub fn pending_color(&self) -> &Color {
        &self.pending_color
    }
}

/// Serde stuff
impl ShapemapConfig {
    #[inline] fn default_nodes_prefixmap() -> PrefixMap { PrefixMap::new() }
    #[inline] fn default_shapes_prefixmap() -> PrefixMap { PrefixMap::new() }
    #[inline] fn default_ok_text() -> String { "OK".to_string() }
    #[inline] fn default_fail_text() -> String { "FAIL".to_string() }
    #[inline] fn default_ok_color() -> Color { Color::Green }
    #[inline] fn default_fail_color() -> Color { Color::Green }
    #[inline] fn default_pending_color() -> Color { Color::Magenta }
}

#[derive(Error, Debug, Clone)]
pub enum ShapemapConfigError {
    #[error("Error reading config file from path {path}: {error}")]
    FromPath { path: String, error: String },

    #[error("Error reading config file from file {file}: {error}")]
    FromFile { file: String, error: String },

    #[error("Error reading config file from path {path}: {error}")]
    Toml { path: String, error: String },
}
