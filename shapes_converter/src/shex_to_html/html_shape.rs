use serde::Serialize;

use super::{Name, ShapeTemplateEntry};

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct HtmlShape {
    /// Name of this shape
    name: Name,

    /// Sequence of entries
    entries: Vec<ShapeTemplateEntry>,

    /// Sequence of shape expressions that this shape extends
    extends: Vec<Name>,

    /// Parent represents the name of the schema or shape expression to which this shape belongs
    parent: Name,

    /// Sequence of shape expressions that extend this shape
    children: Vec<Name>,

    /// SVG visualization of the neighbors of a shape
    pub svg_shape: Option<String>,
}

impl HtmlShape {
    pub fn new(name: Name, parent: Name) -> HtmlShape {
        HtmlShape {
            name,
            entries: Vec::new(),
            extends: Vec::new(),
            parent,
            children: Vec::new(),
            svg_shape: None,
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

    pub fn svg_shape(&self) -> Option<String> {
        match &self.svg_shape {
            None => None,
            Some(s) => Some(s.clone()),
        }
    }

    pub fn set_svg_shape(&mut self, str: &str) {
        self.svg_shape = Some(str.to_string());
    }
}
