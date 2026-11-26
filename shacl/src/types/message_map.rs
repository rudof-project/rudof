use rudof_rdf::rdf_core::term::literal::{ConcreteLiteral, Lang};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::hash_map::IntoIter;
use std::fmt::{Display, Formatter};

/// Serialized as the list of literals it holds, sorted by language tag: a JSON object
/// cannot key a message by `Option<Lang>`, and a language-tagged literal already carries
/// the tag the key would have held.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(into = "Vec<ConcreteLiteral>", from = "Vec<ConcreteLiteral>")]
#[schemars(with = "Vec<ConcreteLiteral>")]
pub struct MessageMap {
    messages: HashMap<Option<Lang>, String>,
}

impl From<MessageMap> for Vec<ConcreteLiteral> {
    fn from(map: MessageMap) -> Self {
        let mut literals: Vec<_> = map.messages.into_iter().collect();
        literals.sort_by(|(left, _), (right, _)| left.cmp(right));
        literals
            .into_iter()
            .map(|(lang, message)| ConcreteLiteral::StringLiteral {
                lexical_form: message,
                lang,
                direction: None,
            })
            .collect()
    }
}

impl From<Vec<ConcreteLiteral>> for MessageMap {
    fn from(literals: Vec<ConcreteLiteral>) -> Self {
        literals.into_iter().fold(MessageMap::new(), |map, literal| {
            map.with_message(literal.lang(), literal.lexical_form())
        })
    }
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
            direction: None,
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
