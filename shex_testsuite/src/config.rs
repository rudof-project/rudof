use crate::manifest_mode::ManifestMode;
use serde::{Deserialize, Serialize};
use std::{fs, io};
use thiserror::Error;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Config {
    pub manifest_mode: ManifestMode,
    pub excluded_entries: Vec<String>,
    pub single_entries: Option<Vec<String>>,
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Reading path {path_name:?} error: {error:?}")]
    ReadingConfigError { path_name: String, error: io::Error },

    #[error("Reading YAML from {path_name:?}. Error: {error:?}")]
    YamlError {
        path_name: String,
        error: serde_yml::Error,
    },
}

impl Config {
    pub fn from_file(file_name: &str) -> Result<Config, ConfigError> {
        let config_str =
            fs::read_to_string(file_name).map_err(|e| ConfigError::ReadingConfigError {
                path_name: file_name.to_string(),
                error: e,
            })?;
        serde_yml::from_str::<Config>(&config_str).map_err(|e| ConfigError::YamlError {
            path_name: file_name.to_string(),
            error: e,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example() {
        let sample_str = r#"
         { "manifest_mode": "Schemas", 
           "excluded_entries": ["entry1", "entry2"] 
         }
        "#;
        let sample = Config {
            manifest_mode: ManifestMode::Schemas,
            excluded_entries: vec!["entry1".to_string(), "entry2".to_string()],
            single_entries: None,
        };
        let sample_parsed = serde_json::from_str::<Config>(sample_str).unwrap();
        assert_eq!(sample_parsed, sample);
    }
}
