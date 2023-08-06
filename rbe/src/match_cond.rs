use std::fmt::Display;
use crate::{Pending, Rbe1Error};
use core::hash::Hash;
use std::fmt::Debug;
use serde_derive::{Serialize, Deserialize};

/// Represents a matching condition
#[derive(Serialize, Deserialize)]
pub struct MatchCond<K, V, R> 
 where K: Hash + Eq + Display + Default,
       V: Hash + Eq + Default + PartialEq + Clone,
       R: Default + PartialEq + Clone,
{
    name: Option<String>, 

    #[serde(skip)]
    cond: Option<Box<dyn Cond<K, V,R>>> 
}

/// We use trait objects instead of function pointers because we need to
/// capture some values in the condition closure.
/// This pattern is inspired by the answer in this thread:
/// https://users.rust-lang.org/t/how-to-clone-a-boxed-closure/31035
trait Cond<K, V, R> 
where K: Hash + Eq + Display + Default,
      V: Hash + Eq + Default + PartialEq + Clone,
      R: Default + PartialEq + Clone
{
    fn clone_box(&self) -> Box<dyn Cond<K,V,R>>;
    fn call(&self, k: &K, v: &V) -> Result<Pending<V,R>, Rbe1Error<K, V, R>>;
}

impl<K, V, R, F> Cond<K,V,R> for F 
where  K: Hash + Eq + Display + Default,
       V: Hash + Eq + Default + PartialEq + Clone,
       R: Default + PartialEq + Clone,
       F: 'static + Fn(&K, &V) -> Result<Pending<V,R>, Rbe1Error<K, V, R>> + Clone 
{

  fn clone_box(&self) -> Box<dyn Cond<K, V, R>> {
    Box::new(self.clone())
  }

  fn call(&self, k:&K, v: &V) -> Result<Pending<V,R>, Rbe1Error<K, V, R>> {
    self(k, v)
  }
}

impl <K, V, R> Clone for Box<dyn Cond<K, V, R>>
where  K: Hash + Eq + Display + Default,
       V: Hash + Eq + Default + PartialEq + Clone,
       R: Default + PartialEq + Clone,
{
    fn clone(&self) -> Self {
        self.clone_box()
    }
} 

/*impl <K, V, R> PartialEq for Box<dyn Cond<K, V, R>>
where  K: Hash + Eq + Display + Default,
       V: Hash + Eq + Default + PartialEq + Clone,
       R: Default + PartialEq + Clone,
{
    fn eq(&self, other: &Box<dyn Cond<K, V, R>>) -> bool { 
        todo!()
    }
}*/ 

impl <K, V, R> Clone for MatchCond<K,V,R> 
where K: Hash + Eq + Display + Default,
      V: Hash + Eq + Default + PartialEq + Clone,
      R: Default + PartialEq + Clone, 
{
    fn clone(&self) -> Self {
        MatchCond {
            name: self.name.clone(),
            cond: match &self.cond {
                Option::None => {
                    None
                },
                Option::Some(f) => {
                    Some((*f).clone())
                }
            } 
        }
    }
}


impl <K, V, R> MatchCond<K, V, R> 
where K: Hash + PartialEq + Eq + Display + Default,
      V: Hash + Default + Eq + Debug + Clone, 
      R: Default + PartialEq + Debug + Clone, 
      {
    
    pub fn matches(&self, key: &K, value: &V) -> Result<Pending<V,R>, Rbe1Error<K, V, R>> {
        match &self.cond {
            None => Ok(Pending::new()),
            Some(f) => {
                f.call(key, value)
            }
        }
    }

    pub fn new() -> MatchCond<K, V, R> {
       MatchCond {
        name: None,
        cond: None
       }
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn with_cond(mut self, cond: impl Fn(&K, &V) -> Result<Pending<V,R>, Rbe1Error<K, V, R>> + Clone + 'static) -> Self {
        self.cond = Some(Box::new(cond));
        self
    }
} 


impl <K,V,R> PartialEq for MatchCond<K, V,R> 
where K: Hash + PartialEq + Eq + Display + Default,
V: Hash + Default + Eq + Clone, 
R: Default + PartialEq + Clone {
    fn eq(&self, other: &Self) -> bool { 
        self.name == other.name
    }
} 

impl <K,V,R> Eq for MatchCond<K, V,R> 
where K: Hash + PartialEq + Eq + Display + Default,
V: Hash + Eq + Default + Clone, 
R: Default + PartialEq + Clone {
}

impl <K, V, R> Default for MatchCond<K, V, R> 
where K: Hash + PartialEq + Eq + Display + Default,
      V: Hash + Default + Eq + Debug + Clone,
      R: Default + PartialEq + Debug + Clone {
    fn default() -> Self { 
        MatchCond::new()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_even_cond_2_pass() {
        let cond_even: MatchCond<char, i32, String> = MatchCond::new().with_cond(|k: &char, v| {
            if v % 2 == 0 {
                Ok(Pending::new())
            } else {
                Err(Rbe1Error::MsgError{ msg: format!("Value {v} for key {k} is not even") })
            }
        });

        assert_eq!(cond_even.matches(&'a',&2), Ok(Pending::new()));
    }

    #[test]
    fn test_even_cond_3_fail() {
        let cond_even: MatchCond<char, i32, String> = MatchCond::new().with_cond(|k: &char, v| {
            if v % 2 == 0 {
                Ok(Pending::new())
            } else {
                Err(Rbe1Error::MsgError{ msg: format!("Value {v} for key {k} is not even") })
            }
        });

        assert!(cond_even.matches(&'a',&3).is_err());
    }

    #[test]
    fn test_name_fail() {
        
        fn cond_name(name: String) -> MatchCond<char, String, String> {
            MatchCond::new().with_cond(move |k: &char, v: &String| {
            if *v == name {
                Ok(Pending::new())
            } else {
                Err(Rbe1Error::MsgError{ msg: format!("Value {v} for key {k} is not equal to {name}") })
            }
          })
        }

        assert!(cond_name("foo".to_string()).matches(&'a',&"baz".to_string()).is_err());
    }

    #[test]
    fn test_name_pass() {
        
        fn cond_name(name: String) -> MatchCond<char, String, String> {
            MatchCond::new().with_name("name".to_string()).with_cond(move |k: &char, v: &String| {
            if *v == name {
                Ok(Pending::new())
            } else {
                Err(Rbe1Error::MsgError{ msg: 
                    format!("Value {v} for key {k} failed condition is not equal to {name}", ) 
                })
            }
          })
        }

        assert_eq!(cond_name("foo".to_string()).matches(&'a',&"foo".to_string()), Ok(Pending::new()));
    }

}