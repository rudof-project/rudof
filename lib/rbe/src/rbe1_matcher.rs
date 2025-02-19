use std::collections::HashSet;
use tracing::debug;

use crate::{rbe1::Rbe, Key, Ref, Value};
use crate::{rbe_error::RbeError, Pending};

#[derive(Default)]
pub struct RbeMatcher<K, V, R>
where
    K: Key,
    V: Value,
    R: Ref,
{
    rbe: Rbe<K, V, R>,
    open: bool,
    controlled: HashSet<K>,
}

impl<K, V, R> RbeMatcher<K, V, R>
where
    K: Key,
    V: Value,
    R: Ref,
{
    pub fn new() -> RbeMatcher<K, V, R> {
        RbeMatcher::default()
    }

    pub fn with_rbe(mut self, rbe: &Rbe<K, V, R>) -> Self {
        self.rbe = (*rbe).clone();
        self.controlled = rbe.symbols();
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

    pub fn matches<T: IntoIterator<Item = (K, V)>>(
        &self,
        iter: T,
    ) -> Result<Pending<V, R>, RbeError<K, V, R>> {
        let mut pending = Pending::new();
        let mut processed: Vec<(K, V)> = Vec::new();
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
            }
        }
    }

    fn matches_iter<T: IntoIterator<Item = (K, V)>>(
        &self,
        iter: T,
        pending: &mut Pending<V, R>,
        processed: &mut Vec<(K, V)>,
    ) -> Rbe<K, V, R> {
        let mut current = self.rbe.clone();
        for (key, value) in iter {
            let deriv = current.deriv(&key, &value, 1, self.open, &self.controlled, pending);
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
                }
                _ => {
                    debug!("Processing: {key}/{value}\ncurrent:{current}\nderiv:{deriv}");
                    processed.push((key.clone(), value.clone()));
                    current = deriv;
                }
            }
        }
        current
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{MatchCond, Max, Min, SingleCond};

    impl Value for i32 {}
    impl Ref for String {}

    fn is_even(v: &i32) -> Result<Pending<i32, String>, RbeError<char, i32, String>> {
        if v % 2 == 0 {
            Ok(Pending::new())
        } else {
            Err(RbeError::MsgError {
                msg: format!("Value {v} is not even"),
            })
        }
    }

    fn ref_x(v: &i32) -> Result<Pending<i32, String>, RbeError<char, i32, String>> {
        let ps = vec![(*v, vec!["X".to_string()])].into_iter();
        Ok(Pending::from(ps))
    }

    impl Value for String {}

    fn cond_name(name: String) -> MatchCond<char, String, String> {
        MatchCond::single(SingleCond::new().with_cond(move |v: &String| {
            if *v == name {
                Ok(Pending::new())
            } else {
                Err(RbeError::MsgError {
                    msg: format!("Value {v} is not equal to {name}"),
                })
            }
        }))
    }

    fn cond_len(len: usize) -> MatchCond<char, String, String> {
        MatchCond::single(SingleCond::new().with_cond(move |v: &String| {
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
        let rbe: Rbe<char, String, String> = Rbe::and(vec![
            Rbe::symbol_cond('a', cond_len(3), Min::from(1), Max::IntMax(1)),
            Rbe::symbol_cond(
                'b',
                cond_name("foo".to_string()),
                Min::from(0),
                Max::IntMax(1),
            ),
        ]);
        let expected = Pending::new();
        let rbe_matcher = RbeMatcher::new().with_rbe(&rbe);

        assert_eq!(
            rbe_matcher.matches(vec![('a', "baz".to_string())].into_iter()),
            Ok(expected)
        );
    }

    #[test]
    fn test_rbe_matcher_even_ref() {
        let cond_even = MatchCond::single(SingleCond::new().with_cond(is_even));
        let cond_ref_x = MatchCond::single(SingleCond::new().with_cond(ref_x));

        let rbe: Rbe<char, i32, String> = Rbe::and(vec![
            Rbe::symbol_cond('a', cond_even, Min::from(1), Max::IntMax(1)),
            Rbe::symbol_cond('b', cond_ref_x, Min::from(0), Max::IntMax(1)),
        ]);
        let expected = Pending::from(vec![(42, vec!["X".to_string()])]);
        let rbe_matcher = RbeMatcher::new().with_rbe(&rbe);

        assert_eq!(
            rbe_matcher.matches(vec![('a', 2), ('b', 42)].into_iter()),
            Ok(expected)
        );
    }

    #[test]
    fn test_rbe_matcher_even_ref_unordered() {
        let cond_even = MatchCond::single(SingleCond::new().with_cond(is_even));
        let cond_ref_x = MatchCond::single(SingleCond::new().with_cond(ref_x));

        let rbe: Rbe<char, i32, String> = Rbe::and(vec![
            Rbe::symbol_cond('a', cond_even, Min::from(1), Max::IntMax(1)),
            Rbe::symbol_cond('b', cond_ref_x, Min::from(0), Max::IntMax(1)),
        ]);
        let expected = Pending::from(vec![(42, vec!["X".to_string()])]);
        let rbe_matcher = RbeMatcher::new().with_rbe(&rbe);

        assert_eq!(
            rbe_matcher.matches(vec![('b', 42), ('a', 2)].into_iter()),
            Ok(expected)
        );
    }

    #[test]
    fn test_rbe_matcher_even_fails_odd() {
        let cond_even = MatchCond::single(SingleCond::new().with_cond(is_even));
        let cond_ref_x = MatchCond::single(SingleCond::new().with_cond(ref_x));

        let rbe: Rbe<char, i32, String> = Rbe::and(vec![
            Rbe::symbol_cond('a', cond_even, Min::from(1), Max::IntMax(1)),
            Rbe::symbol_cond('b', cond_ref_x, Min::from(0), Max::IntMax(1)),
        ]);
        let rbe_matcher = RbeMatcher::new().with_rbe(&rbe);
        let iter = vec![('b', 42), ('a', 3)].into_iter();
        assert!(rbe_matcher.matches(iter).is_err());
    }
}
