use std::{
    fs, io,
    path::{Path, PathBuf},
};
use std::io::Read;
use rudof_iri::IriS;
use rudof_rdf::rdf_core::vocabs::RdfsVocab;
use serde::{Deserialize, Deserializer};
use shex_validation::ShExConfig;
use thiserror::Error;

use crate::ShEx2UmlConfig;

#[derive(PartialEq, Debug, Clone)]
pub struct ShEx2HtmlConfig {
    pub(crate) title: String,
    pub(crate) landing_page_name: String,
    pub(crate) shape_template_name: String,
    pub(crate) template_folder: Option<String>,
    pub(crate) css_file_name: String,
    pub(crate) target_folder: PathBuf,
    pub(crate) color_property_name: String,
    pub(crate) annotation_label: Vec<IriS>,
    pub(crate) replace_iri_by_label: bool,
    pub(crate) embed_svg_schema: bool,
    pub(crate) embed_svg_shape: bool,
    pub(crate) shex2uml: ShEx2UmlConfig,
    shex2uml_needs_fixup: bool,
    pub(crate) shex: ShExConfig, // TODO - Maybe remove, a copy of ShexConfig is in Shex2umlConfig
    shex_needs_fixup: bool,
}

impl ShEx2HtmlConfig {
    pub fn new() -> Self {
        Self {
            title: Self::default_title(),
            landing_page_name: Self::default_landing_page_name(),
            shape_template_name: Self::default_shape_template_name(),
            template_folder: Self::default_template_folder(),
            css_file_name: Self::default_css_file_name(),
            target_folder: Self::default_target_folder(),
            color_property_name: Self::default_color_property_name(),
            annotation_label: Self::default_annotation_label(),
            replace_iri_by_label: Self::default_replace_iri_by_label(),
            embed_svg_schema: Self::default_embed_svg_schema(),
            embed_svg_shape: Self::default_embed_svg_shape(),
            shex2uml: Self::default_shex2uml(),
            shex2uml_needs_fixup: false,
            shex: Self::default_shex(),
            shex_needs_fixup: false,
        }
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, ShEx2HtmlConfigError> {
        let path_name = path.as_ref().display().to_string();
        let mut f = fs::File::open(path).map_err(|e| ShEx2HtmlConfigError::ReadingConfigError {
            path_name: path_name.clone(),
            error: e,
        })?;
        let mut s = String::new();
        f.read_to_string(&mut s)
            .map_err(|e| ShEx2HtmlConfigError::ReadingConfigError {
                path_name: path_name.clone(),
                error: e,
            })?;

        let config: ShEx2HtmlConfig =
            toml::from_str(s.as_str()).map_err(|e| ShEx2HtmlConfigError::TomlError {
                path_name: path_name.clone(),
                error: e,
            })?;
        Ok(config)
    }

    pub fn with_title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    pub fn with_landing_page_name(mut self, name: String) -> Self {
        self.landing_page_name = name;
        self
    }

    pub fn with_shape_template_name(mut self, name: String) -> Self {
        self.shape_template_name = name;
        self
    }

    pub fn with_template_folder(mut self, folder: Option<String>) -> Self {
        self.template_folder = folder;
        self
    }

    pub fn with_css_file_name(mut self, name: String) -> Self {
        self.css_file_name = name;
        self
    }

    pub fn with_target_folder<P: AsRef<Path>>(mut self, folder: P) -> Self {
        self.target_folder = folder.as_ref().to_path_buf();
        self
    }

    pub fn with_color_property_name(mut self, color: String) -> Self {
        self.color_property_name = color;
        self
    }

    pub fn with_annotation_label(mut self, labels: Vec<IriS>) -> Self {
        self.annotation_label = labels;
        self
    }

    pub fn with_replace_iri_by_label(mut self, flag: bool) -> Self {
        self.replace_iri_by_label = flag;
        self
    }

    pub fn with_embed_svg_schema(mut self, flag: bool) -> Self {
        self.embed_svg_schema = flag;
        self
    }

    pub fn with_embed_svg_shape(mut self, flag: bool) -> Self {
        self.embed_svg_shape = flag;
        self
    }

    pub fn with_shex2uml(mut self, cfg: ShEx2UmlConfig) -> Self {
        self.shex2uml = cfg;
        self
    }

    pub fn with_shex(mut self, cfg: ShExConfig) -> Self {
        self.shex = cfg;
        self
    }
}

impl ShEx2HtmlConfig {
    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn landing_page_name(&self) -> &String {
        &self.landing_page_name
    }

    pub fn shape_template_name(&self) -> &String {
        &self.shape_template_name
    }

    pub fn template_folder(&self) -> Option<&String> {
        self.template_folder.as_ref()
    }

    pub fn css_file_name(&self) -> &String {
        &self.css_file_name
    }

    pub fn target_folder(&self) -> &PathBuf {
        &self.target_folder
    }

    pub fn color_property_name(&self) -> &String {
        &self.color_property_name
    }

    pub fn annotation_label(&self) -> &Vec<IriS> {
        self.annotation_label.as_ref()
    }

    pub fn replace_iri_by_label(&self) -> bool {
        self.replace_iri_by_label
    }

    pub fn embed_svg_schema(&self) -> bool {
        self.embed_svg_schema
    }

    pub fn embed_svg_shape(&self) -> bool {
        self.embed_svg_shape
    }

    pub fn shex2uml(&self) -> &ShEx2UmlConfig {
        &self.shex2uml
    }

    /// Get the ShEx config
    pub fn shex(&self) -> &ShExConfig {
        &self.shex
    }
}

impl ShEx2HtmlConfig {
    pub fn landing_page(&self) -> PathBuf {
        self.target_folder.as_path().join(self.landing_page_name.as_str())
    }
}

/// Serde stuff
#[allow(dead_code)]
#[cfg_attr(rustfmt, rustfmt_skip)]
impl ShEx2HtmlConfig {
    #[inline] fn default_title() -> String { "ShEx schema".to_string() }
    #[inline] fn default_landing_page_name() -> String { "index.html".to_string() }
    #[inline] fn default_shape_template_name() -> String { "shape.html".to_string() }
    #[inline] fn default_template_folder() -> Option<String> { None }
    #[inline] fn default_css_file_name() -> String { "shex2html.css".to_string() }
    #[inline] fn default_target_folder() -> PathBuf { Path::new(".").to_path_buf() }
    #[inline] fn default_color_property_name() -> String { "blue".to_string() }
    #[inline] fn default_annotation_label() -> Vec<IriS> { vec![ RdfsVocab::rdfs_label() ] }
    #[inline] fn default_replace_iri_by_label() -> bool { true }
    #[inline] fn default_embed_svg_schema() -> bool { true }
    #[inline] fn default_embed_svg_shape() -> bool { true }
    #[inline] fn default_shex2uml() -> ShEx2UmlConfig { ShEx2UmlConfig::default() }
    #[inline] fn default_shex() -> ShExConfig { ShExConfig::default() }
    pub fn fixup(&mut self, shex: ShExConfig, shex2uml: ShEx2UmlConfig) {
        if self.shex_needs_fixup {
            self.shex_needs_fixup = false;
            self.shex = shex;
        }

        if self.shex2uml_needs_fixup {
            self.shex2uml_needs_fixup = false;
            self.shex2uml = shex2uml;
        }
    }
}

impl<'de> Deserialize<'de> for ShEx2HtmlConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        #[derive(Deserialize)]
        struct Raw {
            #[serde(rename = "title", default = "ShEx2HtmlConfig::default_title")]
            title: String,
            #[serde(rename = "landing_page", default = "ShEx2HtmlConfig::default_landing_page_name")]
            landing_page_name: String,
            #[serde(rename = "shape_template", default = "ShEx2HtmlConfig::default_shape_template_name")]
            shape_template_name: String,
            #[serde(rename = "template_folder", default = "ShEx2HtmlConfig::default_template_folder")]
            template_folder: Option<String>,
            #[serde(rename = "css_file", default = "ShEx2HtmlConfig::default_css_file_name")]
            css_file_name: String,
            #[serde(rename = "target_folder", default = "ShEx2HtmlConfig::default_target_folder")]
            target_folder: PathBuf,
            #[serde(rename = "property_color", default = "ShEx2HtmlConfig::default_color_property_name")]
            color_property_name: String,
            #[serde(rename = "annotation_label", default = "ShEx2HtmlConfig::default_annotation_label")]
            annotation_label: Vec<IriS>,
            #[serde(rename = "replace_iri_by_label", default = "ShEx2HtmlConfig::default_replace_iri_by_label")]
            replace_iri_by_label: bool,
            #[serde(rename = "embed_svg_schema", default = "ShEx2HtmlConfig::default_embed_svg_schema")]
            embed_svg_schema: bool,
            #[serde(rename = "embed_svg_shape", default = "ShEx2HtmlConfig::default_embed_svg_shape")]
            embed_svg_shape: bool,
            #[serde(rename = "shex2uml", default)]
            shex2uml: Option<ShEx2UmlConfig>,
            #[serde(rename = "shex", default)]
            shex: Option<ShExConfig>,
        }

        let raw = Raw::deserialize(deserializer)?;

        Ok(Self {
            title: raw.title,
            landing_page_name: raw.landing_page_name,
            shape_template_name: raw.shape_template_name,
            template_folder: raw.template_folder,
            css_file_name: raw.css_file_name,
            target_folder: raw.target_folder,
            color_property_name: raw.color_property_name,
            annotation_label: raw.annotation_label,
            replace_iri_by_label: raw.replace_iri_by_label,
            embed_svg_schema: raw.embed_svg_schema,
            embed_svg_shape: raw.embed_svg_shape,
            shex2uml_needs_fixup: raw.shex2uml.is_none(),
            shex2uml: raw.shex2uml.unwrap_or(Self::default_shex2uml()),
            shex_needs_fixup: raw.shex.is_none(),
            shex: raw.shex.unwrap_or(Self::default_shex())
        })
    }
}

impl Default for ShEx2HtmlConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Error, Debug)]
pub enum ShEx2HtmlConfigError {
    #[error("Reading path {path_name:?} error: {error:?}")]
    ReadingConfigError { path_name: String, error: io::Error },

    #[error("Reading TOML from {path_name:?}. Error: {error:?}")]
    TomlError { path_name: String, error: toml::de::Error },
}
