use crate::{Context, Key, Ref, Value};
use crate::{Pending, rbe_error::RbeError};
use core::hash::Hash;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;

/// A payload carried by a `SingleCond`. Implementations decide how a value matches.
pub trait MatchKind<K, V, R, Ctx>: Sized + Clone + PartialEq + Eq + Hash + Debug + Serialize
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
{
    fn eval(&self, v: &V, ctx: &Ctx) -> Result<Pending<K, V, R>, RbeError<K, V, R, Ctx, Self>>;
}

/// Default no-op payload.
impl<K, V, R, Ctx> MatchKind<K, V, R, Ctx> for ()
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
{
    fn eval(&self, _v: &V, _ctx: &Ctx) -> Result<Pending<K, V, R>, RbeError<K, V, R, Ctx, ()>> {
        Ok(Pending::empty())
    }
}

/// A `MatchCond` represents a matching condition
/// It can be a single condition, a reference to another condition, or several conditions
/// The `matches` method checks if a value matches the condition and returns a `Pending` with some pending references if it does, or an error if it doesn't.
///
/// The `Pending` struct is used to keep track of pending references that need to be resolved in order to determine if the condition is fully satisfied.
/// The `RbeError` struct is used to represent errors that can occur during the matching process.
/// The `Key`, `Value`, and `Ref` traits are used to represent the types of keys, values, and references that can be used in the conditions.
#[derive(PartialEq, Eq, Hash, Clone, Serialize, Debug, Deserialize)]
pub enum MatchCond<K, V, R, Ctx, P = ()>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    P: MatchKind<K, V, R, Ctx> + Clone + PartialEq + Eq + Hash + Debug + Serialize,
{
    Single(SingleCond<K, V, R, Ctx, P>),
    Ref(R),
    And(Vec<MatchCond<K, V, R, Ctx, P>>),
}

impl<K, V, R, Ctx, P> MatchCond<K, V, R, Ctx, P>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    P: MatchKind<K, V, R, Ctx> + Clone + PartialEq + Eq + Hash + Debug + Serialize,
{
    pub fn new() -> MatchCond<K, V, R, Ctx, P> {
        MatchCond::Single(SingleCond::new())
    }

    pub fn and(conds: Vec<MatchCond<K, V, R, Ctx, P>>) -> MatchCond<K, V, R, Ctx, P> {
        MatchCond::And(conds)
    }

    pub fn empty() -> MatchCond<K, V, R, Ctx, P> {
        MatchCond::Single(SingleCond::new().with_name("empty"))
    }

    pub fn ref_(r: R) -> MatchCond<K, V, R, Ctx, P> {
        MatchCond::Ref(r)
    }

    pub fn matches(&self, value: &V, ctx: &Ctx) -> Result<Pending<K, V, R>, RbeError<K, V, R, Ctx, P>> {
        match self {
            MatchCond::Single(single) => single.matches(value, ctx),
            MatchCond::Ref(r) => Ok(Pending::from_pair(value.clone(), r.clone())),
            MatchCond::And(vs) => vs.iter().try_fold(Pending::empty(), |mut current, cond| {
                let new_pending = cond.matches(value, ctx)?;
                current.merge(new_pending);
                Ok(current)
            }),
        }
    }

    pub fn single(single: SingleCond<K, V, R, Ctx, P>) -> Self {
        MatchCond::Single(single)
    }

    pub fn show(&self) -> String {
        format!("{}", self)
    }
}

impl<K, V, R, Ctx, P> Display for MatchCond<K, V, R, Ctx, P>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    P: MatchKind<K, V, R, Ctx> + Clone + PartialEq + Eq + Hash + Debug + Serialize,
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

impl<K, V, R, Ctx, P> Default for MatchCond<K, V, R, Ctx, P>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    P: MatchKind<K, V, R, Ctx> + Clone + PartialEq + Eq + Hash + Debug + Serialize,
{
    fn default() -> Self {
        MatchCond::Single(SingleCond::default())
    }
}

/// Represents a simple condition
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SingleCond<K, V, R, Ctx, P = ()>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    P: MatchKind<K, V, R, Ctx> + Clone + PartialEq + Eq + Hash + Debug + Serialize,
{
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    kind: Option<P>,

    #[serde(skip)]
    _phantom: PhantomData<(K, V, R, Ctx)>,
}

impl<K, V, R, Ctx, P> SingleCond<K, V, R, Ctx, P>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    P: MatchKind<K, V, R, Ctx> + Clone + PartialEq + Eq + Hash + Debug + Serialize,
{
    pub fn new() -> Self {
        SingleCond {
            name: None,
            kind: None,
            _phantom: PhantomData,
        }
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn with_kind(mut self, kind: P) -> Self {
        self.kind = Some(kind);
        self
    }

    pub fn kind(&self) -> Option<&P> {
        self.kind.as_ref()
    }

    pub fn name(&self) -> String {
        self.name.clone().unwrap_or_default()
    }

    pub fn matches(&self, value: &V, ctx: &Ctx) -> Result<Pending<K, V, R>, RbeError<K, V, R, Ctx, P>> {
        match &self.kind {
            Some(kind) => kind.eval(value, ctx),
            None => Ok(Pending::empty()),
        }
    }
}

impl<K, V, R, Ctx, P> Default for SingleCond<K, V, R, Ctx, P>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    P: MatchKind<K, V, R, Ctx> + Clone + PartialEq + Eq + Hash + Debug + Serialize,
{
    fn default() -> Self {
        SingleCond::new()
    }
}

impl<K, V, R, Ctx, P> Debug for SingleCond<K, V, R, Ctx, P>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    P: MatchKind<K, V, R, Ctx> + Clone + PartialEq + Eq + Hash + Debug + Serialize,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match (&self.name, &self.kind) {
            (Some(name), Some(kind)) => write!(f, "SingleCond {{ name: {name}, kind: {kind:?} }}"),
            (Some(name), None) => write!(f, "SingleCond {{ name: {name}, kind: None }}"),
            (None, Some(kind)) => write!(f, "SingleCond {{ name: None, kind: {kind:?} }}"),
            (None, None) => write!(f, "SingleCond {{ name: None, kind: None }}"),
        }
    }
}

impl<K, V, R, Ctx, P> Display for SingleCond<K, V, R, Ctx, P>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    P: MatchKind<K, V, R, Ctx> + Clone + PartialEq + Eq + Hash + Debug + Serialize,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.name.clone().unwrap_or_default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    enum TestKind {
        Even,
        EqualsName(String),
    }

    impl MatchKind<char, i32, String, char> for TestKind {
        fn eval(
            &self,
            v: &i32,
            _ctx: &char,
        ) -> Result<Pending<char, i32, String>, RbeError<char, i32, String, char, Self>> {
            match self {
                TestKind::Even => {
                    if v % 2 == 0 {
                        Ok(Pending::empty())
                    } else {
                        Err(RbeError::MsgError {
                            msg: format!("Value {v} is not even"),
                        })
                    }
                },
                TestKind::EqualsName(_) => unreachable!("EqualsName should not be used with i32"),
            }
        }
    }

    impl MatchKind<char, String, String, char> for TestKind {
        fn eval(
            &self,
            v: &String,
            _ctx: &char,
        ) -> Result<Pending<char, String, String>, RbeError<char, String, String, char, Self>> {
            match self {
                TestKind::EqualsName(name) => {
                    if v == name {
                        Ok(Pending::empty())
                    } else {
                        Err(RbeError::MsgError {
                            msg: format!("Value {v} is not equal to {name}"),
                        })
                    }
                },
                TestKind::Even => unreachable!("Even should not be used with String"),
            }
        }
    }

    #[test]
    fn test_even_cond_2_pass() {
        let cond_even: SingleCond<char, i32, String, char, TestKind> = SingleCond::new().with_kind(TestKind::Even);
        assert_eq!(cond_even.matches(&2, &'a'), Ok(Pending::empty()));
    }

    #[test]
    fn test_even_cond_3_fail() {
        let cond_even: SingleCond<char, i32, String, char, TestKind> = SingleCond::new().with_kind(TestKind::Even);
        assert!(cond_even.matches(&3, &'a').is_err());
    }

    #[test]
    fn test_name_fail() {
        fn cond_name(name: String) -> SingleCond<char, String, String, char, TestKind> {
            SingleCond::new().with_kind(TestKind::EqualsName(name))
        }
        assert!(cond_name("foo".to_string()).matches(&"baz".to_string(), &'a').is_err());
    }

    #[test]
    fn test_name_pass() {
        fn cond_name(name: String) -> SingleCond<char, String, String, char, TestKind> {
            SingleCond::new()
                .with_name("name")
                .with_kind(TestKind::EqualsName(name))
        }
        assert_eq!(
            cond_name("foo".to_string()).matches(&"foo".to_string(), &'a'),
            Ok(Pending::empty())
        );
    }
}
