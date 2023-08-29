use core::hash::Hash;
use serde_derive::{Deserialize, Serialize};
use std::fmt::Debug;
use std::fmt::Display;

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
