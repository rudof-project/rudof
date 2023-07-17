use crate::{Bag, Cardinality, Max, RbeError};
use core::hash::Hash;
use std::cmp;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Rbe<A> {
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

  fn match_bag(&self, bag: Bag<A>, open: bool) -> Result<(), RbeError<A>> {
     match &self.deriv_bag(bag, open, &self.symbols()) {
      f@Rbe::Fail{ error } => Err(error.clone()),
      d => if d.nullable() {
        Ok(())
      } else {
        Err(RbeError::NonNullable{ rest: Box::new(d.clone()) })
      }
     }
  }
  fn empty() -> Rbe<A> {
    Rbe::Empty
  }

  fn symbol(x: A, min: usize, max: Max) -> Rbe<A> {
    Rbe::Symbol{ value: x, card: Cardinality { min, max } }
  }

  fn symbols(&self) -> HashSet<A> {
    let mut set = HashSet::new();
    self.symbols_aux(&mut set);
    set
  }

  fn symbols_aux(&self, set: &mut HashSet<A>) {
    match &self {
      Rbe::Fail{..} => (),
      Rbe::Empty => (),
      Rbe::Symbol { value, card: _} => { set.insert(value.clone());
      },
      Rbe::And{ v1, v2} => {
        v1.symbols_aux(set);
        v2.symbols_aux(set);
      },
      Rbe::Or{ v1, v2} => {
        v1.symbols_aux(set);
        v2.symbols_aux(set);
      },
      Rbe::Plus { v } => {
        v.symbols_aux(set);
      },
      Rbe::Star { v } => {
        v.symbols_aux(set);
      },
      Rbe::Repeat { v, min, max } => {
        v.symbols_aux(set);
      }
    }
  }

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
            (Rbe::Fail { .. }, _) => v2,
            (_, Rbe::Fail { .. }) => v1,
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

#[cfg(test)]
mod tests {
    use super::*;

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
          v2: Box::new(Rbe::symbol('y', 1, Max::IntMax(1)))
        };
        let expected = HashSet::from(['x','y']);
        assert_eq!(rbe.symbols(), expected);
    }

    #[test]
    fn symbols2() {
        let rbe = Rbe::And { 
          v1: Box::new(Rbe::Or { 
                v1: Box::new(Rbe::symbol('x', 1, Max::IntMax(1))), 
                v2: Box::new(Rbe::symbol('y',2,Max::Unbounded))
              }),
          v2: Box::new(Rbe::symbol('y', 1, Max::IntMax(1)))
        };
        let expected = HashSet::from(['x','y']);
        assert_eq!(rbe.symbols(), expected);
    }

    #[test]
    fn match_bag1() {

      // (x{1,1}|y{2,*});y{1,1}
      let rbe = Box::new(Rbe::symbol('y', 1, Max::IntMax(4)));
        let bag = Bag::from(['y','y']);
        assert_eq!(rbe.match_bag(bag, false), Ok(()));
    }
  }
