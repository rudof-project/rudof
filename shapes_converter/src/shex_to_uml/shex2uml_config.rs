use std::{
    env::{self, VarError},
    fs, io,
    path::PathBuf,
};

use iri_s::IriS;
use serde::{Deserialize, Serialize};
use shex_validation::ShExConfig;
use srdf::{PLANTUML, RDFS_LABEL_STR};
use thiserror::Error;

pub const DEFAULT_REPLACE_IRI_BY_LABEL: bool = true;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ShEx2UmlConfig {
    pub plantuml_path: Option<PathBuf>,
    pub annotation_label: Vec<IriS>,
    pub replace_iri_by_label: Option<bool>,
    pub shadowing: Option<bool>,
    pub shex: Option<ShExConfig>,
}

impl ShEx2UmlConfig {
    pub fn new() -> ShEx2UmlConfig {
        Self {
            annotation_label: vec![IriS::new_unchecked(RDFS_LABEL_STR)],
            replace_iri_by_label: None,
            shex: Some(ShExConfig::default()),
            shadowing: Some(true),
            plantuml_path: None,
        }
    }

    pub fn shex_config(&self) -> ShExConfig {
        match &self.shex {
            None => ShExConfig::default(),
            Some(sc) => sc.clone(),
        }
    }

    pub fn replace_iri_by_label(&self) -> bool {
        self.replace_iri_by_label
            .unwrap_or(DEFAULT_REPLACE_IRI_BY_LABEL)
    }

    pub fn from_file(file_name: &str) -> Result<ShEx2UmlConfig, ShEx2UmlConfigError> {
        let config_str =
            fs::read_to_string(file_name).map_err(|e| ShEx2UmlConfigError::ReadingConfigError {
                path_name: file_name.to_string(),
                error: e,
            })?;
        toml::from_str::<ShEx2UmlConfig>(&config_str).map_err(|e| ShEx2UmlConfigError::TomlError {
            path_name: file_name.to_string(),
            error: e,
        })
    }

    pub fn plantuml_path(&self) -> PathBuf {
        self.plantuml_path.clone().unwrap_or_else(|| {
            env::var(PLANTUML)
                .map(PathBuf::from)
                .unwrap_or_else(|_| env::current_dir().unwrap())
        })
    }
}

#[derive(Error, Debug)]
pub enum ShEx2UmlConfigError {
    #[error("Reading path {path_name:?} error: {error:?}")]
    ReadingConfigError { path_name: String, error: io::Error },

    #[error("Reading TOML from {path_name:?}. Error: {error:?}")]
    TomlError {
        path_name: String,
        error: toml::de::Error,
    },

    #[error("Accessing environment variable {var_name}: {error}")]
    EnvVarError { var_name: String, error: VarError },
}

impl Default for ShEx2UmlConfig {
    fn default() -> Self {
        Self::new()
    }
}
