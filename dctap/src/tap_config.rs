use crate::{PlaceholderResolver, TapError};
use serde::{Deserialize, Serialize};
use std::io::Read;
use std::str::FromStr;
use std::{collections::HashMap, path::Path};

/// Represents configuration file structure of DCTAP files
#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct TapConfig {
    /// Character that is used to separate columns in CSV
    #[serde(rename = "delimiter", default = "TapConfig::default_delimiter")]
    delimiter: char,

    /// The quote character to use when parsing CSV.
    /// The default is `"`.
    /// It can be used to indicate single quotes instead of double quotes.
    #[serde(rename = "quote", default = "TapConfig::default_quote")]
    quote: char,

    /// Whether the number of fields in records is allowed to change or not.
    ///
    /// When disabled, parsing CSV data will return an
    /// error if a record is found with a number of fields different from the
    /// number of fields in a previous record.
    ///
    /// When enabled, this error checking is turned off. It is enabled by default.
    #[serde(rename = "flexible", default = "TapConfig::default_flexible")]
    flexible: bool,

    /// Character that is used to separate values in a picklist cell. The default value is `|`
    #[serde(rename = "picklist_delimiter", default = "TapConfig::default_picklist_delimiter")]
    picklist_delimiter: char,

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
    #[serde(rename = "property_placeholders", default = "TapConfig::default_property_placeholders")]
    property_placeholders: HashMap<String, PlaceholderResolver>,

    /// Indicates how to generate a value for a row whose property ID is empty.
    ///
    /// When the processor find
    ///
    /// <div class="warning">This field is experimental and the syntax may change</div>
    ///
    #[serde(rename = "empty_property_placeholder", default = "TapConfig::default_empty_property_placeholder")]
    empty_property_placeholder: Option<PlaceholderResolver>,

    /// String that is used to separate values in a value shape cell. The default value is whitespace.
    ///
    /// <div class="warning">This field is experimental and the syntax may change</div>
    ///
    #[serde(rename = "value_shape_delimiter", default = "TapConfig::default_value_shape_delimiter")]
    value_shape_delimiter: char,
}

impl TapConfig {
    pub fn new() -> Self {
        Self {
            delimiter: Self::default_delimiter(),
            quote: Self::default_quote(),
            flexible: Self::default_flexible(),
            picklist_delimiter: Self::default_picklist_delimiter(),
            property_placeholders: Self::default_property_placeholders(),
            empty_property_placeholder: Self::default_empty_property_placeholder(),
            value_shape_delimiter: Self::default_value_shape_delimiter(),
        }
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Self {
        let mut f = std::fs::File::open(path).unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();
        toml::from_str(s.as_str()).unwrap()
    }

    pub fn with_delimiter(mut self, c: char) -> Self {
        self.delimiter = c;
        self
    }

    pub fn with_quote(mut self, c: char) -> Self {
        self.quote = c;
        self
    }

    pub fn with_flexible(mut self, flag: bool) -> Self {
        self.flexible = flag;
        self
    }

    pub fn with_picklist_delimiter(mut self, c: char) -> Self {
        self.picklist_delimiter = c;
        self
    }

    pub fn with_property_placeholders(mut self, placeholders: HashMap<String, PlaceholderResolver>) -> Self {
        self.property_placeholders = placeholders;
        self
    }

    pub fn with_empty_property_placeholder(mut self, placeholder: Option<PlaceholderResolver>) -> Self {
        self.empty_property_placeholder = placeholder;
        self
    }

    pub fn with_value_shape_delimiter(mut self, c: char) -> Self {
        self.value_shape_delimiter = c;
        self
    }

    pub fn add_property_placeholder(&mut self, k: String, v: PlaceholderResolver) -> Option<PlaceholderResolver> {
        self.property_placeholders.insert(k, v)
    }
    pub fn remove_property_placeholder(&mut self, k: &String) -> Option<PlaceholderResolver> {
        self.property_placeholders.remove(k)
    }
}

impl TapConfig {
    pub fn delimiter(&self) -> char {
        self.delimiter
    }

    pub fn quote(&self) -> char {
        self.quote
    }

    pub fn flexible(&self) -> bool {
        self.flexible
    }

    pub fn picklist_delimiter(&self) -> char {
        self.picklist_delimiter
    }

    pub fn property_placeholders(&self) -> &HashMap<String, PlaceholderResolver> {
        &self.property_placeholders
    }

    pub fn empty_property_placeholder(&self) -> Option<&PlaceholderResolver> {
        self.empty_property_placeholder.as_ref()
    }

    pub fn value_shape_delimiter(&self) -> char {
        self.value_shape_delimiter
    }

    pub fn get_property_placeholder(&self, k: &String) -> Option<&PlaceholderResolver> {
        self.property_placeholders.get(k)
    }
}

/// Serde stuff
#[allow(dead_code)]
#[cfg_attr(rustfmt, rustfmt_skip)]
impl TapConfig {
    #[inline] fn default_delimiter() -> char { ',' }
    #[inline] fn default_quote() -> char { '"' }
    #[inline] fn default_flexible() -> bool { true }
    #[inline] fn default_picklist_delimiter() -> char { '|' }
    #[inline] fn default_property_placeholders() -> HashMap<String, PlaceholderResolver> { HashMap::new() }
    #[inline] fn default_empty_property_placeholder() -> Option<PlaceholderResolver> { None }
    #[inline] fn default_value_shape_delimiter() -> char { ' ' }
}

impl Default for TapConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for TapConfig {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        toml::from_str(s).map_err(|e| format!("Failed to parse TapConfig: {e}"))
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, str::FromStr};
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
            .with_empty_property_placeholder(resolver.clone().into());
        assert_eq!(config.get_property_placeholder(&"nalt".to_string()), Some(&resolver));
        assert_eq!(config.get_property_placeholder(&"".to_string()), Some(&resolver))
    }

    #[test]
    fn test_tap_config_delimiter() {
        let s = r#"[dctap]
delimiter = ","
picklist_delimiter = " "
"#;
        let config = TapConfig::from_str(s).unwrap();
        assert_eq!(config.delimiter(), ',');
    }

    #[test]
    fn test_tap_config_property_placeholder() {
        let s = r#"[dctap]
        delimiter = ","

        [property_placeholders.y.Stem]
        stem = "pending2"

        [property_placeholders.x.Stem]
        stem = "pending"

        [empty_property_placeholder.Stem]
        stem = "empty"
"#;
        let config = TapConfig::from_str(s).unwrap();
        /*let mut property_placeholders = HashMap::new();
        property_placeholders.insert("x".to_string(), PlaceholderResolver::stem("pending"));
        property_placeholders.insert("y".to_string(), PlaceholderResolver::stem("pending2"));
        let config = TapConfig::default()
            .with_property_placeholders(property_placeholders)
            .with_empty_property_placeholder(PlaceholderResolver::stem("empty")); */
        assert_eq!(
            config.get_property_placeholder(&"x".to_string()).unwrap(),
            &PlaceholderResolver::stem("pending")
        );
        // let str = toml::to_string_pretty(&config).unwrap();
        // assert_eq!(str, "what?".to_string());
    }
}
