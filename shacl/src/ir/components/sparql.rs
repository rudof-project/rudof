use std::fmt::{Display, Formatter};
use prefixmap::PrefixMap;
use crate::types::MessageMap;

#[derive(Debug, Clone)]
pub struct Sparql {
    select: String,
    message: Option<MessageMap>,
    deactivated: Option<bool>,
    prefixes: Option<PrefixMap>,
}

impl Sparql {
    pub fn new(select: String) -> Self {
        Self {
            select,
            message: None,
            deactivated: None,
            prefixes: None,
        }
    }

    pub fn with_message(mut self, message: Option<MessageMap>) -> Self {
        self.message = message;
        self
    }

    pub fn with_deactivated(mut self, deactivated: Option<bool>) -> Self {
        self.deactivated = deactivated;
        self
    }

    pub fn with_prefixes(mut self, prefixes: Option<PrefixMap>) -> Self {
        self.prefixes = prefixes;
        self
    }

    pub fn select(&self) -> &String {
        &self.select
    }

    pub fn message(&self) -> Option<&MessageMap> {
        self.message.as_ref()
    }

    pub fn deactivated(&self) -> Option<bool> {
        self.deactivated
    }

    pub fn prefixes(&self) -> Option<&PrefixMap> {
        self.prefixes.as_ref()
    }
}

impl Display for Sparql {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Sparql: select: {}, deactivated: {:?}, message: {:?}, prefixes: {:?}",
            self.select(),
            self.deactivated(),
            self.message(),
            self.prefixes(),
        )
    }
}