use crate::{rbe_error::RbeError, Pending};
use core::hash::Hash;
use serde_derive::{Deserialize, Serialize};
use std::fmt::Debug;
use std::fmt::{Display, Formatter};

/// Represents a matching condition
#[derive(Serialize, Deserialize)]
pub struct MatchCond<K, V, R>
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
    K: Hash + Eq + Display + Default,
    V: Hash + Eq + Default + Display + PartialEq + Clone,
    R: Hash + Eq + Default + PartialEq + Display + Clone,
{
    fn clone_box(&self) -> Box<dyn Cond<K, V, R>>;
    fn call(&self, k: &K, v: &V) -> Result<Pending<V, R>, RbeError<K, V, R>>;
}

impl<K, V, R, F> Cond<K, V, R> for F
where
    K: Hash + Eq + Display + Default,
    V: Hash + Eq + Default + Display + PartialEq + Clone,
    R: Hash + Eq + Default + PartialEq + Display + Clone,
    F: 'static + Fn(&K, &V) -> Result<Pending<V, R>, RbeError<K, V, R>> + Clone,
{
    fn clone_box(&self) -> Box<dyn Cond<K, V, R>> {
        Box::new(self.clone())
    }

    fn call(&self, k: &K, v: &V) -> Result<Pending<V, R>, RbeError<K, V, R>> {
        self(k, v)
    }
}

impl<K, V, R> Clone for Box<dyn Cond<K, V, R>>
where
    K: Hash + Eq + Display + Default,
    V: Hash + Eq + Default + Display + PartialEq + Clone,
    R: Hash + Eq + Default + PartialEq + Display + Clone,
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

impl<K, V, R> Clone for MatchCond<K, V, R>
where
    K: Hash + Eq + Display + Default,
    V: Hash + Eq + Default + Display + PartialEq + Clone,
    R: Hash + Default + Eq + Display + Clone,
{
    fn clone(&self) -> Self {
        MatchCond {
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

impl<K, V, R> MatchCond<K, V, R>
where
    K: Hash + PartialEq + Eq + Display + Default,
    V: Hash + Default + Eq + Debug + Display + Clone,
    R: Hash + Default + Eq + PartialEq + Debug + Display + Clone,
{
    pub fn matches(&self, key: &K, value: &V) -> Result<Pending<V, R>, RbeError<K, V, R>> {
        self.cond.iter().fold(Ok(Pending::new()), |current, f| {
            current.and_then(|r| Ok(r.merge(f.call(key, value)?)))
        })
        /*        match &self.cond {
            None => Ok(Pending::new()),
            Some(f) => {
                f.call(key, value)
            }
        }*/
    }

    pub fn new() -> MatchCond<K, V, R> {
        MatchCond {
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
        cond: impl Fn(&K, &V) -> Result<Pending<V, R>, RbeError<K, V, R>> + Clone + 'static,
    ) -> Self {
        self.cond.push(Box::new(cond));
        self
    }

    pub fn name(&self) -> String {
        self.name.clone().unwrap_or_default()
    }
}

impl<K, V, R> PartialEq for MatchCond<K, V, R>
where
    K: Hash + PartialEq + Eq + Display + Default,
    V: Hash + Default + Eq + Clone,
    R: Hash + Default + PartialEq + Clone,
{
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl<K, V, R> Eq for MatchCond<K, V, R>
where
    K: Hash + PartialEq + Eq + Display + Default,
    V: Hash + Eq + Default + Clone,
    R: Hash + Default + PartialEq + Clone,
{
}

impl<K, V, R> Default for MatchCond<K, V, R>
where
    K: Hash + Eq + Display + Default,
    V: Hash + Default + Eq + Debug + Display + Clone,
    R: Hash + Default + Eq + Debug + Display + Clone,
{
    fn default() -> Self {
        MatchCond::new()
    }
}

impl<K, V, R> Debug for MatchCond<K, V, R>
where
    K: Hash + PartialEq + Eq + Display + Default,
    V: Hash + Default + Eq + Debug + Display + Clone,
    R: Hash + Default + PartialEq + Debug + Display + Clone,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.name.clone().unwrap_or_else(|| "".to_string()))?;
        Ok(())
    }
}

impl<K, V, R> Display for MatchCond<K, V, R>
where
    K: Hash + PartialEq + Eq + Display + Default,
    V: Hash + Default + Eq + Debug + Display + Clone,
    R: Hash + Default + PartialEq + Debug + Display + Clone,
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
        let cond_even: MatchCond<char, i32, String> = MatchCond::new().with_cond(|k: &char, v| {
            if v % 2 == 0 {
                Ok(Pending::new())
            } else {
                Err(RbeError::MsgError {
                    msg: format!("Value {v} for key {k} is not even"),
                })
            }
        });

        assert_eq!(cond_even.matches(&'a', &2), Ok(Pending::new()));
    }

    #[test]
    fn test_even_cond_3_fail() {
        let cond_even: MatchCond<char, i32, String> = MatchCond::new().with_cond(|k: &char, v| {
            if v % 2 == 0 {
                Ok(Pending::new())
            } else {
                Err(RbeError::MsgError {
                    msg: format!("Value {v} for key {k} is not even"),
                })
            }
        });

        assert!(cond_even.matches(&'a', &3).is_err());
    }

    #[test]
    fn test_name_fail() {
        fn cond_name(name: String) -> MatchCond<char, String, String> {
            MatchCond::new().with_cond(move |k: &char, v: &String| {
                if *v == name {
                    Ok(Pending::new())
                } else {
                    Err(RbeError::MsgError {
                        msg: format!("Value {v} for key {k} is not equal to {name}"),
                    })
                }
            })
        }

        assert!(cond_name("foo".to_string())
            .matches(&'a', &"baz".to_string())
            .is_err());
    }

    #[test]
    fn test_name_pass() {
        fn cond_name(name: String) -> MatchCond<char, String, String> {
            MatchCond::new()
                .with_name("name".to_string())
                .with_cond(move |k: &char, v: &String| {
                    if *v == name {
                        Ok(Pending::new())
                    } else {
                        Err(RbeError::MsgError {
                            msg: format!(
                                "Value {v} for key {k} failed condition is not equal to {name}",
                            ),
                        })
                    }
                })
        }

        assert_eq!(
            cond_name("foo".to_string()).matches(&'a', &"foo".to_string()),
            Ok(Pending::new())
        );
    }
}
