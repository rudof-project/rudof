use std::collections::HashMap;
use std::collections::HashSet;

use srdf::SRDFBasic;

pub(crate) mod constraints;
pub(crate) mod context;
pub mod helper;
pub(crate) mod runner;
pub mod shacl_validation_vocab;
pub(crate) mod shape;
pub mod store;
pub mod validate;
pub mod validate_error;
pub mod validation_report;

pub(crate) struct Targets<S: SRDFBasic>(HashSet<S::Term>);

impl<S: SRDFBasic> Targets<S> {
    pub fn new(iter: impl Iterator<Item = S::Term>) -> Self {
        Self(HashSet::from_iter(iter))
    }

    fn iter(&self) -> impl Iterator<Item = &S::Term> {
        self.0.iter()
    }
}

impl<S: SRDFBasic> Clone for Targets<S> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<S: SRDFBasic> IntoIterator for Targets<S> {
    type Item = S::Term;
    type IntoIter = std::collections::hash_set::IntoIter<S::Term>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

pub(crate) struct ValueNodes<S: SRDFBasic>(HashMap<S::Term, S::Term>);

impl<S: SRDFBasic> ValueNodes<S> {
    pub fn new(iter: impl Iterator<Item = (S::Term, Targets<S>)>) -> Self {
        let flatten = iter.flat_map(|(key, values)| {
            values
                .into_iter()
                .map(move |value: <S as SRDFBasic>::Term| (key.clone(), value))
                .collect::<HashMap<S::Term, S::Term>>()
        });
        Self(HashMap::from_iter(flatten))
    }

    fn iter(&self) -> impl Iterator<Item = (&S::Term, &S::Term)> {
        self.0.iter()
    }
}
