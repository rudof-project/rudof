use core::hash::Hash;
use std::fmt::Debug;
use std::fmt::Display;
use serde_derive::{Serialize, Deserialize};

#[derive(PartialEq, Eq, Hash, Debug, Default, Serialize, Deserialize, Clone, Copy)]
pub struct Component(usize);

impl Component
{
    pub fn new() -> Component {
      Component(0)
    }

    pub fn from(n: usize) -> Component {
      Component(n)
    }
}

impl Display for Component {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"C{}", self.0)
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