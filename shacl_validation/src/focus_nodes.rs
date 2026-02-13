use rdf::rdf_core::Rdf;
use std::collections::HashSet;
use std::fmt::Display;

/// Contains the set of focus nodes
#[derive(Debug)]
pub struct FocusNodes<S: Rdf> {
    set: HashSet<S::Term>,
}

impl<S: Rdf> FocusNodes<S> {
    pub fn new(set: HashSet<S::Term>) -> Self {
        Self { set }
    }

    /*pub fn from_iter(iter: impl Iterator<Item = S::Term>) -> Self {
        Self {
            set: HashSet::from_iter(iter),
        }
    }*/

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
            set: self.set.clone(),
        }
    }
}

impl<S: Rdf> Default for FocusNodes<S> {
    fn default() -> Self {
        Self {
            set: Default::default(),
        }
    }
}

impl<S: Rdf> FromIterator<S::Term> for FocusNodes<S> {
    fn from_iter<T: IntoIterator<Item = S::Term>>(iter: T) -> Self {
        Self {
            set: HashSet::from_iter(iter),
        }
    }
}

impl<S: Rdf> IntoIterator for FocusNodes<S> {
    type Item = S::Term;
    type IntoIter = std::collections::hash_set::IntoIter<S::Term>;

    fn into_iter(self) -> Self::IntoIter {
        self.set.into_iter()
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
