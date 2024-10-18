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

/// Represents configuration file structure of DCTAP files
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone, Default)]
pub struct TapConfig {
    /// Character that is used to separate columns in CSV
    delimiter: Option<char>,

    /// The quote character to use when parsing CSV.
    /// The default is `"`.
    /// It can be used to indicate single quotes instead of double quotes.
    quote: Option<char>,

    /// Whether the number of fields in records is allowed to change or not.
    ///
    /// When disabled, parsing CSV data will return an
    /// error if a record is found with a number of fields different from the
    /// number of fields in a previous record.
    ///
    /// When enabled, this error checking is turned off. It is enabled by default.
    flexible: Option<bool>,

    /// Character that is used to separate values in a picklist cell. The default value is `|`
    picklist_delimiter: Option<char>,

    /// Table that can be used to generate values for some keys.
    /// When the processor finds a cell with some of those keys,
    /// it generates a value according to the placeholder resolver indicated.
    /// At this moment, `rudof` supports the placeholder resolver `!Stem`
    /// which means that it will replace the key by the corresponding stem value.
    ///
    /// For example, if the property placeholder has the entry `x` with the
    /// placeholder resolver of type `!Stem` and the value `stem: "Pending"`,
    /// when a cell contains `x:User`, the generated value will be: `pending:User`.
    ///
    /// <div class="warning">This field is experimental and the syntax may change</div>
    ///
    property_placeholders: HashMap<String, PlaceholderResolver>,

    /// Indicates how to generate a value for a row whose property ID is empty.
    ///
    /// When the processor find
    ///
    /// <div class="warning">This field is experimental and the syntax may change</div>
    ///
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
