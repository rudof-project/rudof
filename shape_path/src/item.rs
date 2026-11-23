//! The items a ShExPath selects.

use serde_json::Value;
use std::fmt;

/// One step from an item to a property value or an array member.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Segment {
    /// The value of a property of an object.
    Key(String),
    /// A member of an array, from 0.
    Index(usize),
}

/// Where an item is in the ShExJ document: the steps from the schema to it.
///
/// Two items are the same item when they have the same location. It is shown as a
/// [JSON Pointer](https://www.rfc-editor.org/rfc/rfc6901); the schema's is the empty string.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Location(Vec<Segment>);

impl Location {
    /// The location of the schema.
    pub fn root() -> Self {
        Location(Vec::new())
    }

    /// The steps from the schema to the item.
    pub fn segments(&self) -> &[Segment] {
        &self.0
    }

    /// The location of the value of property `key` of the item here.
    pub fn key(&self, key: &str) -> Self {
        self.with(Segment::Key(key.to_string()))
    }

    /// The location of member `index` of the array here.
    pub fn index(&self, index: usize) -> Self {
        self.with(Segment::Index(index))
    }

    /// The location of the item that holds this one, if this is not the schema.
    pub fn parent(&self) -> Option<Self> {
        self.0.split_last().map(|(_, parent)| Location(parent.to_vec()))
    }

    fn with(&self, segment: Segment) -> Self {
        let mut segments = self.0.clone();
        segments.push(segment);
        Location(segments)
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for segment in &self.0 {
            match segment {
                Segment::Key(key) => write!(f, "/{}", key.replace('~', "~0").replace('/', "~1"))?,
                Segment::Index(index) => write!(f, "/{index}")?,
            }
        }
        Ok(())
    }
}

/// An item: a node of the ShExJ document, with where it is.
#[derive(Debug, Clone, PartialEq)]
pub struct ShExPathItem<'a> {
    value: &'a Value,
    location: Location,
}

impl<'a> ShExPathItem<'a> {
    pub(crate) fn new(value: &'a Value, location: Location) -> Self {
        ShExPathItem { value, location }
    }

    /// The item: an object, an array or an atomic value.
    pub fn value(&self) -> &'a Value {
        self.value
    }

    /// Where the item is in the document.
    pub fn location(&self) -> &Location {
        &self.location
    }
}
