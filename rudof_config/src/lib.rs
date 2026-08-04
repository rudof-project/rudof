use serde::Serialize;
use serde::de::DeserializeOwned;
#[cfg(not(target_family = "wasm"))]
use std::path::Path;
use thiserror::Error;

#[cfg(not(target_family = "wasm"))]
mod discovery;
#[cfg(not(target_family = "wasm"))]
pub use discovery::{find_config_files_from, merge_tables, read_toml_table, user_config_file};

/// Errors produced by configuration operations
#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Error reading config file from {location}: {error}")]
    Read { location: String, error: String },
    #[error("Error parsing TOML config from {location}: {error}")]
    Parse { location: String, error: String },
    #[error("Error serializing config to TOML: {error}")]
    Serialize { error: String },
    #[error(
        "Incompatible config version: the config targets rudof {config}, but this is rudof {rudof}. \
         Upgrade rudof to at least {config}, or update the config's `version`."
    )]
    IncompatibleVersion { config: String, rudof: String },
}

/// Configuration trait for TOML-backed configuration structs
pub trait TomlConfig: Sized + Default + Serialize + DeserializeOwned {
    /// Parses a configuration from a TOML string
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError::Parse`] if the string is not valid TOML for this type
    fn from_toml_str(s: &str) -> Result<Self, ConfigError> {
        toml::from_str(s).map_err(|e| ConfigError::Parse {
            location: "<string>".to_string(),
            error: e.to_string(),
        })
    }

    /// Loads a configuration from a TOML file at `path`
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError::Read`] if the file cannot be read, or
    /// [`ConfigError::Parse`] if its contents are not valid TOML for this type
    #[cfg(not(target_family = "wasm"))]
    fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let path = path.as_ref();
        let location = path.display().to_string();
        let contents = std::fs::read_to_string(path).map_err(|e| ConfigError::Read {
            location: location.clone(),
            error: e.to_string(),
        })?;
        Self::from_toml_str(&contents).map_err(|e| match e {
            ConfigError::Parse { error, .. } => ConfigError::Parse { location, error },
            other => other,
        })
    }

    /// Serializes this configuration to a TOML string
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError::Serialize`] if serialization fails
    fn to_toml_string(&self) -> Result<String, ConfigError> {
        toml::to_string(self).map_err(|e| ConfigError::Serialize { error: e.to_string() })
    }
}
