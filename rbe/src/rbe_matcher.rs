use std::{fmt::{Debug, Display}, collections::HashSet};
use core::hash::Hash;
use crate::{Pending, rbe_error::RbeError};
use crate::rbe::Rbe;

pub struct RbeMatcher<K, V,R> 
where K: Hash + Eq + Display + Default,
      V: Hash + Default + Eq + Clone, 
      R: Default + PartialEq + Clone 
{
    rbe: Rbe<K, V, R>,
    open: bool,
    controlled: HashSet<K>
}


impl <K, V, R> RbeMatcher<K, V, R> 
where K: Hash + Eq + Default + Display + Debug + Clone,
      V: Hash + Default + Eq + Display + Debug + Clone, 
      R: Default + PartialEq + Display + Debug + Clone 
{
    pub fn new() -> RbeMatcher<K, V, R> {
        RbeMatcher{
            rbe: Rbe::empty(),
            open: false,
            controlled: HashSet::new()
        }
    }

    pub fn with_rbe(mut self, rbe: Rbe<K,V,R>) -> Self {
        self.rbe = rbe;
        self
    }

    pub fn with_controlled(mut self, controlled: HashSet<K>) -> Self {
        self.controlled = controlled;
        self
    }

    pub fn with_open(mut self, open: bool) -> Self {
        self.open = open ;
        self
    }

    pub fn matches<T: IntoIterator<Item=(K,V)>>(&self, iter: T) -> Result<Pending<V, R>, RbeError<K, V, R>> {
        let mut pending = Pending::new();
        match self.matches_iter(iter, &mut pending) {
            Rbe::Fail { error } => Err(error.clone()),
            d => {
                if d.nullable() {
                    Ok(pending)
                } else {
                    Err(RbeError::NonNullableMatch {
                        non_nullable_rbe: Box::new(d.clone()),
                        expr: Box::new((*self).rbe.clone())
                    })
                }
            }
        }
    }

    fn matches_iter<T: IntoIterator<Item=(K,V)>>(&self, iter:T, pending: &mut Pending<V,R>) -> Rbe<K,V,R> {
        let mut current = (*self).rbe.clone();
        let mut processed = Vec::new();
        for (key, value) in iter {
            let deriv = current.deriv(&key, &value, 1, self.open, &self.controlled, pending);
            match deriv {
              Rbe::Fail { error } => {
                current = Rbe::Fail { error: RbeError::DerivIterError {
                    error_msg: format!("{error}"),
                    processed: processed,
                    expr: Box::new((*self).rbe.clone()),
                    current: Box::new(current.clone()),
                    key: key.clone(),
                    open: self.open, 
                }};
                break;
              },
              _ => {
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
    use crate::{Max, MatchCond};
    use super::*;

    fn is_even(k: &char, v: &i32) -> Result<Pending<i32,String>, RbeError<char, i32, String>> {
        if v % 2 == 0 {
            Ok(Pending::new())
        } else {
            Err(RbeError::MsgError{ msg: format!("Value {v} for key {k} is not even") })
        }
    }

    fn ref_x(_: &char, v: &i32) -> Result<Pending<i32,String>, RbeError<char, i32, String>> {
        let ps = vec![(*v,vec!["X".to_string()])].into_iter();
        Ok(Pending::from(ps))
    }

    fn cond_name(name: String) -> MatchCond<char, String, String> {
        MatchCond::new().with_cond(move |k: &char, v: &String| {
        if *v == name {
            Ok(Pending::new())
        } else {
            Err(RbeError::MsgError{ msg: format!("Value {v} for key {k} is not equal to {name}") })
        }
      })
    }

    fn cond_len(len: usize) -> MatchCond<char, String, String> {
        MatchCond::new().with_cond(move |k: &char, v: &String| {
        if v.len() == len {
            Ok(Pending::new())
        } else {
            Err(RbeError::MsgError{ msg: format!("Value {v} for key {k} has no length {len}") })
        }
      })
    }


    #[test]
    fn test_rbe_matcher_len_name() {

        let rbe: Rbe<char, String, String> = Rbe::and(
            vec![
                Rbe::symbol_cond('a', cond_len(3), 1, Max::IntMax(1)),
                Rbe::symbol_cond('b', cond_name("foo".to_string()), 0, Max::IntMax(1))]
        );
        let expected = Pending::new();
        let rbe_matcher = RbeMatcher::new().with_rbe(rbe);

        assert_eq!(rbe_matcher.matches(vec![('a', "baz".to_string())].into_iter()), Ok(expected));
    }

    #[test]
    fn test_rbe_matcher_even_ref() {

        let cond_even = MatchCond::new().with_cond(is_even);
        let cond_ref_x = MatchCond::new().with_cond(ref_x);

        let rbe: Rbe<char, i32, String> = Rbe::and(
            vec![
                Rbe::symbol_cond('a', cond_even, 1, Max::IntMax(1)),
                Rbe::symbol_cond('b', cond_ref_x, 0, Max::IntMax(1))]
        );
        let expected = Pending::from(vec![(42, vec!["X".to_string()])].into_iter());
        let rbe_matcher = RbeMatcher::new().with_rbe(rbe);

        assert_eq!(rbe_matcher.matches(vec![('a', 2), ('b', 42)].into_iter()), Ok(expected));
    }

    #[test]
    fn test_rbe_matcher_even_ref_unordered() {

        let cond_even = MatchCond::new().with_cond(is_even);
        let cond_ref_x = MatchCond::new().with_cond(ref_x);

        let rbe: Rbe<char, i32, String> = Rbe::and(
            vec![
                Rbe::symbol_cond('a', cond_even, 1, Max::IntMax(1)),
                Rbe::symbol_cond('b', cond_ref_x, 0, Max::IntMax(1))]
        );
        let expected = Pending::from(vec![(42, vec!["X".to_string()])].into_iter());
        let rbe_matcher = RbeMatcher::new().with_rbe(rbe);

        assert_eq!(rbe_matcher.matches(vec![('b', 42), ('a', 2)].into_iter()), Ok(expected));
    }

    #[test]
    fn test_rbe_matcher_even_fails_odd() {

        let cond_even = MatchCond::new().with_cond(is_even);
        let cond_ref_x = MatchCond::new().with_cond(ref_x);

        let rbe: Rbe<char, i32, String> = Rbe::and(
            vec![
                Rbe::symbol_cond('a', cond_even, 1, Max::IntMax(1)),
                Rbe::symbol_cond('b', cond_ref_x, 0, Max::IntMax(1))]
        );
        let rbe_matcher = RbeMatcher::new().with_rbe(rbe);
        assert!(rbe_matcher.matches(vec![('b', 42), ('a', 3)].into_iter()).is_err());
    }

}