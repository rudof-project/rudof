use rudof_rdf::rdf_core::term::literal::{ConcreteLiteral, Lang};
use std::collections::HashMap;
use std::collections::hash_map::IntoIter;
use std::fmt::{Display, Formatter};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
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

    pub fn iter_literals(&self) -> impl Iterator<Item = ConcreteLiteral> {
        self.messages.iter().map(|(lang, msg)| ConcreteLiteral::StringLiteral {
            lang: lang.clone(),
            lexical_form: msg.clone(),
        })
    }

    pub fn get(&self, lang: Option<&Lang>) -> Option<&String> {
        self.messages.get(&lang.cloned())
    }

    pub fn merge(mut self, other: Self, over: bool) -> Self {
        other.into_iter().for_each(|(lang, msg)| {
            if over || !self.messages.contains_key(&lang) {
                self.messages.insert(lang, msg);
            }
        });
        self
    }
}

impl IntoIterator for MessageMap {
    type Item = (Option<Lang>, String);
    type IntoIter = IntoIter<Option<Lang>, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.messages.into_iter()
    }
}

impl From<&str> for MessageMap {
    fn from(value: &str) -> Self {
        Self {
            messages: HashMap::from([(None, value.to_string())]),
        }
    }
}

impl From<String> for MessageMap {
    fn from(value: String) -> Self {
        Self {
            messages: HashMap::from([(None, value)]),
        }
    }
}

impl Display for MessageMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut entries: Vec<_> = self.messages.iter().collect();

        entries.sort_by_key(|(lang, _)| lang.as_ref());

        if entries.len() == 1
            && let Some((None, msg)) = entries.first()
        {
            return write!(f, "\"{msg}\"");
        }

        for (i, (lang, msg)) in entries.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }

            match lang {
                None => write!(f, "\"{msg}\"")?,
                Some(lang) => write!(f, "\"{msg}\"@{lang}")?,
            }
        }

        Ok(())
    }
}
