use super::{Entry, Name};

#[derive(Debug, PartialEq, Clone)]
pub struct HtmlShape {
    name: Name,
    entries: Vec<Entry>,
}

impl HtmlShape {
    pub fn new(name: Name) -> HtmlShape {
        HtmlShape {
            name,
            entries: Vec::new(),
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
}
