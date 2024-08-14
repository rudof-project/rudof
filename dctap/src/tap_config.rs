use std::{collections::HashMap, path::Path};

use serde_derive::{Deserialize, Serialize};

use crate::TapError;

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct TapConfig {
    delimiter: Option<char>,
    quote: Option<char>,
    flexible: Option<bool>,
    picklist_delimiter: Option<char>,
    property_placeholders: HashMap<String, String>,
    empty_property_placeholder: Option<String>,
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
        match &self.picklist_delimiter {
            None => &'|',
            Some(c) => c,
        }
    }

    pub fn delimiter(&self) -> u8 {
        match self.delimiter {
            None => ',' as u8,
            Some(c) => c as u8,
        }
    }

    pub fn quote(&self) -> u8 {
        match self.quote {
            None => '"' as u8,
            Some(c) => c as u8,
        }
    }

    pub fn flexible(&self) -> bool {
        self.flexible.unwrap_or_else(|| true)
    }
}

impl Default for TapConfig {
    fn default() -> Self {
        Self {
            picklist_delimiter: None,
            delimiter: None,
            flexible: None,
            quote: None,
            empty_property_placeholder: None,
            property_placeholders: HashMap::new(),
        }
    }
}
