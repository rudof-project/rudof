use crate::failures::Failures;
use crate::{deriv_n, rbe_error::RbeError, Cardinality, MatchCond, Max, Min, Pending};
use crate::{Key, Ref, Value};
use core::hash::Hash;
use itertools::cloned;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt;
use std::fmt::{Debug, Display};

#[derive(Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Rbe<K, V, R>
where
    K: Key,
    V: Value,
    R: Ref,
{
    Fail {
        error: RbeError<K, V, R>,
    },
    #[default]
    Empty,

    Symbol {
        key: K,
        cond: MatchCond<K, V, R>,
        card: Cardinality,
    },

    And {
        exprs: Vec<Rbe<K, V, R>>,
    },
    Or {
        exprs: Vec<Rbe<K, V, R>>,
    },
    Star {
        expr: Box<Rbe<K, V, R>>,
    },
    Plus {
        expr: Box<Rbe<K, V, R>>,
    },
    Repeat {
        expr: Box<Rbe<K, V, R>>,
        card: Cardinality,
    },
}

type NullableResult = bool;

impl<K, V, R> Rbe<K, V, R>
where
    K: Key,
    V: Value,
    R: Ref,
{
    pub fn empty() -> Rbe<K, V, R> {
        Rbe::Empty
    }

    pub fn symbol(key: K, min: usize, max: Max) -> Rbe<K, V, R> {
        Rbe::Symbol {
            key,
            cond: MatchCond::default(),
            card: Cardinality {
                min: Min::from(min),
                max,
            },
        }
    }

    pub fn symbol_cond(key: K, cond: MatchCond<K, V, R>, min: Min, max: Max) -> Rbe<K, V, R> {
        Rbe::Symbol {
            key,
            cond,
            card: Cardinality { min, max },
        }
    }

    pub fn or<I>(exprs: I) -> Rbe<K, V, R>
    where
        I: IntoIterator<Item = Rbe<K, V, R>>,
    {
        let rs = exprs.into_iter().collect();
        Rbe::Or { exprs: rs }
    }

    pub fn and<I>(exprs: I) -> Rbe<K, V, R>
    where
        I: IntoIterator<Item = Rbe<K, V, R>>,
    {
        let rs = exprs.into_iter().collect();
        Rbe::And { exprs: rs }
    }

    pub fn opt(v: Rbe<K, V, R>) -> Rbe<K, V, R> {
        Rbe::Or {
            exprs: vec![v, Rbe::Empty],
        }
    }

    pub fn plus(expr: Rbe<K, V, R>) -> Rbe<K, V, R> {
        Rbe::Plus {
            expr: Box::new(expr),
        }
    }

    pub fn star(expr: Rbe<K, V, R>) -> Rbe<K, V, R> {
        Rbe::Star {
            expr: Box::new(expr),
        }
    }

    pub fn repeat(expr: Rbe<K, V, R>, min: usize, max: Max) -> Rbe<K, V, R> {
        Rbe::Repeat {
            expr: Box::new(expr),
            card: Cardinality::from(Min::from(min), max),
        }
    }

    pub fn is_fail(&self) -> bool {
        matches!(self, Rbe::Fail { .. })
    }

    pub fn symbols(&self) -> HashSet<K> {
        let mut set = HashSet::new();
        self.symbols_aux(&mut set);
        set
    }

    fn symbols_aux(&self, set: &mut HashSet<K>) {
        match &self {
            Rbe::Fail { .. } => (),
            Rbe::Empty => (),
            Rbe::Symbol { key, .. } => {
                set.insert(key.clone());
            }
            Rbe::And { exprs } => {
                exprs.iter().for_each(|v| v.symbols_aux(set));
            }
            Rbe::Or { exprs } => {
                exprs.iter().for_each(|v| v.symbols_aux(set));
            }
            Rbe::Plus { expr } => {
                expr.symbols_aux(set);
            }
            Rbe::Star { expr } => {
                expr.symbols_aux(set);
            }
            Rbe::Repeat { expr, card: _ } => {
                expr.symbols_aux(set);
            }
        }
    }

    pub fn nullable(&self) -> NullableResult {
        match &self {
            Rbe::Fail { .. } => false,
            Rbe::Empty => true,
            Rbe::Symbol { card, .. } if card.nullable() => true,
            Rbe::Symbol { .. } => false,
            Rbe::And { exprs } => exprs.iter().map(|v| v.nullable()).all(|v| v),
            Rbe::Or { exprs } => exprs.iter().map(|v| v.nullable()).any(|v| v),
            Rbe::Star { .. } => true,
            Rbe::Plus { expr } => expr.nullable(),
            Rbe::Repeat { expr: _, card } if card.min.is_0() => true,
            Rbe::Repeat { expr, card: _ } => expr.nullable(),
        }
    }

    /// Calculates the derivative of a `rbe` for a `symbol` with `value`
    /// open indicates if we allow extra symbols
    /// `controlled` contains the list of symbols controlled by the `rbe` that should not be assumed as extra symbols
    /// `pending`
    pub fn deriv(
        &self,
        symbol: &K,
        value: &V,
        n: usize,
        open: bool,
        controlled: &HashSet<K>,
        pending: &mut Pending<V, R>,
    ) -> Rbe<K, V, R>
    where
        K: Eq + Hash + Clone,
    {
        match *self {
            ref fail @ Rbe::Fail { .. } => fail.clone(),
            Rbe::Empty => {
                if open && !(controlled.contains(symbol)) {
                    Rbe::Empty
                } else {
                    Rbe::Fail {
                        error: RbeError::UnexpectedEmpty {
                            x: (*symbol).clone(),
                            open,
                        },
                    }
                }
            }
            Rbe::Symbol {
                ref key,
                ref cond,
                ref card,
            } => {
                if *key == *symbol {
                    match cond.matches(value) {
                        Err(err) => Rbe::Fail { error: err },
                        Ok(new_pending) => {
                            if card.max == Max::IntMax(0) {
                                Rbe::Fail {
                                    error: RbeError::MaxCardinalityZeroFoundValue {
                                        x: (*symbol).clone(),
                                    },
                                }
                            } else {
                                let new_card = card.minus(n);
                                (*pending).merge(new_pending);
                                Self::mk_range_symbol(symbol, cond, &new_card)
                            }
                        }
                    }
                } else {
                    // Symbol is different from symbols defined in Rbe
                    // if the Rbe is open, we allow extra symbols
                    // unless the controlled symbols
                    if open && !(controlled.contains(symbol)) {
                        self.clone()
                    } else {
                        Rbe::Fail {
                            error: RbeError::UnexpectedSymbol {
                                x: (*symbol).clone(),
                                expected: key.clone(),
                                open,
                            },
                        }
                    }
                }
            }
            Rbe::And { ref exprs } => {
                Self::deriv_and(exprs, symbol, value, n, open, controlled, pending)
            }
            Rbe::Or { ref exprs } => Self::mk_or_values(
                exprs
                    .iter()
                    .map(|rbe| rbe.deriv(symbol, value, n, open, controlled, pending)),
            ),
            Rbe::Plus { ref expr } => {
                let d = expr.deriv(symbol, value, n, open, controlled, pending);
                Self::mk_and(&d, &Rbe::Star { expr: expr.clone() })
            }
            Rbe::Repeat { ref expr, ref card } if card.is_0_0() => {
                let d = expr.deriv(symbol, value, n, open, controlled, pending);
                if d.nullable() {
                    Rbe::Fail {
                        error: RbeError::CardinalityZeroZeroDeriv {
                            symbol: symbol.clone(),
                        },
                    }
                } else {
                    Rbe::Empty
                }
            }
            Rbe::Repeat { ref expr, ref card } => {
                let d = expr.deriv(symbol, value, n, open, controlled, pending);
                let card = card.minus(n);
                let rest = Self::mk_range(expr, &card);
                Self::mk_and(&d, &rest)
            }
            Rbe::Star { ref expr } => {
                let d = expr.deriv(symbol, value, n, open, controlled, pending);
                Self::mk_and(&d, expr)
            }
        }
    }

    fn deriv_and(
        values: &Vec<Rbe<K, V, R>>,
        symbol: &K,
        value: &V,
        n: usize,
        open: bool,
        controlled: &HashSet<K>,
        pending: &mut Pending<V, R>,
    ) -> Rbe<K, V, R> {
        let mut or_values: Vec<Rbe<K, V, R>> = Vec::new();
        let mut failures = Failures::new();

        for vs in deriv_n(cloned((*values).iter()).collect(), |expr: &Rbe<K, V, R>| {
            let d = expr.deriv(symbol, value, n, open, controlled, pending);
            match d {
                Rbe::Fail { error } => {
                    failures.push(expr.clone(), error);
                    None
                }
                _ => Some(d),
            }
        }) {
            or_values.push(Rbe::And { exprs: vs });
        }
        match or_values.len() {
            0 => Rbe::Fail {
                error: RbeError::OrValuesFail {
                    e: Box::new(Rbe::And {
                        exprs: cloned(values.iter()).collect(),
                    }),
                    failures,
                },
            },
            1 => or_values[0].clone(),
            _ => Rbe::Or { exprs: or_values },
        }
    }

    fn mk_range(e: &Rbe<K, V, R>, card: &Cardinality) -> Rbe<K, V, R>
    where
        K: Clone,
    {
        if Self::bigger(card.min, &card.max) {
            Rbe::Fail {
                error: RbeError::RangeLowerBoundBiggerMaxExpr {
                    expr: Box::new((*e).clone()),
                    card: card.clone(),
                },
            }
        } else {
            match (e, card) {
                (_, c) if c.is_0_0() => Rbe::Empty,
                (e, c) if c.is_1_1() => e.clone(),
                (fail @ Rbe::Fail { .. }, _) => fail.clone(),
                (Rbe::Empty, _) => Rbe::Empty,
                (e, c) => Rbe::Repeat {
                    expr: Box::new(e.clone()),
                    card: c.clone(),
                },
            }
        }
    }

    fn mk_range_symbol(x: &K, cond: &MatchCond<K, V, R>, card: &Cardinality) -> Rbe<K, V, R>
    where
        K: Clone,
    {
        if Self::bigger(card.min, &card.max) {
            Rbe::Fail {
                error: RbeError::RangeLowerBoundBiggerMax {
                    symbol: (*x).clone(),
                    card: card.clone(),
                },
            }
        } else {
            Rbe::Symbol {
                key: (*x).clone(),
                cond: (*cond).clone(),
                card: card.clone(),
            }
        }
    }

    fn mk_and(v1: &Rbe<K, V, R>, v2: &Rbe<K, V, R>) -> Rbe<K, V, R>
    where
        K: Clone,
    {
        match (v1, v2) {
            (Rbe::Empty, _) => (*v2).clone(),
            (_, Rbe::Empty) => (*v1).clone(),
            (f @ Rbe::Fail { .. }, _) => f.clone(),
            (_, f @ Rbe::Fail { .. }) => f.clone(),
            (_, _) => Rbe::And {
                exprs: vec![(*v1).clone(), (*v2).clone()],
            },
        }
    }

    fn mk_or_values<I>(values: I) -> Rbe<K, V, R>
    where
        I: IntoIterator<Item = Rbe<K, V, R>>,
    {
        let init = Rbe::Fail {
            error: RbeError::MkOrValuesFail,
        };

        values
            .into_iter()
            .fold(init, |result, value| Self::mk_or(&result, &value))
    }

    fn mk_or(v1: &Rbe<K, V, R>, v2: &Rbe<K, V, R>) -> Rbe<K, V, R> {
        match (v1, v2) {
            (Rbe::Fail { .. }, _) => (*v2).clone(),
            (_, Rbe::Fail { .. }) => (*v1).clone(),
            (e1, e2) => {
                if e1 == e2 {
                    (*v1).clone()
                } else {
                    Rbe::Or {
                        exprs: vec![(*v1).clone(), (*v2).clone()],
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

impl<K, V, R> Debug for Rbe<K, V, R>
where
    K: Key,
    V: Value,
    R: Ref,
{
    fn fmt(&self, dest: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Rbe::Fail { error } => write!(dest, "Fail {{{error:?}}}"),
            Rbe::Empty => write!(dest, "Empty"),
            Rbe::Symbol { key, cond, card } => write!(dest, "{key:?}|{cond:?}{card:?}"),
            Rbe::And { exprs } => exprs
                .iter()
                .try_for_each(|value| write!(dest, "{value:?};")),
            Rbe::Or { exprs } => exprs
                .iter()
                .try_for_each(|value| write!(dest, "{value:?}|")),
            Rbe::Star { expr } => write!(dest, "{expr:?}*"),
            Rbe::Plus { expr } => write!(dest, "{expr:?}+"),
            Rbe::Repeat { expr, card } => write!(dest, "({expr:?}){card:?}"),
        }
    }
}

impl<K, V, R> Display for Rbe<K, V, R>
where
    K: Key,
    V: Value,
    R: Ref,
{
    fn fmt(&self, dest: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Rbe::Fail { error } => write!(dest, "Fail {{{error}}}"),
            Rbe::Empty => write!(dest, "Empty"),
            Rbe::Symbol { key, cond, card } => write!(dest, "{key}|{cond}{card}"),
            Rbe::And { exprs } => exprs.iter().try_for_each(|value| write!(dest, "{value};")),
            Rbe::Or { exprs } => exprs.iter().try_for_each(|value| write!(dest, "{value}|")),
            Rbe::Star { expr } => write!(dest, "{expr}*"),
            Rbe::Plus { expr } => write!(dest, "{expr}+"),
            Rbe::Repeat { expr, card } => write!(dest, "({expr}){card}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deriv_a_1_1_and_b_opt_with_a() {
        // a?|b? #= b/2

        impl Ref for i32 {}

        let rbe: Rbe<char, i32, i32> = Rbe::and(vec![
            Rbe::symbol('a', 1, Max::IntMax(1)),
            Rbe::symbol('b', 0, Max::IntMax(1)),
        ]);
        let mut pending = Pending::new();
        let expected = Rbe::and(vec![
            Rbe::symbol('a', 0, Max::IntMax(0)),
            Rbe::symbol('b', 0, Max::IntMax(1)),
        ]);
        assert_eq!(
            rbe.deriv(
                &'a',
                &23,
                1,
                false,
                &HashSet::from(['a', 'b']),
                &mut pending
            ),
            expected
        );
    }

    #[test]
    fn deriv_symbol() {
        let rbe: Rbe<char, i32, i32> = Rbe::symbol('x', 1, Max::IntMax(1));
        let mut pending = Pending::new();
        let d = rbe.deriv(&'x', &2, 1, true, &HashSet::new(), &mut pending);
        assert_eq!(d, Rbe::symbol('x', 0, Max::IntMax(0)));
    }

    #[test]
    fn deriv_symbol_b_2_3() {
        impl Key for String {}
        let rbe: Rbe<String, String, String> = Rbe::symbol("b".to_string(), 2, Max::IntMax(3));
        let mut pending = Pending::new();
        let d = rbe.deriv(
            &"b".to_string(),
            &"vb2".to_string(),
            1,
            true,
            &HashSet::new(),
            &mut pending,
        );
        assert_eq!(Rbe::symbol("b".to_string(), 1, Max::IntMax(2)), d);
    }

    #[test]
    fn symbols() {
        let rbe: Rbe<char, i32, i32> = Rbe::and(vec![
            Rbe::symbol('x', 1, Max::IntMax(1)),
            Rbe::symbol('y', 1, Max::IntMax(1)),
        ]);
        let expected = HashSet::from(['x', 'y']);
        assert_eq!(rbe.symbols(), expected);
    }

    #[test]
    fn symbols2() {
        let rbe: Rbe<char, i32, i32> = Rbe::and(vec![
            Rbe::or(vec![
                Rbe::symbol('x', 1, Max::IntMax(1)),
                Rbe::symbol('y', 2, Max::Unbounded),
            ]),
            Rbe::symbol('y', 1, Max::IntMax(1)),
        ]);
        let expected = HashSet::from(['x', 'y']);
        assert_eq!(rbe.symbols(), expected);
    }

    /*#[test]
    fn test_serialize_rbe() {
        let rbe: Rbe<String, String, String> = Rbe::symbol("foo".to_string(), 1, Max::IntMax(2));
        let expected = indoc! {
        r#"!Symbol
                 key: foo
                 cond: {}
                 card:
                   min: 1
                   max: 2
              "# };
        let rbe: String = serde_yaml::to_string(&rbe).unwrap();
        assert_eq!(rbe, expected);
    }

    #[test]
    fn test_deserialize_rbe() {
        let str = r#"{
            "Symbol": {
                "key": "foo",
                "cond": {},
                "card": {"min": 1, "max": 2 }
            }
        }"#;
        let expected = Rbe::symbol("foo".to_string(), 1, Max::IntMax(2));
        let rbe: Rbe<String, String, String> = serde_json::from_str(str).unwrap();
        assert_eq!(rbe, expected);
    } */
}
