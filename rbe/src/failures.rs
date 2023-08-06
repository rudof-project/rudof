use crate::rbe::Rbe;
use crate::rbe_error::RbeError;
use crate::Cardinality;
use std::fmt::Formatter;
use std::hash::Hash;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use thiserror::Error;
use std::fmt::Display;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Failures<K, V, R> 
where K: Hash + Eq + Display + Default,
      V: Hash + Default + Eq + Clone,
      R: Default + PartialEq + Clone
{
   fs: Vec<(Box<Rbe<K,V,R>>, RbeError<K,V,R>)>
}

impl <K, V, R> Failures<K, V, R> 
where K: Hash + Eq + Display + Default,
      V: Hash + Default + Eq + Clone,
      R: Default + PartialEq + Clone
{
    pub fn new() -> Failures<K, V, R> {
       Failures {
        fs: Vec::new()
       }
    }

    pub fn push(&mut self, expr: Rbe<K, V, R>, err: RbeError<K, V, R>) {
        self.fs.push((Box::new(expr), err));
    }
}

impl <K, V, R> Display for Failures<K, V, R> 
where K: Hash + Eq + Display + Display + Default,
      V: Hash + Default + Display + Eq + Clone,
      R: Default + Display + PartialEq + Clone
      {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { 
        for (expr, err) in &self.fs {
            write!(dest, "Error at {expr}: {err}\n")?;
        }
        Ok(())
    }
}
