use crate::failures::Failures;
use crate::{Cardinality, MatchCond, Max, Min, Pending, deriv_n, rbe_error::RbeError};
use crate::{Context, Key, Ref, Value};
use core::hash::Hash;
use itertools::cloned;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt;
use std::fmt::{Debug, Display};

#[derive(Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum RbeCond<K, V, R, Ctx>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
{
    Fail {
        error: RbeError<K, V, R, Ctx>,
    },

    #[default]
    Empty,

    Symbol {
        key: K,
        cond: MatchCond<K, V, R, Ctx>,
        card: Cardinality,
    },

    And {
        exprs: Vec<RbeCond<K, V, R, Ctx>>,
    },
    Or {
        exprs: Vec<RbeCond<K, V, R, Ctx>>,
    },
    Star {
        expr: Box<RbeCond<K, V, R, Ctx>>,
    },
    Plus {
        expr: Box<RbeCond<K, V, R, Ctx>>,
    },
    Repeat {
        expr: Box<RbeCond<K, V, R, Ctx>>,
        card: Cardinality,
    },
}

type NullableResult = bool;

impl<K, V, R, Ctx> RbeCond<K, V, R, Ctx>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
{
    pub fn empty() -> RbeCond<K, V, R, Ctx> {
        RbeCond::Empty
    }

    pub fn symbol(key: K, min: usize, max: Max) -> RbeCond<K, V, R, Ctx> {
        RbeCond::Symbol {
            key,
            cond: MatchCond::default(),
            card: Cardinality {
                min: Min::from(min),
                max,
            },
        }
    }

    pub fn symbol_cond(key: K, cond: MatchCond<K, V, R, Ctx>, min: Min, max: Max) -> RbeCond<K, V, R, Ctx> {
        RbeCond::Symbol {
            key,
            cond,
            card: Cardinality { min, max },
        }
    }

    pub fn or<I>(exprs: I) -> RbeCond<K, V, R, Ctx>
    where
        I: IntoIterator<Item = RbeCond<K, V, R, Ctx>>,
    {
        let rs = exprs.into_iter().collect();
        RbeCond::Or { exprs: rs }
    }

    pub fn and<I>(exprs: I) -> RbeCond<K, V, R, Ctx>
    where
        I: IntoIterator<Item = RbeCond<K, V, R, Ctx>>,
    {
        let rs = exprs.into_iter().collect();
        RbeCond::And { exprs: rs }
    }

    pub fn opt(v: RbeCond<K, V, R, Ctx>) -> RbeCond<K, V, R, Ctx> {
        RbeCond::Or {
            exprs: vec![v, RbeCond::Empty],
        }
    }

    pub fn plus(expr: RbeCond<K, V, R, Ctx>) -> RbeCond<K, V, R, Ctx> {
        RbeCond::Plus { expr: Box::new(expr) }
    }

    pub fn star(expr: RbeCond<K, V, R, Ctx>) -> RbeCond<K, V, R, Ctx> {
        RbeCond::Star { expr: Box::new(expr) }
    }

    pub fn repeat(expr: RbeCond<K, V, R, Ctx>, min: usize, max: Max) -> RbeCond<K, V, R, Ctx> {
        RbeCond::Repeat {
            expr: Box::new(expr),
            card: Cardinality::from(Min::from(min), max),
        }
    }

    pub fn is_fail(&self) -> bool {
        matches!(self, RbeCond::Fail { .. })
    }

    pub fn symbols(&self) -> HashSet<K> {
        let mut set = HashSet::new();
        self.symbols_aux(&mut set);
        set
    }

    fn symbols_aux(&self, set: &mut HashSet<K>) {
        match &self {
            RbeCond::Fail { .. } => (),
            RbeCond::Empty => (),
            RbeCond::Symbol { key, .. } => {
                set.insert(key.clone());
            },
            RbeCond::And { exprs } => {
                exprs.iter().for_each(|v| v.symbols_aux(set));
            },
            RbeCond::Or { exprs } => {
                exprs.iter().for_each(|v| v.symbols_aux(set));
            },
            RbeCond::Plus { expr } => {
                expr.symbols_aux(set);
            },
            RbeCond::Star { expr } => {
                expr.symbols_aux(set);
            },
            RbeCond::Repeat { expr, card: _ } => {
                expr.symbols_aux(set);
            },
        }
    }

    pub fn nullable(&self) -> NullableResult {
        match &self {
            RbeCond::Fail { .. } => false,
            RbeCond::Empty => true,
            RbeCond::Symbol { card, .. } if card.nullable() => true,
            RbeCond::Symbol { .. } => false,
            RbeCond::And { exprs } => exprs.iter().all(|v| v.nullable()),
            RbeCond::Or { exprs } => exprs.iter().any(|v| v.nullable()),
            RbeCond::Star { .. } => true,
            RbeCond::Plus { expr } => expr.nullable(),
            RbeCond::Repeat { expr: _, card } if card.min.is_0() => true,
            RbeCond::Repeat { expr, card: _ } => expr.nullable(),
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
        ctx: &Ctx,
        n: usize,
        open: bool,
        controlled: &HashSet<K>,
        pending: &mut Pending<V, R>,
    ) -> RbeCond<K, V, R, Ctx>
    where
        K: Eq + Hash + Clone,
    {
        match *self {
            ref fail @ RbeCond::Fail { .. } => fail.clone(),
            RbeCond::Empty => {
                if open && !(controlled.contains(symbol)) {
                    RbeCond::Empty
                } else {
                    RbeCond::Fail {
                        error: RbeError::UnexpectedEmpty {
                            x: (*symbol).clone(),
                            open,
                        },
                    }
                }
            },
            RbeCond::Symbol {
                ref key,
                ref cond,
                ref card,
            } => {
                if *key == *symbol {
                    match cond.matches(value, ctx) {
                        Err(err) => RbeCond::Fail { error: err },
                        Ok(new_pending) => {
                            if card.max == Max::IntMax(0) {
                                RbeCond::Fail {
                                    error: RbeError::MaxCardinalityZeroFoundValue { x: (*symbol).clone() },
                                }
                            } else {
                                let new_card = card.minus(n);
                                (*pending).merge(new_pending);
                                Self::mk_range_symbol(symbol, cond, &new_card)
                            }
                        },
                    }
                } else {
                    // Symbol is different from symbols defined in Rbe
                    // if the Rbe is open, we allow extra symbols
                    // unless the controlled symbols
                    if open && !(controlled.contains(symbol)) {
                        self.clone()
                    } else {
                        RbeCond::Fail {
                            error: RbeError::UnexpectedSymbol {
                                x: (*symbol).clone(),
                                expected: key.clone(),
                                open,
                            },
                        }
                    }
                }
            },
            RbeCond::And { ref exprs } => Self::deriv_and(exprs, symbol, value, ctx, n, open, controlled, pending),
            RbeCond::Or { ref exprs } => Self::mk_or_values(
                exprs
                    .iter()
                    .map(|rbe| rbe.deriv(symbol, value, ctx, n, open, controlled, pending)),
            ),
            RbeCond::Plus { ref expr } => {
                let d = expr.deriv(symbol, value, ctx, n, open, controlled, pending);
                Self::mk_and(&d, &RbeCond::Star { expr: expr.clone() })
            },
            RbeCond::Repeat { ref expr, ref card } if card.is_0_0() => {
                let d = expr.deriv(symbol, value, ctx, n, open, controlled, pending);
                if d.nullable() {
                    RbeCond::Fail {
                        error: RbeError::CardinalityZeroZeroDeriv { symbol: symbol.clone() },
                    }
                } else {
                    RbeCond::Empty
                }
            },
            RbeCond::Repeat { ref expr, ref card } => {
                let d = expr.deriv(symbol, value, ctx, n, open, controlled, pending);
                let card = card.minus(n);
                let rest = Self::mk_range(expr, &card);
                Self::mk_and(&d, &rest)
            },
            RbeCond::Star { ref expr } => {
                let d = expr.deriv(symbol, value, ctx, n, open, controlled, pending);
                Self::mk_and(&d, expr)
            },
        }
    }

    fn deriv_and(
        values: &Vec<RbeCond<K, V, R, Ctx>>,
        symbol: &K,
        value: &V,
        ctx: &Ctx,
        n: usize,
        open: bool,
        controlled: &HashSet<K>,
        pending: &mut Pending<V, R>,
    ) -> RbeCond<K, V, R, Ctx> {
        let mut or_values: Vec<RbeCond<K, V, R, Ctx>> = Vec::new();
        let mut failures = Failures::new();

        for vs in deriv_n(cloned((*values).iter()).collect(), |expr: &RbeCond<K, V, R, Ctx>| {
            let d = expr.deriv(symbol, value, ctx, n, open, controlled, pending);
            match d {
                RbeCond::Fail { error } => {
                    failures.push(expr.clone(), error);
                    None
                },
                _ => Some(d),
            }
        }) {
            or_values.push(RbeCond::And { exprs: vs });
        }
        match or_values.len() {
            0 => RbeCond::Fail {
                error: RbeError::OrValuesFail {
                    e: Box::new(RbeCond::And {
                        exprs: cloned(values.iter()).collect(),
                    }),
                    failures,
                },
            },
            1 => or_values[0].clone(),
            _ => RbeCond::Or { exprs: or_values },
        }
    }

    fn mk_range(e: &RbeCond<K, V, R, Ctx>, card: &Cardinality) -> RbeCond<K, V, R, Ctx>
    where
        K: Clone,
    {
        if Self::bigger(card.min, &card.max) {
            RbeCond::Fail {
                error: RbeError::RangeLowerBoundBiggerMaxExpr {
                    expr: Box::new((*e).clone()),
                    card: card.clone(),
                },
            }
        } else {
            match (e, card) {
                (_, c) if c.is_0_0() => RbeCond::Empty,
                (e, c) if c.is_1_1() => e.clone(),
                (fail @ RbeCond::Fail { .. }, _) => fail.clone(),
                (RbeCond::Empty, _) => RbeCond::Empty,
                (e, c) => RbeCond::Repeat {
                    expr: Box::new(e.clone()),
                    card: c.clone(),
                },
            }
        }
    }

    fn mk_range_symbol(x: &K, cond: &MatchCond<K, V, R, Ctx>, card: &Cardinality) -> RbeCond<K, V, R, Ctx>
    where
        K: Clone,
    {
        if Self::bigger(card.min, &card.max) {
            RbeCond::Fail {
                error: RbeError::RangeLowerBoundBiggerMax {
                    symbol: (*x).clone(),
                    card: card.clone(),
                },
            }
        } else {
            RbeCond::Symbol {
                key: (*x).clone(),
                cond: (*cond).clone(),
                card: card.clone(),
            }
        }
    }

    fn mk_and(v1: &RbeCond<K, V, R, Ctx>, v2: &RbeCond<K, V, R, Ctx>) -> RbeCond<K, V, R, Ctx>
    where
        K: Clone,
    {
        match (v1, v2) {
            (RbeCond::Empty, _) => (*v2).clone(),
            (_, RbeCond::Empty) => (*v1).clone(),
            (f @ RbeCond::Fail { .. }, _) => f.clone(),
            (_, f @ RbeCond::Fail { .. }) => f.clone(),
            (_, _) => RbeCond::And {
                exprs: vec![(*v1).clone(), (*v2).clone()],
            },
        }
    }

    fn mk_or_values<I>(values: I) -> RbeCond<K, V, R, Ctx>
    where
        I: IntoIterator<Item = RbeCond<K, V, R, Ctx>>,
    {
        let init = RbeCond::Fail {
            error: RbeError::MkOrValuesFail,
        };

        values
            .into_iter()
            .fold(init, |result, value| Self::mk_or(&result, &value))
    }

    fn mk_or(v1: &RbeCond<K, V, R, Ctx>, v2: &RbeCond<K, V, R, Ctx>) -> RbeCond<K, V, R, Ctx> {
        match (v1, v2) {
            (RbeCond::Fail { .. }, _) => (*v2).clone(),
            (_, RbeCond::Fail { .. }) => (*v1).clone(),
            (e1, e2) => {
                if e1 == e2 {
                    (*v1).clone()
                } else {
                    RbeCond::Or {
                        exprs: vec![(*v1).clone(), (*v2).clone()],
                    }
                }
            },
        }
    }

    fn bigger(min: Min, max: &Max) -> bool {
        match max {
            Max::Unbounded => false,
            Max::IntMax(max) => min.value > *max,
        }
    }
}

impl<K, V, R, Ctx> Debug for RbeCond<K, V, R, Ctx>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
{
    fn fmt(&self, dest: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            RbeCond::Fail { error } => write!(dest, "Fail {{{error:?}}}"),
            RbeCond::Empty => write!(dest, "Empty"),
            RbeCond::Symbol { key, cond, card } => write!(dest, "{key:?}|{cond:?}{card:?}"),
            RbeCond::And { exprs } => exprs.iter().try_for_each(|value| write!(dest, "{value:?};")),
            RbeCond::Or { exprs } => exprs.iter().try_for_each(|value| write!(dest, "{value:?}|")),
            RbeCond::Star { expr } => write!(dest, "{expr:?}*"),
            RbeCond::Plus { expr } => write!(dest, "{expr:?}+"),
            RbeCond::Repeat { expr, card } => write!(dest, "({expr:?}){card:?}"),
        }
    }
}

impl<K, V, R, Ctx> Display for RbeCond<K, V, R, Ctx>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
{
    fn fmt(&self, dest: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            RbeCond::Fail { error } => write!(dest, "Fail {{{error}}}"),
            RbeCond::Empty => write!(dest, "Empty"),
            RbeCond::Symbol { key, cond, card } => write!(dest, "{key}|{cond}{card}"),
            RbeCond::And { exprs } => exprs.iter().try_for_each(|value| write!(dest, "{value};")),
            RbeCond::Or { exprs } => exprs.iter().try_for_each(|value| write!(dest, "{value}|")),
            RbeCond::Star { expr } => write!(dest, "{expr}*"),
            RbeCond::Plus { expr } => write!(dest, "{expr}+"),
            RbeCond::Repeat { expr, card } => write!(dest, "({expr}){card}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Ref for i32 {}

    impl Key for String {}

    impl Context for char {}

    #[test]
    fn deriv_a_1_1_and_b_opt_with_a() {
        // a?|b? #= b/2

        let rbe: RbeCond<char, i32, i32, char> = RbeCond::and(vec![
            RbeCond::symbol('a', 1, Max::IntMax(1)),
            RbeCond::symbol('b', 0, Max::IntMax(1)),
        ]);
        let mut pending = Pending::new();
        let expected = RbeCond::and(vec![
            RbeCond::symbol('a', 0, Max::IntMax(0)),
            RbeCond::symbol('b', 0, Max::IntMax(1)),
        ]);
        assert_eq!(
            rbe.deriv(&'a', &23, &'a', 1, false, &HashSet::from(['a', 'b']), &mut pending),
            expected
        );
    }

    #[test]
    fn deriv_symbol() {
        let rbe: RbeCond<char, i32, i32, char> = RbeCond::symbol('x', 1, Max::IntMax(1));
        let mut pending = Pending::new();
        let d = rbe.deriv(&'x', &2, &'a', 1, true, &HashSet::new(), &mut pending);
        assert_eq!(d, RbeCond::symbol('x', 0, Max::IntMax(0)));
    }

    #[test]
    fn deriv_symbol_b_2_3() {
        let rbe: RbeCond<String, String, String, char> = RbeCond::symbol("b".to_string(), 2, Max::IntMax(3));
        let mut pending = Pending::new();
        let d = rbe.deriv(
            &"b".to_string(),
            &"vb2".to_string(),
            &'a',
            1,
            true,
            &HashSet::new(),
            &mut pending,
        );
        assert_eq!(RbeCond::symbol("b".to_string(), 1, Max::IntMax(2)), d);
    }

    #[test]
    fn symbols() {
        let rbe: RbeCond<char, i32, i32, char> = RbeCond::and(vec![
            RbeCond::symbol('x', 1, Max::IntMax(1)),
            RbeCond::symbol('y', 1, Max::IntMax(1)),
        ]);
        let expected = HashSet::from(['x', 'y']);
        assert_eq!(rbe.symbols(), expected);
    }

    #[test]
    fn symbols2() {
        let rbe: RbeCond<char, i32, i32, char> = RbeCond::and(vec![
            RbeCond::or(vec![
                RbeCond::symbol('x', 1, Max::IntMax(1)),
                RbeCond::symbol('y', 2, Max::Unbounded),
            ]),
            RbeCond::symbol('y', 1, Max::IntMax(1)),
        ]);
        let expected = HashSet::from(['x', 'y']);
        assert_eq!(rbe.symbols(), expected);
    }
}
