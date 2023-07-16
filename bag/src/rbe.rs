use std::collections::HashSet;
use core::hash::Hash;
use crate::{Bag, Max, RbeError};
use std::cmp;



#[derive(Clone)]
enum Rbe<A> {
    Fail { error: RbeError<A> },
    Empty,
    Symbol { value: A, min: usize, max: Max },
    And { v1: Box<Rbe<A>>, v2: Box<Rbe<A>> },
    Or { v1: Box<Rbe<A>>, v2: Box<Rbe<A>>},
    Star { v: Box<Rbe<A>>},
    Plus { v: Box<Rbe<A>> },
    Repeat { v: Box<Rbe<A>>, n: usize, m: Max }
}

impl <A> Rbe<A> {
    
    
   fn derivBag(&self, bag: Bag<A>, open: bool) -> Rbe<A> {
    todo!()
   }

   fn deriv(&self, x: &A, open: bool, controlled: &HashSet<A>) -> Rbe<A> 
      where A: Eq + Hash + Clone {
     match &self {
        fail@ Rbe::Fail { error: _ }  => { (*fail).clone() },
        Rbe::Empty => {
            if open && !(controlled.contains(&x)) { 
                Rbe::Empty 
            }
            else { 
                Rbe::Fail{ 
                    error: RbeError::UnexpectedEmpty{ x: (*x).clone() ,open: open }
                } 
            }
        },
        Rbe::Symbol{ value, min, max } => { 
            if *x == *value {
                if *max == Max::IntMax(0) {
                    Rbe::Fail {
                        error: RbeError::MaxCardinalityZeroFoundValue{ x: (*x).clone() }
                    }
                } else { Self::mkRangeSymbol(x, cmp::max(min - 1, 0), (max).minus_one()) }
            } else {
                if open && !(controlled.contains(&x)) {
                  self.clone()
                } else {
                    Rbe::Fail { 
                        error: RbeError::UnexpectedSymbol{ 
                            x: (*x).clone(), 
                            expected: value.clone(), 
                            open: open 
                        } 
                    }
                }
            }
        },
       Rbe::And{ v1, v2} => { 
        let d1 = Box::new(v1.deriv(x, open, controlled));
        let d2 = Box::new(v2.deriv(x, open, controlled));
        *Self::mkOr(
            Self::mkAnd(d1, v2.clone()), 
            Self::mkAnd(d2, v1.clone())
        )
       },
       Rbe::Or{ v1, v2 } => { todo!()},
       Rbe::Plus { v } => { todo!() },
       Rbe::Repeat { v, n, m } => { todo!() },
       Rbe::Star { v } => { todo!() }
     }
   }

   fn mkRangeSymbol(x: &A, min: usize, max: Max) -> Rbe<A> 
   where A: Clone
   {
      if min < 0 { Rbe::Fail {
        error: RbeError::RangeNegativeLowerBound{ min: min }
      }} else if Self::bigger(min, &max)  {
        Rbe::Fail {
            error: RbeError::RangeLowerBoundBiggerMax { 
                min: min, 
                max: max.clone() 
            }
        }
      } else { 
        Rbe::Symbol{ value: (*x).clone(), min: min, max: max}
      }
   }

   fn mkOr(v1: Box<Rbe<A>>, v2: Box<Rbe<A>>) -> Box<Rbe<A>> 
   where A: Clone {
    /* match (*v1,*v2) {
        (Rbe::Empty, e2) => v2,
        (e1, Rbe::Empty) => v1,
        (f@Rbe::Fail{..}, _) => v1,
        (_, f@Rbe::Fail{..}) => v2,
        (_,_) => Box::new(Rbe::And{ 
            v1: Box::new((*v1).clone()), 
            v2: Box::new((*v2).clone()) })
    } */
    todo!()
   }

   fn mkAnd(v1: Box<Rbe<A>>, v2: Box<Rbe<A>>) -> Box<Rbe<A>> {
    todo!()
   }

   fn bigger(min: usize, max: &Max) -> bool {
     match max {
        Max::Unbounded => false,
        Max::IntMax(max) => min > *max 
     }
   }
}