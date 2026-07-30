use crate::Context;
use crate::match_cond::MatchKind;
use crate::{Key, Ref, Value, rbe_cond::RbeCond};
use crate::{Pending, rbe_error::RbeError};
use core::hash::Hash;
use serde::Serialize;
use std::collections::HashSet;
use std::fmt::Debug;
use tracing::debug;

pub struct RbeCondMatcher<K, V, R, Ctx, P = ()>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    P: MatchKind<K, V, R, Ctx> + Clone + PartialEq + Eq + Hash + Debug + Serialize,
{
    rbe: RbeCond<K, V, R, Ctx, P>,
    open: bool,
    controlled: HashSet<K>,
}

impl<K, V, R, Ctx, P> Default for RbeCondMatcher<K, V, R, Ctx, P>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    P: MatchKind<K, V, R, Ctx> + Clone + PartialEq + Eq + Hash + Debug + Serialize,
{
    fn default() -> Self {
        Self {
            rbe: RbeCond::default(),
            open: false,
            controlled: HashSet::new(),
        }
    }
}

impl<K, V, R, Ctx, P> RbeCondMatcher<K, V, R, Ctx, P>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    P: MatchKind<K, V, R, Ctx> + Clone + PartialEq + Eq + Hash + Debug + Serialize,
{
    pub fn new() -> RbeCondMatcher<K, V, R, Ctx, P> {
        RbeCondMatcher::default()
    }

    pub fn with_rbe(mut self, rbe: &RbeCond<K, V, R, Ctx, P>) -> Self {
        self.rbe = (*rbe).clone();
        self.controlled = self.rbe.symbols();
        self
    }

    pub fn extend_controlled(mut self, controlled: HashSet<K>) -> Self {
        self.controlled.extend(controlled);
        self
    }

    pub fn with_open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    pub fn matches<T: IntoIterator<Item = (K, V, Ctx)>>(
        &self,
        iter: T,
    ) -> Result<Pending<K, V, R>, RbeError<K, V, R, Ctx, P>> {
        let mut pending = Pending::empty();
        let mut processed: Vec<(K, V, Ctx)> = Vec::new();
        match self.matches_iter(iter, &mut pending, &mut processed) {
            RbeCond::Fail { error } => Err(error.clone()),
            d => {
                if d.nullable() {
                    Ok(pending)
                } else {
                    Err(RbeError::NonNullableMatch {
                        non_nullable_rbe: Box::new(d.clone()),
                        expr: Box::new(self.rbe.clone()),
                    })
                }
            },
        }
    }

    fn matches_iter<T: IntoIterator<Item = (K, V, Ctx)>>(
        &self,
        iter: T,
        pending: &mut Pending<K, V, R>,
        processed: &mut Vec<(K, V, Ctx)>,
    ) -> RbeCond<K, V, R, Ctx, P> {
        let mut current = self.rbe.clone();
        for (key, value, ctx) in iter {
            let deriv = current.deriv(&key, &value, &ctx, 1, self.open, &self.controlled, pending);
            match deriv {
                RbeCond::Fail { error } => {
                    current = RbeCond::Fail {
                        error: RbeError::DerivIterError {
                            error_msg: format!("{error}"),
                            processed: processed.clone(),
                            expr: Box::new(self.rbe.clone()),
                            current: Box::new(current.clone()),
                            key: key.clone(),
                            open: self.open,
                        },
                    };
                    break;
                },
                _ => {
                    debug!("Processing: {key}/{value}\ncurrent:{current}\nderiv:{deriv}");
                    processed.push((key.clone(), value.clone(), ctx.clone()));
                    current = deriv;
                },
            }
        }
        current
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{MatchCond, Max, Min, SingleCond};
    use serde::Deserialize;

    impl Value for i32 {}
    impl Ref for String {}
    impl Value for String {}

    /// Test payload for `i32` values.
    #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    enum IntKind {
        Even,
        RefX,
    }

    impl MatchKind<char, i32, String, char> for IntKind {
        fn eval(
            &self,
            v: &i32,
            _ctx: &char,
        ) -> Result<Pending<char, i32, String>, RbeError<char, i32, String, char, Self>> {
            match self {
                IntKind::Even => {
                    if v % 2 == 0 {
                        Ok(Pending::empty())
                    } else {
                        Err(RbeError::MsgError {
                            msg: format!("Value {v} is not even"),
                        })
                    }
                },
                IntKind::RefX => Ok(Pending::from_pair(*v, "X".to_string())),
            }
        }
    }

    /// Test payload for `String` values.
    #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    enum StrKind {
        EqualsName(String),
        HasLength(usize),
    }

    impl MatchKind<char, String, String, char> for StrKind {
        fn eval(
            &self,
            v: &String,
            _ctx: &char,
        ) -> Result<Pending<char, String, String>, RbeError<char, String, String, char, Self>> {
            match self {
                StrKind::EqualsName(name) => {
                    if v == name {
                        Ok(Pending::empty())
                    } else {
                        Err(RbeError::MsgError {
                            msg: format!("Value {v} is not equal to {name}"),
                        })
                    }
                },
                StrKind::HasLength(len) => {
                    if v.len() == *len {
                        Ok(Pending::empty())
                    } else {
                        Err(RbeError::MsgError {
                            msg: format!("Value {v} has no length {len}"),
                        })
                    }
                },
            }
        }
    }

    fn cond_name(name: String) -> MatchCond<char, String, String, char, StrKind> {
        MatchCond::single(SingleCond::new().with_kind(StrKind::EqualsName(name)))
    }

    fn cond_len(len: usize) -> MatchCond<char, String, String, char, StrKind> {
        MatchCond::single(SingleCond::new().with_kind(StrKind::HasLength(len)))
    }

    #[test]
    fn test_rbe_matcher_len_name() {
        let rbe: RbeCond<char, String, String, char, StrKind> = RbeCond::and(vec![
            RbeCond::symbol_cond('a', cond_len(3), Min::from(1), Max::IntMax(1)),
            RbeCond::symbol_cond('b', cond_name("foo".to_string()), Min::from(0), Max::IntMax(1)),
        ]);
        let expected = Pending::empty();
        let rbe_matcher = RbeCondMatcher::new().with_rbe(&rbe);

        assert_eq!(
            rbe_matcher.matches(vec![('a', "baz".to_string(), 'a')].into_iter()),
            Ok(expected)
        );
    }

    #[test]
    fn test_rbe_matcher_even_ref() {
        let cond_even = MatchCond::single(SingleCond::new().with_kind(IntKind::Even));
        let cond_ref_x = MatchCond::single(SingleCond::new().with_kind(IntKind::RefX));

        let rbe: RbeCond<char, i32, String, char, IntKind> = RbeCond::and(vec![
            RbeCond::symbol_cond('a', cond_even, Min::from(1), Max::IntMax(1)),
            RbeCond::symbol_cond('b', cond_ref_x, Min::from(0), Max::IntMax(1)),
        ]);
        let mut expected = Pending::new();
        expected.insert_with_key(42, "X".to_string(), 'b');
        let rbe_matcher = RbeCondMatcher::new().with_rbe(&rbe);

        assert_eq!(
            rbe_matcher.matches(vec![('a', 2, 'a'), ('b', 42, 'b')].into_iter()),
            Ok(expected)
        );
    }

    #[test]
    fn test_rbe_matcher_even_ref_unordered() {
        let cond_even = MatchCond::single(SingleCond::new().with_kind(IntKind::Even));
        let cond_ref_x = MatchCond::single(SingleCond::new().with_kind(IntKind::RefX));

        let rbe: RbeCond<char, i32, String, char, IntKind> = RbeCond::and(vec![
            RbeCond::symbol_cond('a', cond_even, Min::from(1), Max::IntMax(1)),
            RbeCond::symbol_cond('b', cond_ref_x, Min::from(0), Max::IntMax(1)),
        ]);
        let mut expected = Pending::new();
        expected.insert_with_key(42, "X".to_string(), 'b');
        let rbe_matcher = RbeCondMatcher::new().with_rbe(&rbe);

        assert_eq!(
            rbe_matcher.matches(vec![('b', 42, 'b'), ('a', 2, 'a')].into_iter()),
            Ok(expected)
        );
    }

    #[test]
    fn test_rbe_matcher_even_fails_odd() {
        let cond_even = MatchCond::single(SingleCond::new().with_kind(IntKind::Even));
        let cond_ref_x = MatchCond::single(SingleCond::new().with_kind(IntKind::RefX));

        let rbe: RbeCond<char, i32, String, char, IntKind> = RbeCond::and(vec![
            RbeCond::symbol_cond('a', cond_even, Min::from(1), Max::IntMax(1)),
            RbeCond::symbol_cond('b', cond_ref_x, Min::from(0), Max::IntMax(1)),
        ]);
        let rbe_matcher = RbeCondMatcher::new().with_rbe(&rbe);
        let iter = vec![('b', 42, 'b'), ('a', 3, 'a')].into_iter();
        assert!(rbe_matcher.matches(iter).is_err());
    }
}
