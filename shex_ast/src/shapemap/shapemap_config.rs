use std::io::Read;
use std::path::Path;

use colored::*;
use prefixmap::PrefixMap;
use thiserror::Error;
use rudof_config::TomlConfig;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(default)]
pub struct ShapemapConfig {
    #[serde(rename = "nodes_pm", skip_serializing_if = "PrefixMap::is_empty")]
    pub(crate) nodes_prefixmap: PrefixMap,
    #[serde(rename = "shapes_pm", skip_serializing_if = "PrefixMap::is_empty")]
    pub(crate) shapes_prefixmap: PrefixMap,

    // TODO - Color stuff should be in rudof_lib

    #[serde(rename = "ok_text")]
    pub(crate) ok_text: String,
    #[serde(rename = "fail_text")]
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

    pub fn ok_text(&self) -> &String {
        &self.ok_text
    }

    pub fn fail_text(&self) -> &String {
        &self.fail_text
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
#[allow(dead_code)]
#[cfg_attr(rustfmt, rustfmt_skip)]
impl ShapemapConfig {
    #[inline] fn default_nodes_prefixmap() -> PrefixMap { PrefixMap::new() }
    #[inline] fn default_shapes_prefixmap() -> PrefixMap { PrefixMap::new() }
    #[inline] fn default_ok_text() -> String { "OK".to_string() }
    #[inline] fn default_fail_text() -> String { "FAIL".to_string() }
    #[inline] fn default_ok_color() -> Color { Color::Green }
    #[inline] fn default_fail_color() -> Color { Color::Green }
    #[inline] fn default_pending_color() -> Color { Color::Magenta }
}

impl Default for ShapemapConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl TomlConfig for ShapemapConfig {}



