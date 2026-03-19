use rudof_rdf::rdf_core::Rdf;
use std::collections::HashSet;
use std::fmt::Display;
use std::rc::Rc;

/// Contains the set of focus nodes.
///
/// Internally uses `Rc<HashSet<S::Term>>` so that cloning is O(1)
/// (reference-count bump) instead of deep-copying the entire set.
/// A specialized `single()` constructor is provided for the case
/// of wrapping a single node, avoiding the overhead of building
/// a full `HashSet` via an iterator.
#[derive(Debug)]
pub struct FocusNodes<S: Rdf> {
    set: Rc<HashSet<S::Term>>,
}

impl<S: Rdf> FocusNodes<S> {
    pub fn new(set: HashSet<S::Term>) -> Self {
        Self { set: Rc::new(set) }
    }

    /// Creates a `FocusNodes` containing exactly one node.
    pub fn single(node: S::Term) -> Self {
        let mut set = HashSet::with_capacity(1);
        set.insert(node);
        Self { set: Rc::new(set) }
    }

    pub fn is_empty(&self) -> bool {
        self.set.is_empty()
    }

    pub fn len(&self) -> usize {
        self.set.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &S::Term> {
        self.set.iter()
    }
}

impl<S: Rdf> Clone for FocusNodes<S> {
    fn clone(&self) -> Self {
        Self {
            set: Rc::clone(&self.set),
        }
    }
}

impl<S: Rdf> Default for FocusNodes<S> {
    fn default() -> Self {
        Self {
            set: Rc::new(HashSet::new()),
        }
    }
}

impl<S: Rdf> FromIterator<S::Term> for FocusNodes<S> {
    fn from_iter<T: IntoIterator<Item = S::Term>>(iter: T) -> Self {
        Self {
            set: Rc::new(HashSet::from_iter(iter)),
        }
    }
}

impl<S: Rdf> IntoIterator for FocusNodes<S> {
    type Item = S::Term;
    type IntoIter = std::collections::hash_set::IntoIter<S::Term>;

    fn into_iter(self) -> Self::IntoIter {
        // If this is the only reference, unwrap without cloning.
        // Otherwise, clone the inner HashSet so we can consume it.
        match Rc::try_unwrap(self.set) {
            Ok(set) => set.into_iter(),
            Err(rc) => (*rc).clone().into_iter(),
        }
    }
}

impl<S: Rdf> Display for FocusNodes<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FocusNodes[")?;
        for (i, node) in self.set.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{node}")?;
        }
        write!(f, "]")
    }
}
