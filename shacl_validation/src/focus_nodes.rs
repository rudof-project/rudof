use std::collections::HashSet;

use srdf::Rdf;

#[derive(Debug)]
pub struct FocusNodes<S: Rdf>(HashSet<S::Term>);

impl<S: Rdf> FocusNodes<S> {
    pub fn new(iter: impl Iterator<Item = S::Term>) -> Self {
        Self(HashSet::from_iter(iter))
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &S::Term> {
        self.0.iter()
    }
}

impl<S: Rdf> Clone for FocusNodes<S> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<S: Rdf> Default for FocusNodes<S> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<S: Rdf> IntoIterator for FocusNodes<S> {
    type Item = S::Term;
    type IntoIter = std::collections::hash_set::IntoIter<S::Term>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
