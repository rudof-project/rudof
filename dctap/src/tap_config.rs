use std::path::Path;

use serde_derive::{Deserialize, Serialize};

use crate::TapError;

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct TapConfig {
    delimiter: char,
    quote: char,
    flexible: bool,
    picklist_delimiter: char,
}

impl TapConfig {
    /// Obtain a TapConfig from a path file in YAML
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<TapConfig, TapError> {
        let path_name = path.as_ref().display().to_string();
        let f = std::fs::File::open(path).map_err(|e| TapError::TapConfigFromPathError {
            path: path_name.clone(),
            error: e,
        })?;
        let config: TapConfig =
            serde_yml::from_reader(f).map_err(|e| TapError::TapConfigYamlError {
                path: path_name.clone(),
                error: e,
            })?;
        Ok(config)
    }

    pub fn picklist_delimiter(&self) -> &char {
        &self.picklist_delimiter
    }

    pub fn delimiter(&self) -> u8 {
        self.delimiter as u8
    }

    pub fn quote(&self) -> u8 {
        self.quote as u8
    }

    pub fn flexible(&self) -> bool {
        self.flexible
    }
}

impl Default for TapConfig {
    fn default() -> Self {
        Self {
            picklist_delimiter: '|',
            delimiter: ',',
            flexible: true,
            quote: '"',
        }
    }
}
