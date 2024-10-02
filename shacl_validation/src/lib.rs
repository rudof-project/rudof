use std::collections::HashMap;
use std::collections::HashSet;

use srdf::SRDFBasic;

pub(crate) mod constraints;
pub(crate) mod engine;
pub mod helper;
pub mod shacl_config;
pub mod shacl_processor;
pub mod shacl_validation_vocab;
pub(crate) mod shape;
pub mod store;
pub mod validate_error;
pub mod validation_report;

pub struct Targets<S: SRDFBasic>(HashSet<S::Term>);

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

impl<S: SRDFBasic> Default for Targets<S> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<S: SRDFBasic> IntoIterator for Targets<S> {
    type Item = S::Term;
    type IntoIter = std::collections::hash_set::IntoIter<S::Term>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

pub struct ValueNodes<S: SRDFBasic>(HashMap<S::Term, Targets<S>>);

impl<S: SRDFBasic> ValueNodes<S> {
    pub fn new(iter: impl Iterator<Item = (S::Term, Targets<S>)>) -> Self {
        Self(HashMap::from_iter(iter))
    }

    fn iter_value_nodes(&self) -> impl Iterator<Item = (&S::Term, &S::Term)> {
        self.0.iter().flat_map(|(focus_node, value_nodes)| {
            value_nodes
                .iter()
                .map(move |value_node| (focus_node, value_node))
        })
    }

    fn iter_focus_nodes(&self) -> impl Iterator<Item = (&S::Term, &Targets<S>)> {
        self.0.iter()
    }
}
