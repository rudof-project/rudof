use rudof_rdf::rdf_core::Rdf;
use std::collections::HashSet;
use std::collections::hash_set::IntoIter;
use std::fmt::{Display, Formatter};
use std::sync::Arc;

/// Contains the set of focus nodes.
#[derive(Debug)]
pub struct FocusNodes<RDF: Rdf> {
    set: Arc<HashSet<RDF::Term>>,
}

impl<RDF: Rdf> FocusNodes<RDF> {
    pub fn new(set: HashSet<RDF::Term>) -> Self {
        Self { set: Arc::new(set) }
    }

    /// Creates a [`FocusNodes`] containing exactly one node.
    pub fn single(node: RDF::Term) -> Self {
        let mut set = HashSet::with_capacity(1);
        set.insert(node);
        Self { set: Arc::new(set) }
    }

    pub fn is_empty(&self) -> bool {
        self.set.is_empty()
    }

    pub fn len(&self) -> usize {
        self.set.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &RDF::Term> {
        self.set.iter()
    }
}

impl<RDF: Rdf> Clone for FocusNodes<RDF> {
    fn clone(&self) -> Self {
        Self {
            set: Arc::clone(&self.set),
        }
    }
}

impl<RDF: Rdf> Default for FocusNodes<RDF> {
    fn default() -> Self {
        Self {
            set: Arc::new(HashSet::new()),
        }
    }
}

impl<RDF: Rdf> FromIterator<RDF::Term> for FocusNodes<RDF> {
    fn from_iter<T: IntoIterator<Item = RDF::Term>>(iter: T) -> Self {
        Self {
            set: Arc::new(HashSet::from_iter(iter)),
        }
    }
}

impl<RDF: Rdf> IntoIterator for FocusNodes<RDF> {
    type Item = RDF::Term;
    type IntoIter = IntoIter<RDF::Term>;

    fn into_iter(self) -> Self::IntoIter {
        // If this is the only reference, unwrap without cloning.
        // Otherwise, clone the inner HashSet so we can consume it.
        match Arc::try_unwrap(self.set) {
            Ok(set) => set.into_iter(),
            Err(rc) => (*rc).clone().into_iter(),
        }
    }
}

impl<RDF: Rdf> Display for FocusNodes<RDF> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FocusNodes[{}]",
            self.set
                .iter()
                .map(|node| node.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}
