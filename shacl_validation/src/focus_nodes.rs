use std::collections::HashSet;

use srdf::SRDFBasic;

#[derive(Debug)]
pub struct FocusNodes<S: SRDFBasic>(HashSet<S::Term>);

impl<S: SRDFBasic> FocusNodes<S> {
    pub fn new(iter: impl Iterator<Item = S::Term>) -> Self {
        Self(HashSet::from_iter(iter))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &S::Term> {
        self.0.iter()
    }
}

impl<S: SRDFBasic> Clone for FocusNodes<S> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<S: SRDFBasic> Default for FocusNodes<S> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<S: SRDFBasic> IntoIterator for FocusNodes<S> {
    type Item = S::Term;
    type IntoIter = std::collections::hash_set::IntoIter<S::Term>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
