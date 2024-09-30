use std::{collections::HashMap, path::Path};

use serde_derive::{Deserialize, Serialize};

use crate::{PlaceholderResolver, TapError};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone, Default)]
pub struct DCTapConfig {
    pub dctap: Option<TapConfig>,
}

impl DCTapConfig {
    /// Obtain a DCTapConfig from a path file in YAML
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<DCTapConfig, TapError> {
        let path_name = path.as_ref().display().to_string();
        let f = std::fs::File::open(path).map_err(|e| TapError::TapConfigFromPathError {
            path: path_name.clone(),
            error: e,
        })?;
        let config: DCTapConfig =
            serde_yml::from_reader(f).map_err(|e| TapError::TapConfigYamlError {
                path: path_name.clone(),
                error: e,
            })?;
        Ok(config)
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone, Default)]
pub struct TapConfig {
    delimiter: Option<char>,
    quote: Option<char>,
    flexible: Option<bool>,
    picklist_delimiter: Option<char>,
    property_placeholders: HashMap<String, PlaceholderResolver>,
    empty_property_placeholder: Option<PlaceholderResolver>,
}

impl TapConfig {
    pub fn picklist_delimiter(&self) -> &char {
        match &self.picklist_delimiter {
            None => &'|',
            Some(c) => c,
        }
    }

    pub fn delimiter(&self) -> u8 {
        match self.delimiter {
            None => b',',
            Some(c) => c as u8,
        }
    }

    pub fn quote(&self) -> u8 {
        match self.quote {
            None => b'"',
            Some(c) => c as u8,
        }
    }

    pub fn flexible(&self) -> bool {
        self.flexible.unwrap_or(true)
    }

    pub fn with_property_placeholders(
        mut self,
        property_place_holders: HashMap<String, PlaceholderResolver>,
    ) -> Self {
        self.property_placeholders = property_place_holders;
        self
    }

    pub fn with_picklist_delimiter(mut self, c: char) -> Self {
        self.picklist_delimiter = Some(c);
        self
    }

    pub fn with_empty_property_placeholder(mut self, placeholder: PlaceholderResolver) -> Self {
        self.empty_property_placeholder = Some(placeholder);
        self
    }

    pub fn get_property_placeholder(&self, str: &str) -> Option<PlaceholderResolver> {
        if str.is_empty() {
            self.empty_property_placeholder.clone()
        } else {
            self.property_placeholders.get(str).cloned()
        }
    }

    pub fn empty_property_placeholder(&self) -> Option<PlaceholderResolver> {
        self.empty_property_placeholder.clone()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    // use tracing::debug;
    // use tracing_test::traced_test;

    use crate::PlaceholderResolver;

    use super::TapConfig;

    // #[traced_test]
    #[test]
    fn test_config() {
        let key = "nalt";
        let resolver = PlaceholderResolver::stem("pending");
        let mut ph = HashMap::new();
        ph.insert(key.to_string(), resolver.clone());
        let config = TapConfig::default()
            .with_property_placeholders(ph)
            .with_empty_property_placeholder(resolver.clone());
        // let yaml = serde_yml::to_string(&config).unwrap();
        // debug!("YAML\n{yaml}");
        assert_eq!(
            config.get_property_placeholder("nalt"),
            Some(resolver.clone())
        );
        assert_eq!(config.get_property_placeholder(""), Some(resolver.clone()))
    }
}
