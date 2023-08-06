use crate::{Cardinality, Max, Rbe1Error, deriv_n, Min, Bag1, MatchCond, Pending};
use crate::rbe1_error::Failures;
use core::hash::Hash;
use std::collections::HashSet;
use std::fmt::{Debug, Display};
use std::fmt;
use serde_derive::{Deserialize, Serialize};
use itertools::cloned;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Rbe1<K, V, R>
where K: Hash + Eq + Display + Default,
      V: Hash + Eq + Default + Clone, 
      R: Default + PartialEq + Clone 
{
    Fail { error: Rbe1Error<K,V,R> },
    Empty,

    Symbol { key: K, cond: MatchCond<K, V, R>, card: Cardinality },
    
    And { exprs: Vec<Rbe1<K,V,R>> },
    Or { exprs: Vec<Rbe1<K,V,R>> },
    Star { expr: Box<Rbe1<K,V,R>> },
    Plus { expr: Box<Rbe1<K,V,R>> },
    Repeat { expr: Box<Rbe1<K, V, R>>, card: Cardinality },
}

type NullableResult = bool;

impl <K, V, R> Rbe1<K, V, R>
where
    K: PartialEq + Eq + Hash + Clone + fmt::Debug + fmt::Display + Default,
    V: Hash + Default + Display + Eq + Debug + Clone,
    R: Default + Display + PartialEq + Debug + Clone
{
    pub fn match_bag(&self, bag: &Bag1<K,V>, open: bool) -> Result<(), Rbe1Error<K,V,R>> {
        match &self.deriv_bag(bag, open, &self.symbols()) {
            Rbe1::Fail { error } => {
                Err(error.clone())
            },
            d => {
                if d.nullable() {
                    Ok(())
                } else {
                    Err(Rbe1Error::NonNullable {
                        non_nullable_rbe: Box::new(d.clone()),
                        bag: (*bag).clone(),
                        expr: Box::new((*self).clone())
                    })
                }
            }
        }
    }

    pub fn empty() -> Rbe1<K, V, R> {
        Rbe1::Empty
    }

    pub fn symbol(key:K, min: usize, max: Max) -> Rbe1<K, V, R> {
        Rbe1::Symbol {
            key,
            cond: MatchCond::default(),
            card: Cardinality { min: Min::from(min), max },
        }
    }

    pub fn symbol_cond(key:K, cond: MatchCond<K,V,R>, min: usize, max: Max) -> Rbe1<K, V, R> {
        Rbe1::Symbol {
            key,
            cond: cond,
            card: Cardinality { min: Min::from(min), max },
        }
    }


    pub fn or<I>(exprs: I) -> Rbe1<K, V, R> 
    where I: IntoIterator<Item = Rbe1<K, V, R>> {
        let rs = exprs.into_iter().map(|v| v).collect();
        Rbe1::Or{ exprs: rs}
    }

    pub fn and<I>(exprs: I) -> Rbe1<K, V, R>
    where I: IntoIterator<Item = Rbe1<K, V, R>> {
        let rs = exprs.into_iter().map(|v| v).collect();
        Rbe1::And{ exprs: rs}
    }

    pub fn opt(v: Rbe1<K, V, R>) -> Rbe1<K, V, R> {
        Rbe1::Or {
            exprs: vec![v, Rbe1::Empty]
        }
    }

    pub fn plus(expr: Rbe1<K, V, R>) -> Rbe1<K, V, R> {
        Rbe1::Plus { expr: Box::new(expr) }
    }

    pub fn star(expr: Rbe1<K, V, R>) -> Rbe1<K, V, R> {
        Rbe1::Star { expr: Box::new(expr) }
    }

    pub fn repeat(expr: Rbe1<K, V, R>, min: usize, max: Max) -> Rbe1<K, V, R> {
        Rbe1::Repeat {
            expr: Box::new(expr),
            card: Cardinality::from(Min::from(min), max),
        }
    }

    pub fn is_fail(&self) -> bool {
        match &self {
            Rbe1::Fail {..} => true,
            _ => false
        }
    }

    pub fn symbols(&self) -> HashSet<K> {
        let mut set = HashSet::new();
        self.symbols_aux(&mut set);
        set
    }

    fn symbols_aux(&self, set: &mut HashSet<K>) {
        match &self {
            Rbe1::Fail { .. } => (),
            Rbe1::Empty => (),
            Rbe1::Symbol { key, .. } => {
                set.insert(key.clone());
            }
            Rbe1::And { exprs } => {
                exprs.iter().for_each(|v| v.symbols_aux(set));
            }
            Rbe1::Or { exprs} => {
                exprs.iter().for_each(|v| v.symbols_aux(set));
            }
            Rbe1::Plus { expr } => {
                expr.symbols_aux(set);
            }
            Rbe1::Star { expr } => {
                expr.symbols_aux(set);
            }
            Rbe1::Repeat { expr, card: _ } => {
                expr.symbols_aux(set);
            }
        }
    }

    pub fn deriv_bag(&self, bag: &Bag1<K, V>, open: bool, controlled: &HashSet<K>) -> Rbe1<K, V, R> {
        let mut current = (*self).clone();
        let mut processed = Bag1::new();
        let mut pending = Pending::new();
        for (key, value) in bag.iter() {
            let deriv = current.deriv(key, value, 1, open, controlled, &mut pending);
            match deriv {
              Rbe1::Fail { error } => {
                current = Rbe1::Fail { error: Rbe1Error::DerivBagError {
                    error_msg: format!("{error}"),
                    processed: processed,
                    bag: (*bag).clone(),
                    expr: Box::new((*self).clone()),
                    current: Box::new(current.clone()),
                    value: (*key).clone(),
                    open: open, 
                }};
                break;
              },
              _ => {
                processed.insert(((*key).clone(), (*value).clone()));
                current = deriv;
              }
            }
        }
        current
    }

    pub fn nullable(&self) -> NullableResult {
        match &self {
            Rbe1::Fail { .. } => false,
            Rbe1::Empty => true,
            Rbe1::Symbol { card, .. } if card.nullable() => true,
            Rbe1::Symbol { .. } => false,
            Rbe1::And { exprs } => {
               exprs.iter()
               .map(|v| v.nullable())
               .all(|v| v == true)
            },
            Rbe1::Or { exprs } => {
                exprs.iter()
                .map(|v| v.nullable())
                .any(|v| v == true)},
            Rbe1::Star { .. } => true,
            Rbe1::Plus { expr } => expr.nullable(),
            Rbe1::Repeat { expr: _, card } if card.min.is_0() => true,
            Rbe1::Repeat { expr, card: __ } => expr.nullable(),
        }
    }


    pub fn deriv(&self, 
        symbol: &K, 
        value: &V, 
        n: usize, 
        open: bool, 
        controlled: &HashSet<K>, 
        pending: &mut Pending<V,R>) -> Rbe1<K, V, R>
    where
        K: Eq + Hash + Clone,
    {
        match *self {
            ref fail @ Rbe1::Fail { .. } => fail.clone(),
            Rbe1::Empty => {
                if open && !(controlled.contains(&symbol)) {
                    Rbe1::Empty
                } else {
                    Rbe1::Fail {
                        error: Rbe1Error::UnexpectedEmpty {
                            x: (*symbol).clone(),
                            open: open,
                        },
                    }
                }
            }
            Rbe1::Symbol { ref key, ref cond, ref card } => {
                if *key == *symbol {
                    match cond.matches(symbol, value) {
                        Err(err) => {
                            Rbe1::Fail {
                                error: err
                            }
                        },
                        Ok(new_pending) => {
                            if card.max == Max::IntMax(0) {
                                Rbe1::Fail {
                                    error: Rbe1Error::MaxCardinalityZeroFoundValue { x: (*symbol).clone() },
                                }
                            } else {
                                if let Some(card) = card.minus(n) {
                                    pending.merge(new_pending);
                                    Self::mk_range_symbol(&symbol, cond, &card)
                                } else {
                                    Rbe1::Fail {
                                        error: Rbe1Error::CardinalityFail {
                                            symbol: key.clone(),
                                            expected_cardinality: card.clone(),
                                            current_number: n,
                                        },
                                    }
                                }
                            }
                        }
                    }
                } else {
                    // Symbol is different from symbols defined in Rbe1
                    // if the Rbe1 is open, we allow extra symbols
                    if open && !(controlled.contains(&symbol)) {
                        self.clone()
                    } else {
                        Rbe1::Fail {
                            error: Rbe1Error::UnexpectedSymbol {
                                x: (*symbol).clone(),
                                expected: key.clone(),
                                open: open,
                            },
                        }
                    }
                }
            },
            Rbe1::And { ref exprs } => {
                Self::deriv_and(exprs, &symbol, &value, n, open, controlled, pending)
            }
            Rbe1::Or { ref exprs } => {
                Self::mk_or_values(exprs.into_iter().map(|rbe1| { 
                    rbe1.deriv(symbol, value, n, open, controlled, pending)
                }))
            },
            Rbe1::Plus { ref expr } => {
                let d = expr.deriv(symbol, value, n, open, controlled, pending);
                Self::mk_and(&d, &Rbe1::Star { expr: expr.clone() })
            }
            Rbe1::Repeat { ref expr, ref card } 
            if card.is_0_0() => {
                let d = expr.deriv(symbol, value, n, open, controlled, pending);
                if d.nullable() {
                  Rbe1::Fail { error: Rbe1Error::CardinalityZeroZeroDeriv { 
                    symbol: symbol.clone()
                   }}
                } else { 
                  Rbe1::Empty
                }
            }
            Rbe1::Repeat { ref expr, ref card } => {
                let d = expr.deriv(symbol, value, n, open, controlled, pending);
                if let Some(card) = card.minus(n) {
                    let rest = Self::mk_range(&expr, &card);
                    Self::mk_and(&d, &rest)
                } else {
                    Rbe1::Fail {
                        error: Rbe1Error::CardinalityFailRepeat {
                            expected_cardinality: card.clone(),
                            current_number: n,
                        },
                    }
                }
            }
            Rbe1::Star { ref expr } => {
                let d = expr.deriv(symbol, value, n, open, controlled, pending);
                Self::mk_and(&d, &expr)
            }
        }
    }

    fn deriv_and(
        values: &Vec<Rbe1<K, V, R>>, 
        symbol: &K,
        value: &V, 
        n: usize, 
        open: bool, 
        controlled: &HashSet<K>,
        pending: &mut Pending<V,R>
    ) -> Rbe1<K, V, R> {

        let mut or_values: Vec<Rbe1<K, V, R>> = Vec::new();
        let mut failures = Failures::new();
        
        for vs in deriv_n(
            cloned((*values).iter()).collect(), 
            |expr: &Rbe1<K, V, R>| {
                let d = expr.deriv(symbol, value, n, open, controlled, pending);
                match d { 
                    Rbe1::Fail { error } => {
                        failures.push(expr.clone(), error);
                        None
                    },
                    _ => {
                        Some(d)
                    }
                }
            }
        ) {
            or_values.push(Rbe1::And { exprs: vs });
        }
        match or_values.len() {
            0 => Rbe1::Fail { 
                error: Rbe1Error::OrValuesFail { 
                  e: Box::new(Rbe1::And { exprs: cloned(values.iter()).collect() }),
                  failures: failures
                } 
            },
            1 => {
                or_values[0].clone()
            }
            _ => Rbe1::Or {
                exprs: or_values
            }
        }
    }

    fn mk_range(e: &Rbe1<K, V, R>, card: &Cardinality) -> Rbe1<K, V, R>
    where
        K: Clone,
    {
        if Self::bigger((*card).min, &(*card).max) {
            Rbe1::Fail {
                error: Rbe1Error::RangeLowerBoundBiggerMaxExpr { 
                    expr: Box::new((*e).clone()),
                    card: card.clone() 
                },
            }
        } else {
            match (e, card) {
                (_, c) if c.is_0_0() => Rbe1::Empty,
                (e, c) if c.is_1_1() => e.clone(),
                (fail@Rbe1::Fail { ..}, _) => fail.clone(),
                (Rbe1::Empty, _) => Rbe1::Empty,
                (e, c) => Rbe1::Repeat { 
                    expr: Box::new(e.clone()), 
                    card: c.clone()
                }
            }
        }
    }

    fn mk_range_symbol(x: &K, cond: &MatchCond<K, V, R>, card: &Cardinality) -> Rbe1<K, V, R>
    where
        K: Clone,
    {
        if Self::bigger((*card).min, &(*card).max) {
            Rbe1::Fail {
                error: Rbe1Error::RangeLowerBoundBiggerMax { 
                    symbol: (*x).clone(),
                    card: card.clone() 
                },
            }
        } else {
            Rbe1::Symbol {
                key: (*x).clone(),
                cond: (*cond).clone(), 
                card: card.clone(),
            }
        }
    }

    fn mk_and(v1: &Rbe1<K, V, R>, v2: &Rbe1<K, V, R>) -> Rbe1<K, V, R>
    where
        K: Clone,
    {
        match (v1, v2) {
            (Rbe1::Empty, _) => (*v2).clone(),
            (_, Rbe1::Empty) => (*v1).clone(),
            (f @ Rbe1::Fail { .. }, _) => f.clone(),
            (_, f @ Rbe1::Fail { .. }) => f.clone(),
            (_, _) => Rbe1::And {
                exprs: vec![(*v1).clone(),(*v2).clone()] 
             },
        }
    }

    fn mk_or_values<I>(values: I) -> Rbe1<K, V, R> 
    where I: IntoIterator<Item = Rbe1<K, V, R>> {
        let init = Rbe1::Fail { error: Rbe1Error::MkOrValuesFail };
        let result = values
           .into_iter()
           .fold(init, |result, value| {
            Self::mk_or(&result, &value)
        });
        result
    }

    fn mk_or(v1: &Rbe1<K, V, R>, v2: &Rbe1<K, V, R>) -> Rbe1<K, V, R> {
        match (v1, v2) {
            (Rbe1::Fail { .. }, _) => (*v2).clone(),
            (_, Rbe1::Fail { .. }) => (*v1).clone(),
            (e1, e2) => {
                if e1 == e2 {
                    (*v1).clone()
                } else {
                    Rbe1::Or {
                        exprs: vec![
                            (*v1).clone(),
                            (*v2).clone()]
                    }
                }
            }
        }
    }

    fn bigger(min: Min, max: &Max) -> bool {
        match max {
            Max::Unbounded => false,
            Max::IntMax(max) => min.value > *max,
        }
    }
}

impl <K, V, R> Default for Rbe1<K, V, R> 
where K: Hash + Eq + fmt::Display + Default,
      V: Hash + Default + Eq + Clone,
      R: Default + PartialEq + Clone
{
    fn default() -> Self { 
        Rbe1::Empty 
    }
}

impl <K, V, R> Debug for Rbe1<K, V, R> 
where K: Debug + Hash + Eq + fmt::Display + Default,
      V: Hash + Debug + Eq + Default + Clone,
      R: Debug + PartialEq + Default + Clone
{
    fn fmt(&self, dest: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Rbe1::Fail { error } => write!(dest,"Fail {{{error:?}}}"),
            Rbe1::Empty => write!(dest,"Empty"),
            Rbe1::Symbol { key, cond:_ , card } => write!(dest,"{key:?}{card:?}"),
            Rbe1::And { exprs} => {
                exprs.iter().fold(Ok(()), |result,value| {
                    result.and_then(|_| write!(dest, "{value:?};"))
                })
            },
            Rbe1::Or { exprs } => {
                exprs.iter().fold(Ok(()), |result,value| {
                    result.and_then(|_| write!(dest, "{value:?}|"))
                })
            }
            Rbe1::Star { expr } => write!(dest,"{expr:?}*"),
            Rbe1::Plus { expr } => write!(dest,"{expr:?}+"),
            Rbe1::Repeat { expr, card } => write!(dest,"({expr:?}){card:?}"),
        }
    }
}

impl <K, V, R> Display for Rbe1<K, V, R> 
where K: Display + Hash + Eq + Default,
      V: Hash + Eq + Default + Display + Clone,
      R: Display + PartialEq + Default + Clone
{
    fn fmt(&self, dest: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Rbe1::Fail { error } => write!(dest,"Fail {{{error}}}"),
            Rbe1::Empty => write!(dest,"Empty"),
            Rbe1::Symbol { key, cond:_, card } => write!(dest,"{key}{card}"),
            Rbe1::And { exprs } => {
                exprs.iter().fold(Ok(()), |result,value| {
                    result.and_then(|_| write!(dest, "{value};"))
                })
            },
            Rbe1::Or { exprs } => {
                exprs.iter().fold(Ok(()), |result,value| {
                    result.and_then(|_| write!(dest, "{value}|"))
                })
            },
            Rbe1::Star { expr } => write!(dest,"{expr}*"),
            Rbe1::Plus { expr } => write!(dest,"{expr}+"),
            Rbe1::Repeat { expr, card } => write!(dest,"({expr}){card}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;
    use indoc::indoc;

    #[test_log::test]
    fn deriv_a_1_1_and_b_opt_with_a() {
        // a?|b? #= b/2
        let rbe1: Rbe1<char, i32, i32> = Rbe1::and(
            vec![
                Rbe1::symbol('a', 1, Max::IntMax(1)),
                Rbe1::symbol('b', 0, Max::IntMax(1))]
        );
        let mut pending = Pending::new();
        let expected = Rbe1::and(
            vec![
                Rbe1::symbol('a', 0, Max::IntMax(0)),
                Rbe1::symbol('b', 0, Max::IntMax(1))]
            );
        assert_eq!(rbe1.deriv(&'a',&23, 1, false, &HashSet::from(['a','b']), &mut pending), expected);
    }

    #[test]
    fn deriv_symbol() {
        let rbe1: Rbe1<char, i32, i32> = Rbe1::symbol('x', 1, Max::IntMax(1));
        let mut pending = Pending::new();
        let d = rbe1.deriv(&'x', &2, 1, true, &HashSet::new(), &mut pending);
        assert_eq!(d, Rbe1::symbol('x', 0, Max::IntMax(0)));
    }

    #[test]
    fn symbols() {
        let rbe1: Rbe1<char, i32, i32> = Rbe1::and(
            vec![
                Rbe1::symbol('x', 1, Max::IntMax(1)),
                Rbe1::symbol('y', 1, Max::IntMax(1))]
        );
        let expected = HashSet::from(['x', 'y']);
        assert_eq!(rbe1.symbols(), expected);
    }

    #[test]
    fn symbols2() {
        let rbe1: Rbe1<char, i32, i32> = Rbe1::and(
            vec![Rbe1::or(
                vec![
                    Rbe1::symbol('x', 1, Max::IntMax(1)),
                    Rbe1::symbol('y', 2, Max::Unbounded)
                    ]),
            Rbe1::symbol('y', 1, Max::IntMax(1))
            ]);
        let expected = HashSet::from(['x', 'y']);
        assert_eq!(rbe1.symbols(), expected);
    }

    #[test]
    fn match_bag_y1_4_y_2() {
        // y{1,4} #= y/2
        let rbe1: Rbe1<char,i32,i32> = Rbe1::symbol('y', 1, Max::IntMax(4));
        assert_eq!(rbe1.match_bag(&Bag1::from(vec![('y',1), ('y',2)]), false), Ok(()));
    }

    #[test]
    fn match_bag_a_opt_or_b_opt_with_a() {
        // a?|b? #= a
        let rbe1: Rbe1<char, i32, i32> = Rbe1::or(
            vec![
                Rbe1::symbol('a', 0, Max::IntMax(1)),
                Rbe1::symbol('b', 0, Max::IntMax(1))]
        );
        assert_eq!(rbe1.match_bag(&Bag1::from(vec![('a',1)]), false), Ok(()));
    }

    #[test]
    fn match_bag_a_opt_or_b_opt_with_b() {
        // a?|b? #= a
        let rbe1: Rbe1<char, i32, i32> = Rbe1::or(
            vec![
                Rbe1::symbol('a', 0, Max::IntMax(1)),
                Rbe1::symbol('b', 0, Max::IntMax(1))]
        );
        assert_eq!(rbe1.match_bag(&Bag1::from(vec![('b',1)]), false), Ok(()));
    }

    #[test]
    fn match_bag_a_opt_and_b_opt_with_ba() {
        // a?|b? #= a
        let rbe1: Rbe1<char, i32, i32> = Rbe1::and(
            vec![
                Rbe1::symbol('a', 0, Max::IntMax(1)),
                Rbe1::symbol('b', 0, Max::IntMax(1))]
        );
        assert_eq!(rbe1.match_bag(&Bag1::from(vec![('b',1), ('a', 2)]), false), Ok(()));
    }

    #[test]
    fn match_bag_a_and_b_opt_with_ab() {
        // a?|b? #= b/2
        let rbe1: Rbe1<char, i32, i32> = Rbe1::and(
            vec![Rbe1::symbol('a', 1, Max::IntMax(1)),
            Rbe1::symbol('b', 0, Max::IntMax(1))]
        );
        assert_eq!(rbe1.match_bag(&Bag1::from(vec![('a',1), ('b',1)]), false), Ok(()));
    }

    #[test]
    fn no_match_bag_a_and_b_opt_with_b_2() {
        // a?|b? #= b/2
        let rbe1: Rbe1<char, i32, i32> = Rbe1::and(
            vec![Rbe1::symbol('a', 1, Max::IntMax(1)),
            Rbe1::symbol('b', 0, Max::IntMax(1))]
        );
        assert!(rbe1.match_bag(&Bag1::from(vec![('b', 2), ('b',3)]), false).is_err());
    }

    #[test]
    fn no_match_bag_a_and_b_opt_with_c() {
        // a?|b? #= a
        let rbe1: Rbe1<char, String, String> = Rbe1::and(
            vec![Rbe1::symbol('a', 1, Max::IntMax(1)),
            Rbe1::symbol('b', 1, Max::IntMax(1))]
        );
        assert!(rbe1.match_bag(&Bag1::from(vec![('c', "One".to_string())]), false).is_err());
    }

    #[test]
    fn test_serialize_rbe1() {
        
        let rbe1: Rbe1<String, String, String> = Rbe1::symbol("foo".to_string(), 1, Max::IntMax(2));
        let expected = indoc! {
            r#"!Symbol
                 key: foo
                 cond:
                   name: null
                 card:
                   min: 1
                   max: 2
              "# };
        let rbe1: String = serde_yaml::to_string(&rbe1).unwrap();
        assert_eq!(rbe1, expected);
    } 

    #[test]
    fn test_deserialize_rbe1() {
        let str = r#"{ 
            "Symbol": { 
                "key": "foo",
                "cond": { "name": null },
                "card": {"min": 1, "max": 2 } 
            }
        }"#;
        let expected = Rbe1::symbol("foo".to_string(), 1, Max::IntMax(2));
        let rbe1: Rbe1<String, String, String> = serde_json::from_str(str).unwrap();
        assert_eq!(rbe1, expected);
    }

}

