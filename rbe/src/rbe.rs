use crate::{deriv_error::DerivError, deriv_n, Bag, Cardinality, Max, Min};
use core::hash::Hash;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt;
use std::fmt::{Debug, Display};
//use log::debug;
use itertools::cloned;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Pending<A> {
    pending: A,
}

/// Implementation of Regular Bag Expressions
#[derive(Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Rbe<A>
where
    A: Hash + Eq + Display,
{
    Fail {
        error: DerivError<A>,
    },
    #[default]
    Empty,
    Symbol {
        value: A,
        card: Cardinality,
    },
    And {
        values: Vec<Rbe<A>>,
    },
    Or {
        values: Vec<Rbe<A>>,
    },
    Star {
        value: Box<Rbe<A>>,
    },
    Plus {
        value: Box<Rbe<A>>,
    },
    Repeat {
        value: Box<Rbe<A>>,
        card: Cardinality,
    },
}

type NullableResult = bool;

impl<A> Rbe<A>
where
    A: PartialEq + Eq + Hash + Clone + fmt::Debug + fmt::Display,
{
    pub fn match_bag(&self, bag: &Bag<A>, open: bool) -> Result<(), DerivError<A>> {
        match &self.deriv_bag(bag, open, &self.symbols()) {
            Rbe::Fail { error } => Err(error.clone()),
            d => {
                if d.nullable() {
                    Ok(())
                } else {
                    Err(DerivError::NonNullable {
                        non_nullable_rbe: Box::new(d.clone()),
                        bag: (*bag).clone(),
                        expr: Box::new((*self).clone()),
                    })
                }
            }
        }
    }

    pub fn empty() -> Rbe<A> {
        Rbe::Empty
    }

    pub fn symbol(x: A, min: usize, max: Max) -> Rbe<A> {
        Rbe::Symbol {
            value: x,
            card: Cardinality {
                min: Min::from(min),
                max,
            },
        }
    }

    pub fn or<I>(values: I) -> Rbe<A>
    where
        I: IntoIterator<Item = Rbe<A>>,
    {
        let rs: Vec<Rbe<A>> = values.into_iter().collect();
        Rbe::Or { values: rs }
    }

    pub fn and<I>(values: I) -> Rbe<A>
    where
        I: IntoIterator<Item = Rbe<A>>,
    {
        let rs: Vec<Rbe<A>> = values.into_iter().collect();
        Rbe::And { values: rs }
    }

    pub fn opt(v: Rbe<A>) -> Rbe<A> {
        Rbe::Or {
            values: vec![v, Rbe::Empty],
        }
    }

    pub fn plus(v: Rbe<A>) -> Rbe<A> {
        Rbe::Plus { value: Box::new(v) }
    }

    pub fn star(v: Rbe<A>) -> Rbe<A> {
        Rbe::Star { value: Box::new(v) }
    }

    pub fn repeat(v: Rbe<A>, min: usize, max: Max) -> Rbe<A> {
        Rbe::Repeat {
            value: Box::new(v),
            card: Cardinality::from(Min::from(min), max),
        }
    }

    pub fn is_fail(&self) -> bool {
        matches!(self, Rbe::Fail { .. })
    }

    pub fn symbols(&self) -> HashSet<A> {
        let mut set = HashSet::new();
        self.symbols_aux(&mut set);
        set
    }

    fn symbols_aux(&self, set: &mut HashSet<A>) {
        match &self {
            Rbe::Fail { .. } => (),
            Rbe::Empty => (),
            Rbe::Symbol { value, card: _ } => {
                set.insert(value.clone());
            }
            Rbe::And { values } => {
                values.iter().for_each(|v| v.symbols_aux(set));
            }
            Rbe::Or { values } => {
                values.iter().for_each(|v| v.symbols_aux(set));
            }
            Rbe::Plus { value } => {
                value.symbols_aux(set);
            }
            Rbe::Star { value } => {
                value.symbols_aux(set);
            }
            Rbe::Repeat { value, card: _ } => {
                value.symbols_aux(set);
            }
        }
    }

    pub fn deriv_bag(&self, bag: &Bag<A>, open: bool, controlled: &HashSet<A>) -> Rbe<A> {
        let mut current = (*self).clone();
        let mut processed = Bag::new();
        for (x, card) in bag.iter() {
            let deriv = current.deriv(x, card, open, controlled);
            match deriv {
                Rbe::Fail { error } => {
                    current = Rbe::Fail {
                        error: DerivError::DerivBagError {
                            error_msg: format!("{error}"),
                            processed: Box::new(processed),
                            bag: Box::new((*bag).clone()),
                            expr: Box::new((*self).clone()),
                            current: Box::new(current.clone()),
                            value: (*x).clone(),
                            open,
                        },
                    };
                    break;
                }
                _ => {
                    processed.insert((*x).clone());
                    current = deriv;
                }
            }
        }
        current
    }

    fn nullable(&self) -> NullableResult {
        match &self {
            Rbe::Fail { .. } => false,
            Rbe::Empty => true,
            Rbe::Symbol { card, .. } if card.nullable() => true,
            Rbe::Symbol { .. } => false,
            Rbe::And { values } => values.iter().map(|v| v.nullable()).all(|v| v),
            Rbe::Or { values } => values.iter().map(|v| v.nullable()).any(|v| v),
            Rbe::Star { .. } => true,
            Rbe::Plus { value } => value.nullable(),
            Rbe::Repeat { value: _, card } if card.min.is_0() => true,
            Rbe::Repeat { value, card: _ } => value.nullable(),
        }
    }

    pub fn deriv(&self, x: &A, n: usize, open: bool, controlled: &HashSet<A>) -> Rbe<A>
    where
        A: Eq + Hash + Clone,
    {
        match *self {
            ref fail @ Rbe::Fail { .. } => fail.clone(),
            Rbe::Empty => {
                if open && !(controlled.contains(x)) {
                    Rbe::Empty
                } else {
                    Rbe::Fail {
                        error: DerivError::UnexpectedEmpty {
                            x: (*x).clone(),
                            open,
                        },
                    }
                }
            }
            Rbe::Symbol {
                ref value,
                ref card,
            } => {
                if *x == *value {
                    if card.max == Max::IntMax(0) {
                        Rbe::Fail {
                            error: DerivError::MaxCardinalityZeroFoundValue { x: (*x).clone() },
                        }
                    } else if card.contains(n) {
                        let card = card.minus(n);
                        Self::mk_range_symbol(x, &card)
                    } else {
                        Rbe::Fail {
                            error: DerivError::CardinalityFail {
                                symbol: value.clone(),
                                expected_cardinality: card.clone(),
                                current_number: n,
                            },
                        }
                    }
                } else if open && !controlled.contains(x) {
                    // Symbol is different from symbols defined in rbe if the rbe is open, we allow extra symbols
                    self.clone()
                } else {
                    Rbe::Fail {
                        error: DerivError::UnexpectedSymbol {
                            x: (*x).clone(),
                            expected: value.clone(),
                            open,
                        },
                    }
                }
            }
            Rbe::And { ref values } => Self::deriv_and(values, x, n, open, controlled),
            Rbe::Or { ref values } => {
                Self::mk_or_values(values.iter().map(|rbe| rbe.deriv(x, n, open, controlled)))
            }
            Rbe::Plus { ref value } => {
                let d = value.deriv(x, n, open, controlled);
                Self::mk_and(
                    &d,
                    &Rbe::Star {
                        value: value.clone(),
                    },
                )
            }
            Rbe::Repeat {
                ref value,
                ref card,
            } if card.is_0_0() => {
                let d = value.deriv(x, n, open, controlled);
                if d.nullable() {
                    Rbe::Fail {
                        error: DerivError::CardinalityZeroZeroDeriv { symbol: x.clone() },
                    }
                } else {
                    Rbe::Empty
                }
            }
            Rbe::Repeat {
                ref value,
                ref card,
            } => {
                let d = value.deriv(x, n, open, controlled);
                let card = card.minus(n);
                let rest = Self::mk_range(value, &card);
                Self::mk_and(&d, &rest)
            }
            Rbe::Star { ref value } => {
                let d = value.deriv(x, n, open, controlled);
                Self::mk_and(&d, value)
            }
        }
    }

    fn deriv_and(
        values: &Vec<Rbe<A>>,
        x: &A,
        n: usize,
        open: bool,
        controlled: &HashSet<A>,
    ) -> Rbe<A> {
        let mut or_values: Vec<Rbe<A>> = Vec::new();
        let mut failures = crate::deriv_error::Failures::new();

        for vs in deriv_n(cloned((*values).iter()).collect(), |value: &Rbe<A>| {
            let d = value.deriv(x, n, open, controlled);
            match d {
                Rbe::Fail { error } => {
                    failures.push(value.clone(), error);
                    None
                }
                _ => Some(d),
            }
        }) {
            or_values.push(Rbe::And { values: vs });
        }
        match or_values.len() {
            0 => Rbe::Fail {
                error: DerivError::OrValuesFail {
                    e: Box::new(Rbe::And {
                        values: cloned(values.iter()).collect(),
                    }),
                    failures,
                },
            },
            1 => or_values[0].clone(),
            _ => Rbe::Or { values: or_values },
        }
    }

    fn mk_range(e: &Rbe<A>, card: &Cardinality) -> Rbe<A>
    where
        A: Clone,
    {
        if Self::bigger(card.min, &card.max) {
            Rbe::Fail {
                error: DerivError::RangeLowerBoundBiggerMaxExpr {
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
                    value: Box::new(e.clone()),
                    card: c.clone(),
                },
            }
        }
    }

    fn mk_range_symbol(x: &A, card: &Cardinality) -> Rbe<A>
    where
        A: Clone,
    {
        if Self::bigger(card.min, &card.max) {
            Rbe::Fail {
                error: DerivError::RangeLowerBoundBiggerMax {
                    symbol: (*x).clone(),
                    card: card.clone(),
                },
            }
        } else {
            Rbe::Symbol {
                value: (*x).clone(),
                card: card.clone(),
            }
        }
    }

    fn mk_and(v1: &Rbe<A>, v2: &Rbe<A>) -> Rbe<A>
    where
        A: Clone,
    {
        match (v1, v2) {
            (Rbe::Empty, _) => (*v2).clone(),
            (_, Rbe::Empty) => (*v1).clone(),
            (f @ Rbe::Fail { .. }, _) => f.clone(),
            (_, f @ Rbe::Fail { .. }) => f.clone(),
            (_, _) => Rbe::And {
                values: vec![(*v1).clone(), (*v2).clone()],
            },
        }
    }

    fn mk_or_values<I>(values: I) -> Rbe<A>
    where
        I: IntoIterator<Item = Rbe<A>>,
    {
        let init = Rbe::Fail {
            error: DerivError::MkOrValuesFail,
        };

        values
            .into_iter()
            .fold(init, |result, value| Self::mk_or(&result, &value))
    }

    fn mk_or(v1: &Rbe<A>, v2: &Rbe<A>) -> Rbe<A> {
        match (v1, v2) {
            (Rbe::Fail { .. }, _) => (*v2).clone(),
            (_, Rbe::Fail { .. }) => (*v1).clone(),
            (e1, e2) => {
                if e1 == e2 {
                    (*v1).clone()
                } else {
                    Rbe::Or {
                        values: vec![(*v1).clone(), (*v2).clone()],
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

impl<A> Debug for Rbe<A>
where
    A: Debug + Hash + Eq + fmt::Display,
{
    fn fmt(&self, dest: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Rbe::Fail { error } => write!(dest, "Fail {{{error:?}}}"),
            Rbe::Empty => write!(dest, "Empty"),
            Rbe::Symbol { value, card } => write!(dest, "{value:?}{card:?}"),
            Rbe::And { values } => values
                .iter()
                .try_for_each(|value| write!(dest, "{value:?};")),
            Rbe::Or { values } => values
                .iter()
                .try_for_each(|value| write!(dest, "{value:?}|")),
            Rbe::Star { value } => write!(dest, "{value:?}*"),
            Rbe::Plus { value } => write!(dest, "{value:?}+"),
            Rbe::Repeat { value, card } => write!(dest, "({value:?}){card:?}"),
        }
    }
}

impl<A> Display for Rbe<A>
where
    A: Display + Hash + Eq,
{
    fn fmt(&self, dest: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Rbe::Fail { error } => write!(dest, "Fail {{{error}}}"),
            Rbe::Empty => write!(dest, "Empty"),
            Rbe::Symbol { value, card } => write!(dest, "{value}{card}"),
            Rbe::And { values } => values.iter().try_for_each(|value| write!(dest, "{value};")),
            Rbe::Or { values } => values.iter().try_for_each(|value| write!(dest, "{value}|")),
            Rbe::Star { value } => write!(dest, "{value}*"),
            Rbe::Plus { value } => write!(dest, "{value}+"),
            Rbe::Repeat { value, card } => write!(dest, "({value}){card}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    // use test_log::test;

    #[test]
    fn deriv_a_1_1_and_b_opt_with_a() {
        // a?|b? #= b/2
        let rbe = Rbe::and(vec![
            Rbe::symbol('a', 1, Max::IntMax(1)),
            Rbe::symbol('b', 0, Max::IntMax(1)),
        ]);
        let expected = Rbe::and(vec![
            Rbe::symbol('a', 0, Max::IntMax(0)),
            Rbe::symbol('b', 0, Max::IntMax(1)),
        ]);
        assert_eq!(
            rbe.deriv(&'a', 1, false, &HashSet::from(['a', 'b'])),
            expected
        );
    }

    #[test]
    fn deriv_symbol() {
        let rbe = Rbe::symbol('x', 1, Max::IntMax(1));
        let d = rbe.deriv(&'x', 1, true, &HashSet::new());
        assert_eq!(d, Rbe::symbol('x', 0, Max::IntMax(0)));
    }

    #[test]
    fn symbols() {
        let rbe = Rbe::and(vec![
            Rbe::symbol('x', 1, Max::IntMax(1)),
            Rbe::symbol('y', 1, Max::IntMax(1)),
        ]);
        let expected = HashSet::from(['x', 'y']);
        assert_eq!(rbe.symbols(), expected);
    }

    #[test]
    fn symbols2() {
        let rbe = Rbe::and(vec![
            Rbe::or(vec![
                Rbe::symbol('x', 1, Max::IntMax(1)),
                Rbe::symbol('y', 2, Max::Unbounded),
            ]),
            Rbe::symbol('y', 1, Max::IntMax(1)),
        ]);
        let expected = HashSet::from(['x', 'y']);
        assert_eq!(rbe.symbols(), expected);
    }

    #[test]
    fn match_bag_y1_4_y_2() {
        // y{1,4} #= y/2
        let rbe = Rbe::symbol('y', 1, Max::IntMax(4));
        assert_eq!(rbe.match_bag(&Bag::from(['y', 'y']), false), Ok(()));
    }

    #[test]
    fn match_bag_a_opt_or_b_opt_with_a() {
        // a?|b? #= a
        let rbe = Rbe::or(vec![
            Rbe::symbol('a', 0, Max::IntMax(1)),
            Rbe::symbol('b', 0, Max::IntMax(1)),
        ]);
        assert_eq!(rbe.match_bag(&Bag::from(['a']), false), Ok(()));
    }

    #[test]
    fn match_bag_a_opt_or_b_opt_with_b() {
        // a?|b? #= a
        let rbe = Rbe::or(vec![
            Rbe::symbol('a', 0, Max::IntMax(1)),
            Rbe::symbol('b', 0, Max::IntMax(1)),
        ]);
        assert_eq!(rbe.match_bag(&Bag::from(['b']), false), Ok(()));
    }

    #[test]
    fn match_bag_a_opt_and_b_opt_with_ba() {
        // a?|b? #= a
        let rbe = Rbe::and(vec![
            Rbe::symbol('a', 0, Max::IntMax(1)),
            Rbe::symbol('b', 0, Max::IntMax(1)),
        ]);
        assert_eq!(rbe.match_bag(&Bag::from(['b', 'a']), false), Ok(()));
    }

    #[test]
    fn match_bag_a_and_b_opt_with_ab() {
        // a?|b? #= b/2
        let rbe = Rbe::and(vec![
            Rbe::symbol('a', 1, Max::IntMax(1)),
            Rbe::symbol('b', 0, Max::IntMax(1)),
        ]);
        assert_eq!(rbe.match_bag(&Bag::from(['a', 'b']), false), Ok(()));
    }

    #[test]
    fn no_match_bag_a_and_b_opt_with_b_2() {
        // a?|b? #= b/2
        let rbe = Rbe::and(vec![
            Rbe::symbol('a', 1, Max::IntMax(1)),
            Rbe::symbol('b', 0, Max::IntMax(1)),
        ]);
        assert!(rbe.match_bag(&Bag::from(['b', 'b']), false).is_err());
    }

    #[test]
    fn no_match_bag_a_and_b_opt_with_c() {
        // a?|b? #= a
        let rbe = Rbe::and(vec![
            Rbe::symbol('a', 1, Max::IntMax(1)),
            Rbe::symbol('b', 1, Max::IntMax(1)),
        ]);
        assert!(rbe.match_bag(&Bag::from(['c']), false).is_err());
    }

    #[test]
    fn test_serialize_rbe() {
        let rbe = Rbe::symbol("foo".to_string(), 1, Max::IntMax(2));
        let expected = indoc! {
        r#"!Symbol
                 value: foo
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
                "value": "foo",
                "card": {"min": 1, "max": 2 }
            }
        }"#;
        let expected = Rbe::symbol("foo".to_string(), 1, Max::IntMax(2));
        let rbe: Rbe<String> = serde_json::from_str(str).unwrap();
        assert_eq!(rbe, expected);
    }
}
