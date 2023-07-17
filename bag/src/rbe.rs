use crate::{Bag, Cardinality, Max, RbeError};
use core::hash::Hash;
use std::cmp;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Clone)]
enum Rbe<A> {
    Fail {
        error: RbeError<A>,
    },
    Empty,
    Symbol {
        value: A,
        card: Cardinality,
    },
    And {
        v1: Box<Rbe<A>>,
        v2: Box<Rbe<A>>,
    },
    Or {
        v1: Box<Rbe<A>>,
        v2: Box<Rbe<A>>,
    },
    Star {
        v: Box<Rbe<A>>,
    },
    Plus {
        v: Box<Rbe<A>>,
    },
    Repeat {
        v: Box<Rbe<A>>,
        min: usize,
        max: Max,
    },
}

type NullableResult = bool;

impl<A> Rbe<A>
where
    A: Eq + Hash + Clone,
{
    fn deriv_bag(&self, bag: Bag<A>, open: bool, controlled: &HashSet<A>) -> Rbe<A> {
        let mut current = (*self).clone();
        for (x, card) in bag.iter() {
            current = self.deriv(&x, card, open, controlled);
        }
        current
    }

    fn nullable(&self) -> NullableResult {
        match &self {
            Rbe::Fail { .. } => false,
            Rbe::Empty => true,
            Rbe::Symbol {
                value: _,
                card: card,
            } if card.nullable() => true,
            Rbe::Symbol {
                value: x,
                card: card,
            } => false,
            Rbe::And { v1, v2 } => Self::combineAnd(v1.nullable(), v2.nullable()),
            Rbe::Or { v1, v2 } => Self::combineOr(v1.nullable(), v2.nullable()),
            Rbe::Star { .. } => true,
            Rbe::Plus { v } => v.nullable(),
            Rbe::Repeat { v, min: 0, max } => true,
            Rbe::Repeat { v, min: _, max: _ } => v.nullable(),
        }
    }

    fn combineAnd(v1: NullableResult, v2: NullableResult) -> NullableResult {
        match (v1, v2) {
            (true, true) => true,
            (true, false) => false,
            (false, true) => false,
            (false, false) => false,
        }
    }

    fn combineOr(v1: NullableResult, v2: NullableResult) -> NullableResult {
        match (v1, v2) {
            (true, true) => true,
            (true, false) => true,
            (false, true) => true,
            (false, false) => false,
        }
    }

    fn deriv(&self, x: &A, n: usize, open: bool, controlled: &HashSet<A>) -> Rbe<A>
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
            Rbe::Symbol { value, card: card } => {
                if *x == *value {
                    if (*card).max == Max::IntMax(0) {
                        Rbe::Fail {
                            error: RbeError::MaxCardinalityZeroFoundValue { x: (*x).clone() },
                        }
                    } else {
                        if let Some(card) = (*card).minus(n) {
                            Self::mkRangeSymbol(x, &card)
                        } else {
                            Rbe::Fail {
                                error: RbeError::CardinalityFail {
                                    n: n,
                                    card: card.clone(),
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
                Self::mkOr(
                    Self::mkAnd(d1, (**v2).clone()),
                    Self::mkAnd(d2, (**v1).clone()),
                )
            }
            Rbe::Or { v1, v2 } => {
                let d1 = v1.deriv(x, n, open, controlled);
                let d2 = v2.deriv(x, n, open, controlled);
                Self::mkOr(d1, d2)
            }
            Rbe::Plus { v } => {
                let d = v.deriv(x, n, open, controlled);
                Self::mkAnd(d, Rbe::Star { v: v.clone() })
            }
            Rbe::Repeat {
                v,
                min: 0,
                max: Max::IntMax(0),
            } => {
                let d = v.deriv(x, n, open, controlled);
                if d.nullable() {
                    Rbe::Fail {
                        error: RbeError::CardinalityZeroZeroDeriv { x: (*x).clone() },
                    }
                } else {
                    Rbe::Empty
                }
            }
            Rbe::Repeat { v, min: min, max } => {
                todo!()
            }
            Rbe::Star { v } => {
                let d = v.deriv(x, n, open, controlled);
                Self::mkAnd(d, (**v).clone())
            }
        }
    }

    fn mkRangeSymbol(x: &A, card: &Cardinality) -> Rbe<A>
    where
        A: Clone,
    {
        if (*card).min < 0 {
            Rbe::Fail {
                error: RbeError::RangeNegativeLowerBound { min: (*card).min },
            }
        } else if Self::bigger((*card).min, &(*card).max) {
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

    fn mkAnd(v1: Rbe<A>, v2: Rbe<A>) -> Rbe<A>
    where
        A: Clone,
    {
        match (&v1, &v2) {
            (Rbe::Empty, _) => v2,
            (_, Rbe::Empty) => v1,
            (f @ Rbe::Fail { .. }, _) => f.clone(),
            (_, f @ Rbe::Fail { .. }) => f.clone(),
            (_, _) => Rbe::And {
                v1: Box::new(v1),
                v2: Box::new(v2),
            },
        }
    }

    fn mkOr(v1: Rbe<A>, v2: Rbe<A>) -> Rbe<A> {
        match (&v1, &v2) {
            (f @ Rbe::Fail { .. }, _) => v2,
            (_, f @ Rbe::Fail { .. }) => v1,
            (e1, e2) => {
                if e1 == e2 {
                    v1
                } else {
                    Rbe::Or {
                        v1: Box::new(v1),
                        v2: Box::new(v2),
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
