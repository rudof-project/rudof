use std::collections::HashSet;

use srdf::model::rdf::Object;
use srdf::model::rdf::Rdf;

#[derive(Debug)]
pub struct FocusNodes<R: Rdf>(HashSet<Object<R>>);

impl<R: Rdf> FocusNodes<R> {
    pub fn new(iter: impl Iterator<Item = Object<R>>) -> Self {
        Self(HashSet::from_iter(iter))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Object<R>> {
        self.0.iter()
    }
}

impl<R: Rdf> Clone for FocusNodes<R> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<R: Rdf> Default for FocusNodes<R> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<R: Rdf> IntoIterator for FocusNodes<R> {
    type Item = Object<R>;
    type IntoIter = std::collections::hash_set::IntoIter<Object<R>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
