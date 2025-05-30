use core::hash::Hash;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::fmt::Display;

/// A wrapper around a usize to represent a component in the RBE table.
/// This is used to identify components in the RBE table and is used as a key in the
/// `RbeTable` struct.
#[derive(PartialEq, Eq, Hash, Default, Serialize, Deserialize, Clone, Copy)]
pub struct Component(usize);

impl Component {
    pub fn new() -> Component {
        Component(0)
    }

    pub fn from(n: usize) -> Component {
        Component(n)
    }
}

impl Display for Component {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "C{}", self.0)
    }
}

impl Debug for Component {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "C{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_creation() {
        let c1: Component = Component::new();
        assert_eq!(c1.0, 0)
    }
}
