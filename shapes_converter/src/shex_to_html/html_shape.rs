use serde::Serialize;

use super::{Name, ShapeTemplateEntry};

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct HtmlShape {
    name: Name,
    entries: Vec<ShapeTemplateEntry>,
    extends: Vec<Name>,
    parent: Name,
}

impl HtmlShape {
    pub fn new(name: Name, parent: Name) -> HtmlShape {
        HtmlShape {
            name,
            entries: Vec::new(),
            extends: Vec::new(),
            parent,
        }
    }

    pub fn add_entry(&mut self, entry: ShapeTemplateEntry) {
        self.entries.push(entry)
    }

    pub fn name(&self) -> Name {
        self.name.clone()
    }

    pub fn entries(&self) -> impl Iterator<Item = &ShapeTemplateEntry> {
        self.entries.iter()
    }

    pub fn add_extends(&mut self, name: &Name) {
        self.extends.push(name.clone())
    }

    pub fn extends(&self) -> impl Iterator<Item = &Name> {
        self.extends.iter()
    }

    pub fn merge(&mut self, other: &HtmlShape) {
        for entry in other.entries() {
            self.add_entry(entry.clone())
        }
        for extend in other.extends() {
            self.add_extends(extend)
        }
    }
}
