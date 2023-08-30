use crate::{rbe_error::RbeError, Pending};
use crate::{Key, Ref, Value};
use core::hash::Hash;
use serde_derive::{Deserialize, Serialize};
use std::fmt::Debug;
use std::fmt::{Display, Formatter};
use std::hash::Hasher;

#[derive(PartialEq, Eq, Hash, Clone, Serialize, Debug, Deserialize)]
pub enum MatchCond<K, V, R>
where
    K: Key,
    V: Value,
    R: Ref,
{
    Single(SingleCond<K, V, R>),
    And(Vec<MatchCond<K, V, R>>),
    Or(Vec<MatchCond<K, V, R>>),
    Not(Box<MatchCond<K, V, R>>),
}

impl<K, V, R> MatchCond<K, V, R>
where
    K: Key,
    V: Value,
    R: Ref,
{
    pub fn new() -> MatchCond<K, V, R> {
        MatchCond::Single(SingleCond::new())
    }

    pub fn empty() -> MatchCond<K, V, R> {
        MatchCond::Single(SingleCond::new().with_name("empty".into()))
    }

    pub fn matches(&self, value: &V) -> Result<Pending<V, R>, RbeError<K, V, R>> {
        match self {
            MatchCond::Single(single) => single.matches(value),
            MatchCond::And(vs) => vs.iter().fold(Ok(Pending::new()), |result, c| {
                result.and_then(|mut p| {
                    let new_pending = c.matches(value)?;
                    p.merge(new_pending);
                    Ok(p)
                })
            }),
            _ => {
                todo!()
            }
        }
    }

    pub fn single(single: SingleCond<K, V, R>) -> Self {
        MatchCond::Single(single)
    }
}

impl<K, V, R> Display for MatchCond<K, V, R>
where
    K: Key,
    V: Value,
    R: Ref,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MatchCond::Single(sc) => {
                write!(f, "{sc}")?;
                Ok(())
            }
            MatchCond::And(cs) => {
                write!(f, "And(")?;
                cs.iter()
                    .fold(Ok(()), |result, c| result.and_then(|_| write!(f, "|{c}")))?;
                write!(f, ")")
            }
            MatchCond::Or(cs) => {
                write!(f, "Or")?;
                cs.iter()
                    .fold(Ok(()), |result, c| result.and_then(|_| write!(f, "|{c}")))?;
                write!(f, ")")
            }
            MatchCond::Not(c) => write!(f, "Not({c})"),
        }
    }
}

impl<K, V, R> Default for MatchCond<K, V, R>
where
    K: Key,
    V: Value,
    R: Ref,
{
    fn default() -> Self {
        MatchCond::Single(SingleCond::default())
    }
}

/// Represents a matching condition
#[derive(Serialize, Deserialize)]
pub struct SingleCond<K, V, R>
where
    K: Hash + Eq + Display + Default,
    V: Hash + Eq + Default + PartialEq + Clone,
    R: Hash + Default + PartialEq + Clone,
{
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip)]
    cond: Vec<Box<dyn Cond<K, V, R>>>,
}

/// We use trait objects instead of function pointers because we need to
/// capture some values in the condition closure.
/// This pattern is inspired by the answer in this thread:
/// https://users.rust-lang.org/t/how-to-clone-a-boxed-closure/31035
trait Cond<K, V, R>
where
    K: Key,
    V: Value,
    R: Ref,
{
    fn clone_box(&self) -> Box<dyn Cond<K, V, R>>;
    fn call(&self, v: &V) -> Result<Pending<V, R>, RbeError<K, V, R>>;
}

impl<K, V, R, F> Cond<K, V, R> for F
where
    K: Key,
    V: Value,
    R: Ref,
    F: 'static + Fn(&V) -> Result<Pending<V, R>, RbeError<K, V, R>> + Clone,
{
    fn clone_box(&self) -> Box<dyn Cond<K, V, R>> {
        Box::new(self.clone())
    }

    fn call(&self, v: &V) -> Result<Pending<V, R>, RbeError<K, V, R>> {
        self(v)
    }
}

impl<K, V, R> Clone for Box<dyn Cond<K, V, R>>
where
    K: Key,
    V: Value,
    R: Ref,
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

impl<K, V, R> Clone for SingleCond<K, V, R>
where
    K: Key,
    V: Value,
    R: Ref,
{
    fn clone(&self) -> Self {
        SingleCond {
            name: self.name.clone(),
            cond: {
                let mut r = Vec::new();
                for c in self.cond.iter() {
                    r.push(c.clone())
                }
                r
                // self.cond.into_iter().map(|c| (*c).clone_box()).collect()
            }, /*            match &self.cond {
                   Option::None => {
                       None
                   },
                   Option::Some(f) => {
                       Some((*f).clone())
                   }
               } */
        }
    }
}

impl<K, V, R> SingleCond<K, V, R>
where
    K: Key,
    V: Value,
    R: Ref,
{
    pub fn matches(&self, value: &V) -> Result<Pending<V, R>, RbeError<K, V, R>> {
        self.cond.iter().fold(Ok(Pending::new()), |current, f| {
            current.and_then(|mut r| {
                let pending = f.call(value)?;
                r.merge(pending);
                Ok(r)
            })
        })
    }

    pub fn new() -> SingleCond<K, V, R> {
        SingleCond {
            name: None,
            cond: Vec::new(),
        }
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn with_cond(
        mut self,
        cond: impl Fn(&V) -> Result<Pending<V, R>, RbeError<K, V, R>> + Clone + 'static,
    ) -> Self {
        self.cond.push(Box::new(cond));
        self
    }

    pub fn name(&self) -> String {
        self.name.clone().unwrap_or_default()
    }
}

impl<K, V, R> PartialEq for SingleCond<K, V, R>
where
    K: Key,
    V: Value,
    R: Ref,
{
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl<K, V, R> Eq for SingleCond<K, V, R>
where
    K: Key,
    V: Value,
    R: Ref,
{
}

impl<K, V, R> Hash for SingleCond<K, V, R>
where
    K: Key,
    V: Value,
    R: Ref,
{
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.name.hash(hasher)
    }
}

impl<K, V, R> Default for SingleCond<K, V, R>
where
    K: Key,
    V: Value,
    R: Ref,
{
    fn default() -> Self {
        SingleCond::new()
    }
}

impl<K, V, R> Debug for SingleCond<K, V, R>
where
    K: Key,
    V: Value,
    R: Ref,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.name.clone().unwrap_or_else(|| "".to_string()))?;
        Ok(())
    }
}

impl<K, V, R> Display for SingleCond<K, V, R>
where
    K: Key,
    V: Value,
    R: Ref,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.name.clone().unwrap_or_else(|| "".to_string()))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_even_cond_2_pass() {
        let cond_even: SingleCond<char, i32, String> = SingleCond::new().with_cond(|v| {
            if v % 2 == 0 {
                Ok(Pending::new())
            } else {
                Err(RbeError::MsgError {
                    msg: format!("Value {v} is not even"),
                })
            }
        });

        assert_eq!(cond_even.matches(&2), Ok(Pending::new()));
    }

    #[test]
    fn test_even_cond_3_fail() {
        let cond_even: SingleCond<char, i32, String> = SingleCond::new().with_cond(|v| {
            if v % 2 == 0 {
                Ok(Pending::new())
            } else {
                Err(RbeError::MsgError {
                    msg: format!("Value {v} is not even"),
                })
            }
        });

        assert!(cond_even.matches(&3).is_err());
    }

    #[test]
    fn test_name_fail() {
        fn cond_name(name: String) -> SingleCond<char, String, String> {
            SingleCond::new().with_cond(move |v: &String| {
                if *v == name {
                    Ok(Pending::new())
                } else {
                    Err(RbeError::MsgError {
                        msg: format!("Value {v} is not equal to {name}"),
                    })
                }
            })
        }

        assert!(cond_name("foo".to_string())
            .matches(&"baz".to_string())
            .is_err());
    }

    #[test]
    fn test_name_pass() {
        fn cond_name(name: String) -> SingleCond<char, String, String> {
            SingleCond::new()
                .with_name("name".to_string())
                .with_cond(move |v: &String| {
                    if *v == name {
                        Ok(Pending::new())
                    } else {
                        Err(RbeError::MsgError {
                            msg: format!("Value {v} failed condition is not equal to {name}",),
                        })
                    }
                })
        }

        assert_eq!(
            cond_name("foo".to_string()).matches(&"foo".to_string()),
            Ok(Pending::new())
        );
    }
}
