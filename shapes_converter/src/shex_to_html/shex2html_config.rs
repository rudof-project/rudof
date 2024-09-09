use std::{
    fs, io,
    path::{Path, PathBuf},
};

use iri_s::IriS;
use serde::{Deserialize, Serialize};
use srdf::RDFS_LABEL_STR;
use thiserror::Error;

use crate::ShEx2UmlConfig;

pub const DEFAULT_COLOR_PROPERTY_NAME: &str = "blue";
pub const DEFAULT_LANDING_PAGE_NAME: &str = "index.html";
pub const DEFAULT_SHAPE_TEMPLATE_NAME: &str = "shape.html";

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ShEx2HtmlConfig {
    pub landing_page_name: String,
    pub shape_template_name: String,
    pub css_file_name: Option<String>,
    pub title: String,
    pub target_folder: Option<PathBuf>,
    pub color_property_name: Option<String>,
    pub replace_iri_by_label: Option<bool>,
    pub annotation_label: Vec<IriS>,
    pub embed_svg_schema: bool,
    pub embed_svg_shape: bool,
    pub shex2uml: Option<ShEx2UmlConfig>,
}

impl Default for ShEx2HtmlConfig {
    fn default() -> Self {
        Self {
            title: "ShEx schema".to_string(),
            landing_page_name: DEFAULT_LANDING_PAGE_NAME.to_string(),
            shape_template_name: DEFAULT_SHAPE_TEMPLATE_NAME.to_string(),
            css_file_name: Some("shex2html.css".to_string()),
            target_folder: None,
            color_property_name: Some(DEFAULT_COLOR_PROPERTY_NAME.to_string()),
            annotation_label: vec![IriS::new_unchecked(RDFS_LABEL_STR)],
            replace_iri_by_label: None,
            embed_svg_schema: true,
            embed_svg_shape: true,
            shex2uml: Some(ShEx2UmlConfig::new()),
        }
    }
}

impl ShEx2HtmlConfig {
    pub fn with_target_folder<P: AsRef<Path>>(mut self, target_folder: P) -> Self {
        self.target_folder = Some(target_folder.as_ref().to_path_buf());
        self
    }

    pub fn target_folder(&self) -> PathBuf {
        match &self.target_folder {
            Some(tf) => tf.to_owned(),
            None => Path::new(".").to_path_buf(),
        }
    }

    pub fn landing_page(&self) -> PathBuf {
        match &self.target_folder {
            Some(tf) => tf.as_path().join(self.landing_page_name.as_str()),
            None => Path::new(self.landing_page_name.as_str()).to_path_buf(),
        }
    }

    pub fn landing_page_name(&self) -> String {
        self.landing_page().to_string_lossy().to_string()
    }

    pub fn from_file(file_name: &str) -> Result<ShEx2HtmlConfig, ShEx2HtmlConfigError> {
        let config_str = fs::read_to_string(file_name).map_err(|e| {
            ShEx2HtmlConfigError::ReadingConfigError {
                path_name: file_name.to_string(),
                error: e,
            }
        })?;
        serde_yml::from_str::<ShEx2HtmlConfig>(&config_str).map_err(|e| {
            ShEx2HtmlConfigError::YamlError {
                path_name: file_name.to_string(),
                error: e,
            }
        })
    }

    pub fn shex2uml_config(&self) -> ShEx2UmlConfig {
        match &self.shex2uml {
            None => ShEx2UmlConfig::default(),
            Some(s) => s.clone(),
        }
    }
}

#[derive(Error, Debug)]
pub enum ShEx2HtmlConfigError {
    #[error("Reading path {path_name:?} error: {error:?}")]
    ReadingConfigError { path_name: String, error: io::Error },

    #[error("Reading YAML from {path_name:?}. Error: {error:?}")]
    YamlError {
        path_name: String,
        error: serde_yml::Error,
    },
}
