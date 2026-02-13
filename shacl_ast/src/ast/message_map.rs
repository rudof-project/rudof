use rdf::rdf_core::term::literal::Lang;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct MessageMap {
    messages: HashMap<Option<Lang>, String>,
}

impl MessageMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_message(mut self, lang: Option<Lang>, message: String) -> Self {
        self.messages.insert(lang, message);
        self
    }

    pub fn messages(&self) -> &HashMap<Option<Lang>, String> {
        &self.messages
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Option<Lang>, &String)> {
        self.messages.iter()
    }
}

impl FromStr for MessageMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            messages: HashMap::from([(None, s.to_string())]),
        })
    }
}
