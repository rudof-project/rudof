use std::fmt::Display;
use crate::{Pending, Rbe1Error};
use core::hash::Hash;
use std::fmt::Debug;
use serde_derive::{Serialize, Deserialize};



#[derive(Clone, Serialize, Deserialize)]
pub struct MatchCond<K, V, R> 
 where K: Hash + Eq + Display + Default,
       V: Hash + Eq + Default + PartialEq + Clone,
       R: Default + PartialEq + Clone
{
    #[serde(skip)]
    cond: Option<fn(&K, &V) -> Result<Pending<V,R>, Rbe1Error<K, V, R>>> 
}



impl <K, V, R> MatchCond<K, V, R> 
where K: Hash + PartialEq + Eq + Display + Default,
      V: Hash + Default + Eq + Debug + Clone, 
      R: Default + PartialEq + Debug + Clone {
    
    pub fn matches(&self, key: &K, value: &V) -> Result<Pending<V,R>, Rbe1Error<K, V, R>> {
        match self.cond {
            None => Ok(Pending::new()),
            Some(f) => (f)(key, value)
        }
    }
}

impl <K,V,R> PartialEq for MatchCond<K, V,R> 
where K: Hash + PartialEq + Eq + Display + Default,
V: Hash + Default + Eq + Clone, 
R: Default + PartialEq + Clone {
    fn eq(&self, other: &Self) -> bool { 
        match (self.cond, other.cond) {
            (None, None) => true,
            (None, Some(_)) => false,
            (Some(_), None) => false,
            (Some(_), Some(_)) => {
                todo!()
            }
        } 
    }
}

impl <K,V,R> Eq for MatchCond<K, V,R> 
where K: Hash + PartialEq + Eq + Display + Default,
V: Hash + Eq + Default + Clone, 
R: Default + PartialEq + Clone {
}

impl <K, V, R> Default for MatchCond<K, V, R> 
where K: Hash + PartialEq + Eq + Display + Default,
      V: Hash + Default + Eq + Clone,
      R: Default + PartialEq + Clone {
    fn default() -> Self { MatchCond {
        cond: None
    }}
}
