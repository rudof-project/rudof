use core::hash::Hash;
use std::fmt::Debug;
use std::fmt::Display;
use crate::MatchCond;
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Component<K, V, R> 
where K: Hash + Eq + Display + Default,
      V: Hash + Eq + Default + PartialEq + Clone,
      R: Default + PartialEq + Clone,
{
    key: K, 
    cond: MatchCond<K, V, R>
}

impl <K,V,R> Component<K,V,R> 
where K: Hash + Eq + Display + Default + Clone,
      V: Hash + Eq + Default + Display + Debug + PartialEq + Clone,
      R: Default + PartialEq + Display + Debug + Clone,
{

    pub fn new() -> Component<K,V,R> {
      Component {key: K::default(),
        cond: MatchCond::new()
      }
    }

    pub fn key(&self) -> K {
       self.key.clone()
    }


}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_component_creation() {
     let c1: Component<char, i32, String> = Component::new();
     assert_eq!(c1.key(), '\0')

  }
}