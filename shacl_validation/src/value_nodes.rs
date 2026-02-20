use rudof_rdf::rdf_core::Rdf;
use std::{collections::HashMap, fmt::Display};

use crate::focus_nodes::FocusNodes;

pub struct ValueNodes<S: Rdf> {
    map: HashMap<S::Term, FocusNodes<S>>,
}

impl<S: Rdf> ValueNodes<S> {
    pub fn new(iter: impl Iterator<Item = (S::Term, FocusNodes<S>)>) -> Self {
        Self {
            map: HashMap::from_iter(iter),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&S::Term, &FocusNodes<S>)> {
        self.map.iter()
    }
}

impl<R: Rdf> Display for ValueNodes<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ValueNodes[")?;
        for (i, (node, vnodes)) in self.map.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{node} -> {vnodes}")?;
        }
        write!(f, "]")
    }
}
