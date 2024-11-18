use std::collections::HashSet;

use srdf::model::rdf::Rdf;
use srdf::model::rdf::TObject;

#[derive(Debug)]
pub struct FocusNodes<R: Rdf>(HashSet<TObject<R>>);

impl<R: Rdf> FocusNodes<R> {
    pub fn new(iter: impl Iterator<Item = TObject<R>>) -> Self {
        Self(HashSet::from_iter(iter))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &TObject<R>> {
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
    type Item = TObject<R>;
    type IntoIter = std::collections::hash_set::IntoIter<TObject<R>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
