use std::collections::HashSet;
use tracing::debug;

use crate::{Context, State};
use crate::{Key, Ref, Value, rbe1::Rbe};
use crate::{Pending, rbe_error::RbeError};

#[derive(Default)]
pub struct RbeMatcher<K, V, R, Ctx, St>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    St: State,
{
    rbe: Rbe<K, V, R, Ctx, St>,
    open: bool,
    controlled: HashSet<K>,
}

impl<K, V, R, Ctx, St> RbeMatcher<K, V, R, Ctx, St>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    St: State,
{
    pub fn new() -> RbeMatcher<K, V, R, Ctx, St> {
        RbeMatcher::default()
    }

    pub fn with_rbe(mut self, rbe: &Rbe<K, V, R, Ctx, St>) -> Self {
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

    pub fn matches<T: IntoIterator<Item = (K, V, Ctx, St)>>(
        &self,
        iter: T,
    ) -> Result<Pending<V, R>, RbeError<K, V, R, Ctx, St>> {
        let mut pending = Pending::new();
        let mut processed: Vec<(K, V, Ctx, St)> = Vec::new();
        match self.matches_iter(iter, &mut pending, &mut processed) {
            Rbe::Fail { error } => Err(error.clone()),
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

    fn matches_iter<T: IntoIterator<Item = (K, V, Ctx, St)>>(
        &self,
        iter: T,
        pending: &mut Pending<V, R>,
        processed: &mut Vec<(K, V, Ctx, St)>,
    ) -> Rbe<K, V, R, Ctx, St> {
        let mut current = self.rbe.clone();
        for (key, value, ctx, st) in iter {
            let deriv = current.deriv(&key, &value, &ctx, &st, 1, self.open, &self.controlled, pending);
            match deriv {
                Rbe::Fail { error } => {
                    current = Rbe::Fail {
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
                    processed.push((key.clone(), value.clone(), ctx.clone(), st.clone()));
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
    use crate::{MatchCond, Max, Min, SingleCond, State};

    impl Value for i32 {}
    impl Ref for String {}
    impl State for char {}

    fn is_even(
        v: &i32,
        _ctx: &char,
        _st: &char,
    ) -> Result<Pending<i32, String>, RbeError<char, i32, String, char, char>> {
        if v % 2 == 0 {
            Ok(Pending::new())
        } else {
            Err(RbeError::MsgError {
                msg: format!("Value {v} is not even"),
            })
        }
    }

    fn ref_x(
        v: &i32,
        _ctx: &char,
        _st: &char,
    ) -> Result<Pending<i32, String>, RbeError<char, i32, String, char, char>> {
        let ps = vec![(*v, vec!["X".to_string()])].into_iter();
        Ok(Pending::from(ps))
    }

    impl Value for String {}

    fn cond_name(name: String) -> MatchCond<char, String, String, char, char> {
        MatchCond::single(SingleCond::new().with_cond(move |v: &String, _ctx: &char, _st: &char| {
            if *v == name {
                Ok(Pending::new())
            } else {
                Err(RbeError::MsgError {
                    msg: format!("Value {v} is not equal to {name}"),
                })
            }
        }))
    }

    fn cond_len(len: usize) -> MatchCond<char, String, String, char, char> {
        MatchCond::single(SingleCond::new().with_cond(move |v: &String, _ctx: &char, _st: &char| {
            if v.len() == len {
                Ok(Pending::new())
            } else {
                Err(RbeError::MsgError {
                    msg: format!("Value {v} has no length {len}"),
                })
            }
        }))
    }

    #[test]
    fn test_rbe_matcher_len_name() {
        let rbe: Rbe<char, String, String, char, char> = Rbe::and(vec![
            Rbe::symbol_cond('a', cond_len(3), Min::from(1), Max::IntMax(1)),
            Rbe::symbol_cond('b', cond_name("foo".to_string()), Min::from(0), Max::IntMax(1)),
        ]);
        let expected = Pending::new();
        let rbe_matcher = RbeMatcher::new().with_rbe(&rbe);

        assert_eq!(
            rbe_matcher.matches(vec![('a', "baz".to_string(), 'a', 'z')].into_iter()),
            Ok(expected)
        );
    }

    #[test]
    fn test_rbe_matcher_even_ref() {
        let cond_even = MatchCond::single(SingleCond::new().with_cond(is_even));
        let cond_ref_x = MatchCond::single(SingleCond::new().with_cond(ref_x));

        let rbe: Rbe<char, i32, String, char, char> = Rbe::and(vec![
            Rbe::symbol_cond('a', cond_even, Min::from(1), Max::IntMax(1)),
            Rbe::symbol_cond('b', cond_ref_x, Min::from(0), Max::IntMax(1)),
        ]);
        let expected = Pending::from(vec![(42, vec!["X".to_string()])]);
        let rbe_matcher = RbeMatcher::new().with_rbe(&rbe);

        assert_eq!(
            rbe_matcher.matches(vec![('a', 2, 'a', 'z'), ('b', 42, 'b', 'z')].into_iter()),
            Ok(expected)
        );
    }

    #[test]
    fn test_rbe_matcher_even_ref_unordered() {
        let cond_even = MatchCond::single(SingleCond::new().with_cond(is_even));
        let cond_ref_x = MatchCond::single(SingleCond::new().with_cond(ref_x));

        let rbe: Rbe<char, i32, String, char, char> = Rbe::and(vec![
            Rbe::symbol_cond('a', cond_even, Min::from(1), Max::IntMax(1)),
            Rbe::symbol_cond('b', cond_ref_x, Min::from(0), Max::IntMax(1)),
        ]);
        let expected = Pending::from(vec![(42, vec!["X".to_string()])]);
        let rbe_matcher = RbeMatcher::new().with_rbe(&rbe);

        assert_eq!(
            rbe_matcher.matches(vec![('b', 42, 'b', 'z'), ('a', 2, 'a', 'z')].into_iter()),
            Ok(expected)
        );
    }

    #[test]
    fn test_rbe_matcher_even_fails_odd() {
        let cond_even = MatchCond::single(SingleCond::new().with_cond(is_even));
        let cond_ref_x = MatchCond::single(SingleCond::new().with_cond(ref_x));

        let rbe: Rbe<char, i32, String, char, char> = Rbe::and(vec![
            Rbe::symbol_cond('a', cond_even, Min::from(1), Max::IntMax(1)),
            Rbe::symbol_cond('b', cond_ref_x, Min::from(0), Max::IntMax(1)),
        ]);
        let rbe_matcher = RbeMatcher::new().with_rbe(&rbe);
        let iter = vec![('b', 42, 'b', 'z'), ('a', 3, 'a', 'z')].into_iter();
        assert!(rbe_matcher.matches(iter).is_err());
    }
}
