use serde::Serialize;

use super::{Entry, Name};

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct HtmlShape {
    name: Name,
    entries: Vec<Entry>,
    extends: Vec<Name>,
}

impl HtmlShape {
    pub fn new(name: Name) -> HtmlShape {
        HtmlShape {
            name,
            entries: Vec::new(),
            extends: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, entry: Entry) {
        self.entries.push(entry)
    }

    pub fn name(&self) -> Name {
        self.name.clone()
    }

    pub fn entries(&self) -> impl Iterator<Item = &Entry> {
        self.entries.iter()
    }

    pub fn add_extends(&mut self, name: &Name) {
        self.extends.push(name.clone())
    }

    pub fn extends(&self) -> impl Iterator<Item = &Name> {
        self.extends.iter()
    }
}
