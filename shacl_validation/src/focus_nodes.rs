use std::collections::HashSet;

use srdf::model::rdf::Rdf;
use srdf::model::rdf::TObjectRef;

#[derive(Debug)]
pub struct FocusNodes<R: Rdf>(HashSet<TObjectRef<R>>);

impl<R: Rdf> FocusNodes<R> {
    pub fn new(iter: impl Iterator<Item = TObjectRef<R>>) -> Self {
        Self(HashSet::from_iter(iter))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &TObjectRef<R>> {
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
    type Item = TObjectRef<R>;
    type IntoIter = std::collections::hash_set::IntoIter<TObjectRef<R>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
