use std::{
    fs, io,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ShEx2HtmlConfig {
    pub landing_page_name: String,
    pub css_file_name: Option<String>,
    pub title: String,
    pub target_folder: PathBuf,
    pub color_property_name: Option<String>,
}

impl Default for ShEx2HtmlConfig {
    fn default() -> Self {
        Self {
            title: "Generated shapes".to_string(),
            landing_page_name: "index.html".to_string(),
            css_file_name: Some("shex2html.css".to_string()),
            target_folder: PathBuf::new(),
            color_property_name: Some("blue".to_string()),
        }
    }
}

impl ShEx2HtmlConfig {
    pub fn with_target_folder<P: AsRef<Path>>(mut self, target_folder: P) -> Self {
        self.target_folder = target_folder.as_ref().to_path_buf();
        self
    }

    pub fn landing_page(&self) -> PathBuf {
        self.target_folder
            .as_path()
            .join(self.landing_page_name.as_str())
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
