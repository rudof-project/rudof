use oxrdf::{Literal as OxLiteral, Term as OxTerm};
use srdf::lang::Lang;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Default, Clone)]
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

    pub fn to_term_iter(&self) -> impl Iterator<Item = OxTerm> + '_ {
        self.messages.iter().map(|(lang, message)| {
            let literal = if let Some(lang) = lang {
                OxLiteral::new_language_tagged_literal(message, lang.value()).unwrap()
            } else {
                OxLiteral::new_simple_literal(message)
            };

            OxTerm::Literal(literal)
        })
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
