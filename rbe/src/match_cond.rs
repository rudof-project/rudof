use crate::{Context, Key, Ref, State, Value};
use crate::{Pending, rbe_error::RbeError};
use core::hash::Hash;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::fmt::{Display, Formatter};
use std::hash::Hasher;

/// A `MatchCond` represents a matching condition
/// It can be a single condition, a reference to another condition, or several conditions
/// The `matches` method checks if a value matches the condition and returns a `Pending` with some pending references if it does, or an error if it doesn't.
///
/// The `Pending` struct is used to keep track of pending references that need to be resolved in order to determine if the condition is fully satisfied.
/// The `RbeError` struct is used to represent errors that can occur during the matching process.
/// The `Key`, `Value`, and `Ref` traits are used to represent the types of keys, values, and references that can be used in the conditions.
#[derive(PartialEq, Eq, Hash, Clone, Serialize, Debug, Deserialize)]
pub enum MatchCond<K, V, R, Ctx, St>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    St: State,
{
    Single(SingleCond<K, V, R, Ctx, St>),
    Ref(R),
    And(Vec<MatchCond<K, V, R, Ctx, St>>),
}

unsafe impl<K, V, R, Ctx, St> Sync for MatchCond<K, V, R, Ctx, St>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    St: State,
{
}

impl<K, V, R, Ctx, St> MatchCond<K, V, R, Ctx, St>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    St: State,
{
    pub fn new() -> MatchCond<K, V, R, Ctx, St> {
        MatchCond::Single(SingleCond::new())
    }

    pub fn and(conds: Vec<MatchCond<K, V, R, Ctx, St>>) -> MatchCond<K, V, R, Ctx, St> {
        MatchCond::And(conds)
    }

    pub fn empty() -> MatchCond<K, V, R, Ctx, St> {
        MatchCond::Single(SingleCond::new().with_name("empty"))
    }

    pub fn ref_(r: R) -> MatchCond<K, V, R, Ctx, St> {
        MatchCond::Ref(r)
    }

    pub fn matches(&self, value: &V, ctx: &Ctx, state: &St) -> Result<Pending<V, R>, RbeError<K, V, R, Ctx, St>> {
        match self {
            MatchCond::Single(single) => single.matches(value, ctx, state),
            MatchCond::Ref(r) => Ok(Pending::from_pair(value.clone(), r.clone())),
            MatchCond::And(vs) => vs.iter().try_fold(Pending::new(), |mut current, cond| {
                let new_pending = cond.matches(value, ctx, state)?;
                current.merge(new_pending);
                Ok(current)
            }),
        }
    }

    pub fn single(single: SingleCond<K, V, R, Ctx, St>) -> Self {
        MatchCond::Single(single)
    }

    pub fn simple(
        name: &str,
        cond: impl Fn(&V, &Ctx, &St) -> Result<Pending<V, R>, RbeError<K, V, R, Ctx, St>> + Clone + 'static + Send + Sync,
    ) -> Self {
        MatchCond::single(SingleCond::new().with_name(name).with_cond(cond))
    }

    pub fn show(&self) -> String {
        format!("{}", self)
    }
}

impl<K, V, R, Ctx, St> Display for MatchCond<K, V, R, Ctx, St>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    St: State,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MatchCond::Single(sc) => {
                write!(f, "{sc}")?;
                Ok(())
            },
            MatchCond::Ref(r) => {
                write!(f, "@{r}")?;
                Ok(())
            },
            MatchCond::And(cs) => {
                write!(f, "And(")?;
                cs.iter().try_fold((), |_, c| write!(f, "|{c}"))?;
                write!(f, ")")
            },
        }
    }
}

impl<K, V, R, Ctx, St> Default for MatchCond<K, V, R, Ctx, St>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    St: State,
{
    fn default() -> Self {
        MatchCond::Single(SingleCond::default())
    }
}

/// Represents a simple condition
#[derive(Serialize, Deserialize)]
pub struct SingleCond<K, V, R, Ctx, St>
where
    K: Hash + Eq + Display + Default,
    V: Hash + Eq + Default + PartialEq + Clone,
    R: Hash + Default + PartialEq + Clone,
    Ctx: Hash + Default + PartialEq + Clone,
    St: State,
{
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip)]
    cond: Vec<Box<dyn Cond<K, V, R, Ctx, St> + Send + Sync>>,
}

unsafe impl<K, V, R, Ctx, St> Sync for SingleCond<K, V, R, Ctx, St>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    St: State,
{
}

/// We use trait objects instead of function pointers because we need to
/// capture some values in the condition closure.
/// This pattern is inspired by the answer in this thread:
/// https://users.rust-lang.org/t/how-to-clone-a-boxed-closure/31035
trait Cond<K, V, R, Ctx, St>: Send + Sync
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    St: State,
{
    fn clone_box(&self) -> Box<dyn Cond<K, V, R, Ctx, St> + Send + Sync>;
    fn call(&self, v: &V, ctx: &Ctx, st: &St) -> Result<Pending<V, R>, RbeError<K, V, R, Ctx, St>>;
}

impl<K, V, R, F, Ctx, St> Cond<K, V, R, Ctx, St> for F
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    St: State,
    F: 'static + Fn(&V, &Ctx, &St) -> Result<Pending<V, R>, RbeError<K, V, R, Ctx, St>> + Clone + Send + Sync,
{
    fn clone_box(&self) -> Box<dyn Cond<K, V, R, Ctx, St> + Send + Sync> {
        Box::new(self.clone())
    }

    fn call(&self, v: &V, ctx: &Ctx, st: &St) -> Result<Pending<V, R>, RbeError<K, V, R, Ctx, St>> {
        self(v, ctx, st)
    }
}

impl<K, V, R, Ctx, St> Clone for Box<dyn Cond<K, V, R, Ctx, St> + Send + Sync>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    St: State,
{
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl<K, V, R, Ctx, St> Clone for SingleCond<K, V, R, Ctx, St>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    St: State,
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
            },
        }
    }
}

impl<K, V, R, Ctx, St> SingleCond<K, V, R, Ctx, St>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    St: State,
{
    pub fn matches(&self, value: &V, ctx: &Ctx, st: &St) -> Result<Pending<V, R>, RbeError<K, V, R, Ctx, St>> {
        self.cond.iter().try_fold(Pending::new(), |mut current, f| {
            let pending = f.call(value, ctx, st)?;
            current.merge(pending);
            Ok(current)
        })
    }

    pub fn new() -> SingleCond<K, V, R, Ctx, St> {
        SingleCond {
            name: None,
            cond: Vec::new(),
        }
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn with_cond(
        mut self,
        cond: impl Fn(&V, &Ctx, &St) -> Result<Pending<V, R>, RbeError<K, V, R, Ctx, St>> + Clone + 'static + Send + Sync,
    ) -> Self {
        self.cond.push(Box::new(cond));
        self
    }

    pub fn name(&self) -> String {
        self.name.clone().unwrap_or_default()
    }
}

impl<K, V, R, Ctx, St> PartialEq for SingleCond<K, V, R, Ctx, St>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    St: State,
{
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl<K, V, R, Ctx, St> Eq for SingleCond<K, V, R, Ctx, St>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    St: State,
{
}

impl<K, V, R, Ctx, St> Hash for SingleCond<K, V, R, Ctx, St>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    St: State,
{
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.name.hash(hasher)
    }
}

impl<K, V, R, Ctx, St> Default for SingleCond<K, V, R, Ctx, St>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    St: State,
{
    fn default() -> Self {
        SingleCond::new()
    }
}

impl<K, V, R, Ctx, St> Debug for SingleCond<K, V, R, Ctx, St>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    St: State,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.name.clone().unwrap_or_default())?;
        Ok(())
    }
}

impl<K, V, R, Ctx, St> Display for SingleCond<K, V, R, Ctx, St>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    St: State,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.name.clone().unwrap_or_default())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl State for char {}

    #[test]
    fn test_even_cond_2_pass() {
        let cond_even: SingleCond<char, i32, String, char, char> =
            SingleCond::new().with_cond(|v, _ctx, _st: &char| {
                if v % 2 == 0 {
                    Ok(Pending::new())
                } else {
                    Err(RbeError::MsgError {
                        msg: format!("Value {v} is not even"),
                    })
                }
            });

        assert_eq!(cond_even.matches(&2, &'a', &'z'), Ok(Pending::new()));
    }

    #[test]
    fn test_even_cond_3_fail() {
        let cond_even: SingleCond<char, i32, String, char, char> =
            SingleCond::new().with_cond(|v, _ctx, _st: &char| {
                if v % 2 == 0 {
                    Ok(Pending::new())
                } else {
                    Err(RbeError::MsgError {
                        msg: format!("Value {v} is not even"),
                    })
                }
            });

        assert!(cond_even.matches(&3, &'a', &'z').is_err());
    }

    #[test]
    fn test_name_fail() {
        fn cond_name(name: String) -> SingleCond<char, String, String, char, char> {
            SingleCond::new().with_cond(move |v: &String, _ctx: &char, _st: &char| {
                if *v == name {
                    Ok(Pending::new())
                } else {
                    Err(RbeError::MsgError {
                        msg: format!("Value {v} is not equal to {name}"),
                    })
                }
            })
        }

        assert!(
            cond_name("foo".to_string())
                .matches(&"baz".to_string(), &'a', &'z')
                .is_err()
        );
    }

    #[test]
    fn test_name_pass() {
        fn cond_name(name: String) -> SingleCond<char, String, String, char, char> {
            SingleCond::new()
                .with_name("name")
                .with_cond(move |v: &String, _ctx: &char, _st: &char| {
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
            cond_name("foo".to_string()).matches(&"foo".to_string(), &'a', &'z'),
            Ok(Pending::new())
        );
    }
}
