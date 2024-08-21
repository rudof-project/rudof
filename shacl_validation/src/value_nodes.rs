use std::collections::HashMap;

use srdf::SRDFBasic;

use crate::targets::Targets;

pub struct ValueNodes<'a, S: SRDFBasic> {
    iter: HashMap<&'a S::Term, Targets<'a, S>>,
}

impl<'a, S: SRDFBasic> ValueNodes<'a, S> {
    pub fn new(iter: impl Iterator<Item = (&'a S::Term, Targets<'a, S>)> + 'a) -> Self {
        Self {
            iter: HashMap::from_iter(iter),
        }
    }
}

impl<'a, S: SRDFBasic> Iterator for ValueNodes<'a, S> {
    type Item = (&'a S::Term, &'a S::Term);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((outer_term, mut targets_iter)) = self.iter.iter().next() {
            if let Some(inner_term) = targets_iter.next() {
                Some((outer_term, inner_term))
            } else {
                self.next()
            }
        } else {
            None
        }
    }
}
