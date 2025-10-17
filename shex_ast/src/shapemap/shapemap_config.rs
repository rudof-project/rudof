use std::io::Read;
use std::path::Path;

use colored::*;
use prefixmap::PrefixMap;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone, Default)]

pub struct ShapemapConfigMain {
    /// Specific shapemap configuration
    pub shex: Option<ShapemapConfig>,
}

impl ShapemapConfigMain {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<ShapemapConfigMain, ShapemapConfigError> {
        let path_name = path.as_ref().display().to_string();
        let mut f = std::fs::File::open(path).map_err(|e| ShapemapConfigError::FromPathError {
            path: path_name.clone(),
            error: e.to_string(),
        })?;
        let mut s = String::new();
        f.read_to_string(&mut s)
            .map_err(|e| ShapemapConfigError::FromFileError {
                file: path_name.clone(),
                error: e.to_string(),
            })?;

        let config: ShapemapConfigMain =
            toml::from_str(s.as_str()).map_err(|e| ShapemapConfigError::TomlError {
                path: path_name.clone(),
                error: e.to_string(),
            })?;
        Ok(config)
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]

pub struct ShapemapConfig {
    nodes_prefixmap: PrefixMap,
    shapes_prefixmap: PrefixMap,

    // TODO: Color doesn't implement Serialize/Deserialize...
    #[serde(skip)]
    ok_color: Option<Color>,

    ok_text: Option<String>,

    fail_text: Option<String>,

    #[serde(skip)]
    fail_color: Option<Color>,

    #[serde(skip)]
    pending_color: Option<Color>,
}

impl Default for ShapemapConfig {
    fn default() -> Self {
        Self {
            nodes_prefixmap: Default::default(),
            shapes_prefixmap: Default::default(),
            ok_color: Some(Color::Green),
            fail_color: Some(Color::Red),
            pending_color: Some(Color::Magenta),
            ok_text: Some("OK".to_string()),
            fail_text: Some("FAIL".to_string()),
        }
    }
}

impl ShapemapConfig {
    pub fn ok_text(&self) -> String {
        match &self.ok_text {
            None => "OK".to_string(),
            Some(t) => t.clone(),
        }
    }

    pub fn fail_text(&self) -> String {
        match &self.fail_text {
            None => "FAIL".to_string(),
            Some(t) => t.clone(),
        }
    }

    pub fn ok_color(&self) -> Option<Color> {
        self.ok_color
    }
    pub fn fail_color(&self) -> Option<Color> {
        self.fail_color
    }
    pub fn pending_color(&self) -> Option<Color> {
        self.pending_color
    }
    pub fn set_ok_color(&mut self, color: Color) {
        self.ok_color = Some(color);
    }

    pub fn set_fail_color(&mut self, color: Color) {
        self.fail_color = Some(color);
    }

    pub fn set_pending_color(&mut self, color: Color) {
        self.pending_color = Some(color)
    }

    pub fn nodes_prefixmap(&self) -> PrefixMap {
        self.nodes_prefixmap.clone()
    }

    pub fn shapes_prefixmap(&self) -> PrefixMap {
        self.shapes_prefixmap.clone()
    }

    pub fn with_nodes_prefixmap(mut self, prefixmap: &PrefixMap) -> Self {
        self.nodes_prefixmap = prefixmap.clone();
        self
    }

    pub fn with_shapes_prefixmap(mut self, prefixmap: &PrefixMap) -> Self {
        self.shapes_prefixmap = prefixmap.clone();
        self
    }
}

#[derive(Error, Debug, Clone)]
pub enum ShapemapConfigError {
    #[error("Error reading config file from path {path}: {error}")]
    FromPathError { path: String, error: String },

    #[error("Error reading config file from file {file}: {error}")]
    FromFileError { file: String, error: String },

    #[error("Error reading config file from path {path}: {error}")]
    TomlError { path: String, error: String },
}
