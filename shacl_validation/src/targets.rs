use std::collections::hash_set;
use std::collections::HashSet;

use srdf::SRDFBasic;

pub struct Targets<'a, S: SRDFBasic> {
    iter: hash_set::Iter<'a, &'a S::Term>,
}

impl<'a, S: SRDFBasic> Targets<'a, S> {
    pub fn new(iter: impl Iterator<Item = &'a S::Term> + 'a) -> Self {
        Self {
            iter: HashSet::<&'a S::Term>::from_iter(iter).iter(),
        }
    }
}

impl<'a, S: SRDFBasic> Iterator for Targets<'a, S> {
    type Item = &'a S::Term;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().copied()
    }
}
