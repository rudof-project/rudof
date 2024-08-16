use std::{
    env::{self, VarError},
    fs, io,
    path::{Path, PathBuf},
};

use iri_s::IriS;
use serde::{Deserialize, Serialize};
use srdf::RDFS_LABEL_STR;
use thiserror::Error;

pub const PLANTUML: &str = "PLANTUML";

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ShEx2UmlConfig {
    pub plantuml_path: Option<PathBuf>,
    pub annotation_label: Vec<IriS>,
}

impl ShEx2UmlConfig {
    pub fn new() -> ShEx2UmlConfig {
        let plantuml_path = match env::var(PLANTUML) {
            Ok(value) => Some(Path::new(value.as_str()).to_path_buf()),
            Err(_) => None,
        };
        Self {
            plantuml_path,
            annotation_label: vec![IriS::new_unchecked(RDFS_LABEL_STR)],
        }
    }

    pub fn from_file(file_name: &str) -> Result<ShEx2UmlConfig, ShEx2UmlConfigError> {
        let config_str =
            fs::read_to_string(file_name).map_err(|e| ShEx2UmlConfigError::ReadingConfigError {
                path_name: file_name.to_string(),
                error: e,
            })?;
        serde_yml::from_str::<ShEx2UmlConfig>(&config_str).map_err(|e| {
            ShEx2UmlConfigError::YamlError {
                path_name: file_name.to_string(),
                error: e,
            }
        })
    }

    pub fn with_plantuml_path<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.plantuml_path = Some(path.as_ref().to_owned());
        self
    }
}

#[derive(Error, Debug)]
pub enum ShEx2UmlConfigError {
    #[error("Reading path {path_name:?} error: {error:?}")]
    ReadingConfigError { path_name: String, error: io::Error },

    #[error("Reading YAML from {path_name:?}. Error: {error:?}")]
    YamlError {
        path_name: String,
        error: serde_yml::Error,
    },

    #[error("Accessing environment variable {var_name}: {error}")]
    EnvVarError { var_name: String, error: VarError },
}

impl Default for ShEx2UmlConfig {
    fn default() -> Self {
        Self::new()
    }
}
