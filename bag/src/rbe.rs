use crate::{Bag, Cardinality, Max, RbeError};
use core::hash::Hash;
use std::collections::HashSet;
use std::{cmp, fmt};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Rbe<A>
where
    A: Hash + Eq,
{
    Fail { error: RbeError<A> },
    Empty,
    Symbol { value: A, card: Cardinality },
    And { v1: Box<Rbe<A>>, v2: Box<Rbe<A>> },
    Or { v1: Box<Rbe<A>>, v2: Box<Rbe<A>> },
    Star { v: Box<Rbe<A>> },
    Plus { v: Box<Rbe<A>> },
    Repeat { v: Box<Rbe<A>>, card: Cardinality },
}

type NullableResult = bool;

impl<A> Rbe<A>
where
    A: PartialEq + Eq + Hash + Clone + fmt::Debug,
{
    pub fn match_bag(&self, bag: &Bag<A>, open: bool) -> Result<(), RbeError<A>> {
        match &self.deriv_bag(bag, open, &self.symbols()) {
            f @ Rbe::Fail { error } => Err(error.clone()),
            d => {
                if d.nullable() {
                    println!(
                        "Finished symbols: resulting rbe = {:?} which is nullable",
                        d
                    );
                    Ok(())
                } else {
                    println!(
                        "Finished symbols: resulting rbe = {:?} which is non-nullable",
                        d
                    );
                    Err(RbeError::NonNullable {
                        non_nullable_rbe: Box::new(d.clone()),
                        bag: (*bag).clone(),
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
            card: Cardinality { min, max },
        }
    }

    pub fn or(v1: Rbe<A>, v2: Rbe<A>) -> Rbe<A> {
        Rbe::Or {
            v1: Box::new(v1),
            v2: Box::new(v2),
        }
    }

    pub fn and(v1: Rbe<A>, v2: Rbe<A>) -> Rbe<A> {
        Rbe::And {
            v1: Box::new(v1),
            v2: Box::new(v2),
        }
    }

    pub fn opt(v: Rbe<A>) -> Rbe<A> {
        Rbe::Or {
            v1: Box::new(v),
            v2: Box::new(Rbe::Empty),
        }
    }

    pub fn plus(v: Rbe<A>) -> Rbe<A> {
        Rbe::Plus { v: Box::new(v) }
    }

    pub fn star(v: Rbe<A>) -> Rbe<A> {
        Rbe::Star { v: Box::new(v) }
    }

    pub fn repeat(v: Rbe<A>, min: usize, max: Max) -> Rbe<A> {
        Rbe::Repeat {
            v: Box::new(v),
            card: Cardinality::from(min, max),
        }
    }

    fn symbols(&self) -> HashSet<A> {
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
            Rbe::And { v1, v2 } => {
                v1.symbols_aux(set);
                v2.symbols_aux(set);
            }
            Rbe::Or { v1, v2 } => {
                v1.symbols_aux(set);
                v2.symbols_aux(set);
            }
            Rbe::Plus { v } => {
                v.symbols_aux(set);
            }
            Rbe::Star { v } => {
                v.symbols_aux(set);
            }
            Rbe::Repeat { v, card: _ } => {
                v.symbols_aux(set);
            }
        }
    }

    pub fn deriv_bag(&self, bag: &Bag<A>, open: bool, controlled: &HashSet<A>) -> Rbe<A> {
        let mut current = (*self).clone();
        for (x, card) in bag.iter() {
            current = self.deriv(&x, card, open, controlled);
            println!("Checking: {:?}, deriv: {:?}", x, current);
        }
        current
    }

    fn nullable(&self) -> NullableResult {
        match &self {
            Rbe::Fail { .. } => false,
            Rbe::Empty => true,
            Rbe::Symbol { value: _, card } if card.nullable() => true,
            Rbe::Symbol { .. } => false,
            Rbe::And { v1, v2 } => Self::combine_and(v1.nullable(), v2.nullable()),
            Rbe::Or { v1, v2 } => Self::combine_or(v1.nullable(), v2.nullable()),
            Rbe::Star { .. } => true,
            Rbe::Plus { v } => v.nullable(),
            Rbe::Repeat { v: _, card } if card.min == 0 => true,
            Rbe::Repeat { v, card: __ } => v.nullable(),
        }
    }

    fn combine_and(v1: NullableResult, v2: NullableResult) -> NullableResult {
        match (v1, v2) {
            (true, true) => true,
            (true, false) => false,
            (false, true) => false,
            (false, false) => false,
        }
    }

    fn combine_or(v1: NullableResult, v2: NullableResult) -> NullableResult {
        match (v1, v2) {
            (true, true) => true,
            (true, false) => true,
            (false, true) => true,
            (false, false) => false,
        }
    }

    pub fn deriv(&self, x: &A, n: usize, open: bool, controlled: &HashSet<A>) -> Rbe<A>
    where
        A: Eq + Hash + Clone,
    {
        match &self {
            fail @ Rbe::Fail { error: _ } => (*fail).clone(),
            Rbe::Empty => {
                if open && !(controlled.contains(&x)) {
                    Rbe::Empty
                } else {
                    Rbe::Fail {
                        error: RbeError::UnexpectedEmpty {
                            x: (*x).clone(),
                            open: open,
                        },
                    }
                }
            }
            Rbe::Symbol { value, card } => {
                if *x == *value {
                    if (*card).max == Max::IntMax(0) {
                        Rbe::Fail {
                            error: RbeError::MaxCardinalityZeroFoundValue { x: (*x).clone() },
                        }
                    } else {
                        if let Some(card) = (*card).minus(n) {
                            Self::mk_range_symbol(x, &card)
                        } else {
                            Rbe::Fail {
                                error: RbeError::CardinalityFail {
                                    symbol: (*value).clone(),
                                    expected_cardinality: card.clone(),
                                    current_number: n,
                                },
                            }
                        }
                    }
                } else {
                    // Symbol is different from symbols defined in rbe
                    // if the rbe is open, we allow extra symbols
                    if open && !(controlled.contains(&x)) {
                        self.clone()
                    } else {
                        Rbe::Fail {
                            error: RbeError::UnexpectedSymbol {
                                x: (*x).clone(),
                                expected: value.clone(),
                                open: open,
                            },
                        }
                    }
                }
            }
            Rbe::And { v1, v2 } => {
                let d1 = v1.deriv(x, n, open, controlled);
                let d2 = v2.deriv(x, n, open, controlled);
                let result = Self::mk_or(
                    &Self::mk_and(&d1, v2),
                    &Self::mk_and(&d2, v2),
                );
                println!("Case And\nv1={v1:?}\nd_{x:?}(v1)={d2:?}\nv2={v2:?}\nd_{x:?}(v2)={d2:?}");
                result
            }
            Rbe::Or { v1, v2 } => {
                let d1 = v1.deriv(x, n, open, controlled);
                let d2 = v2.deriv(x, n, open, controlled);
                Self::mk_or(&d1, &d2)
            }
            Rbe::Plus { v } => {
                let d = v.deriv(x, n, open, controlled);
                Self::mk_and(&d, &Rbe::Star { v: v.clone() })
            }
            /*             Rbe::Repeat { v, card }
              if card.min == 0 && card.max == Max::IntMax(0) => {
                let d = v.deriv(x, n, open, controlled);
                if d.nullable() {
                    Rbe::Fail {
                        error: RbeError::CardinalityZeroZeroDeriv {
                            x: (*x).clone(),
                            card: (*card).clone()
                        },
                    }
                } else {
                    Rbe::Empty
                }
            } */
            Rbe::Repeat { v, card } => {
                todo!()
            }
            Rbe::Star { v } => {
                let d = v.deriv(x, n, open, controlled);
                Self::mk_and(&d, v)
            }
        }
    }

    fn mk_range_symbol(x: &A, card: &Cardinality) -> Rbe<A>
    where
        A: Clone,
    {
        if Self::bigger((*card).min, &(*card).max) {
            Rbe::Fail {
                error: RbeError::RangeLowerBoundBiggerMax { card: card.clone() },
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
                v1: Box::new((*v1).clone()),
                v2: Box::new((*v2).clone()),
            },
        }
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
                        v1: Box::new((*v1).clone()),
                        v2: Box::new((*v2).clone()),
                    }
                }
            }
        }
    }

    fn bigger(min: usize, max: &Max) -> bool {
        match max {
            Max::Unbounded => false,
            Max::IntMax(max) => min > *max,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deriv_a_1_1_and_b_opt_with_a() {
        // a?|b? #= b/2
        let rbe = Rbe::and(
            Rbe::symbol('a', 1, Max::IntMax(1)),
            Rbe::symbol('b', 0, Max::IntMax(1)),
        );
        let expected = Rbe::and(
            Rbe::symbol('a', 0, Max::IntMax(0)),
            Rbe::symbol('b', 0, Max::IntMax(1)),
        );
        println!("Before assert!");
        assert_eq!(rbe.deriv(&'a',1,false, &HashSet::from(['a','b'])), expected);
    }

    #[test]
    fn deriv_symbol() {
        let rbe = Rbe::symbol('x', 1, Max::IntMax(1));
        let d = rbe.deriv(&'x', 1, true, &HashSet::new());
        assert_eq!(d, Rbe::symbol('x', 0, Max::IntMax(0)));
    }

    #[test]
    fn symbols() {
        let rbe = Rbe::And {
            v1: Box::new(Rbe::symbol('x', 1, Max::IntMax(1))),
            v2: Box::new(Rbe::symbol('y', 1, Max::IntMax(1))),
        };
        let expected = HashSet::from(['x', 'y']);
        assert_eq!(rbe.symbols(), expected);
    }

    #[test]
    fn symbols2() {
        let rbe = Rbe::And {
            v1: Box::new(Rbe::Or {
                v1: Box::new(Rbe::symbol('x', 1, Max::IntMax(1))),
                v2: Box::new(Rbe::symbol('y', 2, Max::Unbounded)),
            }),
            v2: Box::new(Rbe::symbol('y', 1, Max::IntMax(1))),
        };
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
        let rbe = Rbe::or(
            Rbe::symbol('a', 0, Max::IntMax(1)),
            Rbe::symbol('b', 0, Max::IntMax(1)),
        );
        assert_eq!(rbe.match_bag(&Bag::from(['a']), false), Ok(()));
    }

    #[test]
    fn match_bag_a_opt_or_b_opt_with_b() {
        // a?|b? #= a
        let rbe = Rbe::or(
            Rbe::symbol('a', 0, Max::IntMax(1)),
            Rbe::symbol('b', 0, Max::IntMax(1)),
        );
        assert_eq!(rbe.match_bag(&Bag::from(['b']), false), Ok(()));
    }

    #[test]
    fn match_bag_a_opt_and_b_opt_with_ba() {
        // a?|b? #= a
        let rbe = Rbe::and(
            Rbe::symbol('a', 0, Max::IntMax(1)),
            Rbe::symbol('b', 0, Max::IntMax(1)),
        );
        assert_eq!(rbe.match_bag(&Bag::from(['b', 'a']), false), Ok(()));
    }

    #[test]
    fn match_bag_a_and_b_opt_with_ab() {
        // a?|b? #= b/2
        let rbe = Rbe::and(
            Rbe::symbol('a', 1, Max::IntMax(1)),
            Rbe::symbol('b', 0, Max::IntMax(1)),
        );
        println!("Before assert!");
        assert_eq!(rbe.match_bag(&Bag::from(['a', 'b']), false), Ok(()));
    }

    #[test]
    fn no_match_bag_a_and_b_opt_with_b_2() {
        // a?|b? #= b/2
        let rbe = Rbe::and(
            Rbe::symbol('a', 1, Max::IntMax(1)),
            Rbe::symbol('b', 0, Max::IntMax(1)),
        );
        assert!(rbe.match_bag(&Bag::from(['b', 'b']), false).is_err());
    }

    #[test]
    fn no_match_bag_a_and_b_opt_with_c() {
        // a?|b? #= a
        let rbe = Rbe::and(
            Rbe::symbol('a', 1, Max::IntMax(1)),
            Rbe::symbol('b', 1, Max::IntMax(1)),
        );
        assert!(rbe.match_bag(&Bag::from(['c']), false).is_err());
    }
}
