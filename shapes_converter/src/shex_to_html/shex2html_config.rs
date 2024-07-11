use std::{fs, io};

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ShEx2HtmlConfig {
    pub landing_page_name: String,
    pub title: String,
}

impl Default for ShEx2HtmlConfig {
    fn default() -> Self {
        Self {
            title: "Generated shapes".to_string(),
            landing_page_name: "index.html".to_string(),
        }
    }
}

impl ShEx2HtmlConfig {
    pub fn from_file(file_name: &str) -> Result<ShEx2HtmlConfig, ShEx2UmlConfigError> {
        let config_str =
            fs::read_to_string(file_name).map_err(|e| ShEx2UmlConfigError::ReadingConfigError {
                path_name: file_name.to_string(),
                error: e,
            })?;
        serde_yaml::from_str::<ShEx2HtmlConfig>(&config_str).map_err(|e| {
            ShEx2UmlConfigError::YamlError {
                path_name: file_name.to_string(),
                error: e,
            }
        })
    }
}

#[derive(Error, Debug)]
pub enum ShEx2UmlConfigError {
    #[error("Reading path {path_name:?} error: {error:?}")]
    ReadingConfigError { path_name: String, error: io::Error },

    #[error("Reading YAML from {path_name:?}. Error: {error:?}")]
    YamlError {
        path_name: String,
        error: serde_yaml::Error,
    },
}
