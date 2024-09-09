use std::collections::HashSet;

use super::{Name, NodeId, UmlEntry};

#[derive(Debug, PartialEq)]

pub struct UmlClass {
    name: Name,
    entries: Vec<UmlEntry>,
    extends: HashSet<NodeId>,
}

impl UmlClass {
    pub fn new(name: Name) -> UmlClass {
        UmlClass {
            name,
            entries: Vec::new(),
            extends: HashSet::new(),
        }
    }

    pub fn add_entry(&mut self, entry: UmlEntry) {
        self.entries.push(entry)
    }

    pub fn add_extends(&mut self, node: &NodeId) {
        self.extends.insert(*node);
    }

    pub fn name(&self) -> String {
        self.name.name()
    }

    pub fn label(&self) -> Option<String> {
        self.name.label()
    }

    pub fn href(&self) -> Option<String> {
        self.name.href()
    }

    pub fn entries(&self) -> impl Iterator<Item = &UmlEntry> {
        self.entries.iter()
    }
}
