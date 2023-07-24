use crate::{Bag, Cardinality, Max, RbeError};
use core::hash::Hash;
use std::collections::HashSet;
use std::fmt::{Debug, Display};
use std::{cmp, fmt};
use serde_derive::{Deserialize, Serialize};
use log::debug;

/// Implementation of Regular Bag Expressions
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Rbe<A>
where
    A: Hash + Eq,
{
    Fail { error: RbeError<A> },
    Empty,
    Symbol { value: A, card: Cardinality },
    And { values: Vec<Box<Rbe<A>>> },
    Or { values: Vec<Box<Rbe<A>>> },
    Star { value: Box<Rbe<A>> },
    Plus { value: Box<Rbe<A>> },
    Repeat { value: Box<Rbe<A>>, card: Cardinality },
}

type NullableResult = bool;

impl<A> Rbe<A>
where
    A: PartialEq + Eq + Hash + Clone + fmt::Debug,
{
    pub fn match_bag(&self, bag: &Bag<A>, open: bool) -> Result<(), RbeError<A>> {
        match &self.deriv_bag(bag, open, &self.symbols()) {
            Rbe::Fail { error } => {
                debug!("deriv_bag failed with error {error:?}");
                Err(error.clone())
            },
            d => {
                if d.nullable() {
                    debug!(
                        "Finished symbols: resulting rbe = {:?} which is nullable",
                        d
                    );
                    Ok(())
                } else {
                    debug!(
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

    pub fn or<I>(values: I) -> Rbe<A> 
    where I: IntoIterator<Item = Rbe<A>> {
        let rs: Vec<Box<Rbe<A>>> = values.into_iter().map(|v| Box::new(v)).collect();
        Rbe::Or{ values: rs}
    }

    pub fn and<I>(values: I) -> Rbe<A>
    where I: IntoIterator<Item = Rbe<A>> {
        let rs: Vec<Box<Rbe<A>>> = values.into_iter().map(|v| Box::new(v)).collect();
        Rbe::And{ values: rs}
    }

    pub fn opt(v: Rbe<A>) -> Rbe<A> {
        Rbe::Or {
            values: vec![Box::new(v), Box::new(Rbe::Empty)]
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
            card: Cardinality::from(min, max),
        }
    }

    fn is_fail(&self) -> bool {
        match &self {
            Rbe::Fail {..} => true,
            _ => false
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
            Rbe::And { values } => {
                values.iter().for_each(|v| v.symbols_aux(set));
            }
            Rbe::Or { values} => {
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
        for (x, card) in bag.iter() {
            current = current.deriv(&x, card, open, controlled);
            if current.is_fail() {
                debug!("Found failed in deriv {current:?}");
                break;
            }
            debug!("Checking: {:?}, deriv: {:?}", x, &current);
        }
        current
    }

    fn nullable(&self) -> NullableResult {
        match &self {
            Rbe::Fail { .. } => false,
            Rbe::Empty => true,
            Rbe::Symbol { value: _, card } if card.nullable() => true,
            Rbe::Symbol { .. } => false,
            Rbe::And { values } => {
               values.iter()
               .map(|v| v.nullable())
               .all(|v| v == true)
            },
            Rbe::Or { values } => {
                values.iter()
                .map(|v| v.nullable())
                .any(|v| v == true)},
            Rbe::Star { .. } => true,
            Rbe::Plus { value } => value.nullable(),
            Rbe::Repeat { value: _, card } if card.min == 0 => true,
            Rbe::Repeat { value, card: __ } => value.nullable(),
        }
    }


    pub fn deriv(&self, x: &A, n: usize, open: bool, controlled: &HashSet<A>) -> Rbe<A>
    where
        A: Eq + Hash + Clone,
    {
        dbg!(&self);
        dbg!(x);
        match *self {
            ref fail @ Rbe::Fail { .. } => fail.clone(),
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
            Rbe::Symbol { ref value, ref card } => {
                if *x == *value {
                    if card.max == Max::IntMax(0) {
                        Rbe::Fail {
                            error: RbeError::MaxCardinalityZeroFoundValue { x: (*x).clone() },
                        }
                    } else {
                        if let Some(card) = card.minus(n) {
                            Self::mk_range_symbol(x, &card)
                        } else {
                            Rbe::Fail {
                                error: RbeError::CardinalityFail {
                                    symbol: value.clone(),
                                    expected_cardinality: card.clone(),
                                    current_number: n,
                                },
                            }
                        }
                    }
                } else {
                    // Symbol is different from symbols defined in rbe
                    // if the rbe is open, we allow extra symbols
                    dbg!(value);
                    dbg!(x);
                    dbg!(controlled);
                    dbg!(open);
                    if open && !(controlled.contains(&x)) {
                        debug!("Open condition satisfied!");
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
            Rbe::And { ref values } => {
                debug!("And {{ values: {values:?} }}");
                /*let mut it = values.iter();
                let rs = it.reduce(|current,v| {
                   let d1 = v.deriv(x, n, open, controlled);
                   let d2 = current.deriv(x, n, open, controlled);
                   let result = Self::mk_or(
                    &Self::mk_and(&d1, &current),
                    &Self::mk_and(&d2, &v),
                   ); 
                   Box::new(result) 
                   Self::deriv_and(current,v)
                });
                let x = rs.unwrap();
                let v = *x;
                let w = *v;
                w */
                // let init = Rbe::Empty;
                let mut it = values.iter();
                let next = it.next().unwrap();
                let init = Self::mk_first(next);
                let result = values.iter().fold(init, |current,v| {
                    let d1 = v.deriv(x,n,open,controlled);
                    debug!("d_{x:?}(v = {v:?}) = {d1:?}");
                    let d2 = current.deriv(x,n,open,controlled);
                    debug!("d_{x:?}(current = {current:?}) = {d2:?}");
                    let result = Self::mk_or(
                        &Self::mk_and(&d1, &current),
                        &Self::mk_and(&d2, &v),
                    ); 
                    debug!("Result of mk_or = {result:?}");
                    result
                });
                debug!("Case And\nvalues={values:?}\nx:{x:?}\nResult: {result:?}");
                result
                /*let init = Rbe::Empty;
                let result = values.iter().fold(init, |current,v| {
                    let d1 = v.deriv(x,n,open,controlled);
                    debug!("d_{x:?}(v = {v:?}) = {d1:?}");
                    let d2 = current.deriv(x,n,open,controlled);
                    debug!("d_{x:?}(current = {current:?}) = {d2:?}");
                    let result = Self::mk_or(
                        &Self::mk_and(&d1, &current),
                        &Self::mk_and(&d2, &v),
                    ); 
                    debug!("Result of mk_or = {result:?}");
                    result
                });
                debug!("Case And\nvalues={values:?}\nx:{x:?}\nResult: {result:?}");
                result */
                // todo!()
            }

            Rbe::Or { ref values } => {
                todo!()
                // Self::mk_or_values(values.into_iter().map(|rbe| { (*rbe).clone() } ))
            },
            Rbe::Plus { ref value } => {
                let d = value.deriv(x, n, open, controlled);
                Self::mk_and(&d, &Rbe::Star { value: value.clone() })
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
            Rbe::Repeat { ref value, ref card } => {
                todo!()
            }
            Rbe::Star { ref value } => {
                let d = value.deriv(x, n, open, controlled);
                Self::mk_and(&d, &value)
            }
        }
    }

    fn mk_first(v: &Rbe<A>) -> Rbe<A> {
        (*v).clone()
    }

    fn deriv_and<'a>(v1: &'a Box<Rbe<A>>, v2: &'a Box<Rbe<A>>) -> &'a Box<Rbe<A>> {
        todo!()
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
                values: vec![Box::new((*v1).clone()),Box::new((*v2).clone())] 
             },
        }
    }

    fn mk_or_values<I>(values: I) -> Rbe<A> 
    where I: IntoIterator<Item = Rbe<A>> {
        let init = Rbe::Fail { error: RbeError::OrValuesFail };
        let result = values.into_iter().fold(init, |result, value| {
            Self::mk_or(&result, &value)
        });
        result
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
                        values: vec![
                            Box::new((*v1).clone()),
                            Box::new((*v2).clone())]
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

impl <A> Debug for Rbe<A> 
where A: Debug + Hash + Eq {
    fn fmt(&self, dest: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Rbe::Fail { error } => write!(dest,"Fail {{{error:?}}}"),
            Rbe::Empty => write!(dest,"Empty"),
            Rbe::Symbol { value, card } => write!(dest,"{value:?}{card:?}"),
            Rbe::And { values} => {
                values.iter().fold(Ok(()), |result,value| {
                    result.and_then(|_| write!(dest, "{value:?};"))
                })
            },
            Rbe::Or { values } => {
                values.iter().fold(Ok(()), |result,value| {
                    result.and_then(|_| write!(dest, "{value:?}|"))
                })
            }
            Rbe::Star { value } => write!(dest,"{value:?}*"),
            Rbe::Plus { value } => write!(dest,"{value:?}+"),
            Rbe::Repeat { value, card } => write!(dest,"({value:?}){card:?}"),
        }
    }
}

impl <A> Display for Rbe<A> 
where A: Display + Hash + Eq {
    fn fmt(&self, dest: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Rbe::Fail { error } => write!(dest,"Fail {{{error}}}"),
            Rbe::Empty => write!(dest,"Empty"),
            Rbe::Symbol { value, card } => write!(dest,"{value}{card}"),
            Rbe::And { values } => {
                values.iter().fold(Ok(()), |result,value| {
                    result.and_then(|_| write!(dest, "{value};"))
                })
            },
            Rbe::Or { values } => {
                values.iter().fold(Ok(()), |result,value| {
                    result.and_then(|_| write!(dest, "{value}|"))
                })
            },
            Rbe::Star { value } => write!(dest,"{value}*"),
            Rbe::Plus { value } => write!(dest,"{value}+"),
            Rbe::Repeat { value, card } => write!(dest,"({value}){card}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test_log::test]
    fn deriv_a_1_1_and_b_opt_with_a() {
        // a?|b? #= b/2
        let rbe = Rbe::and(
            vec![
                Rbe::symbol('a', 1, Max::IntMax(1)),
                Rbe::symbol('b', 0, Max::IntMax(1))]
        );
        let expected = Rbe::and(
            vec![
                Rbe::symbol('a', 0, Max::IntMax(0)),
                Rbe::symbol('b', 0, Max::IntMax(1))]
            );
        debug!("Before assert!");
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
        let rbe = Rbe::and(
            vec![
                Rbe::symbol('x', 1, Max::IntMax(1)),
                Rbe::symbol('y', 1, Max::IntMax(1))]
        );
        let expected = HashSet::from(['x', 'y']);
        assert_eq!(rbe.symbols(), expected);
    }

    #[test]
    fn symbols2() {
        let rbe = Rbe::and(
            vec![Rbe::or(
                vec![
                    Rbe::symbol('x', 1, Max::IntMax(1)),
                    Rbe::symbol('y', 2, Max::Unbounded)
                    ]),
            Rbe::symbol('y', 1, Max::IntMax(1))
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
        let rbe = Rbe::or(
            vec![
                Rbe::symbol('a', 0, Max::IntMax(1)),
                Rbe::symbol('b', 0, Max::IntMax(1))]
        );
        assert_eq!(rbe.match_bag(&Bag::from(['a']), false), Ok(()));
    }

    #[test]
    fn match_bag_a_opt_or_b_opt_with_b() {
        // a?|b? #= a
        let rbe = Rbe::or(
            vec![
                Rbe::symbol('a', 0, Max::IntMax(1)),
                Rbe::symbol('b', 0, Max::IntMax(1))]
        );
        assert_eq!(rbe.match_bag(&Bag::from(['b']), false), Ok(()));
    }

    #[test]
    fn match_bag_a_opt_and_b_opt_with_ba() {
        // a?|b? #= a
        let rbe = Rbe::and(
            vec![
                Rbe::symbol('a', 0, Max::IntMax(1)),
                Rbe::symbol('b', 0, Max::IntMax(1))]
        );
        assert_eq!(rbe.match_bag(&Bag::from(['b', 'a']), false), Ok(()));
    }

    #[test]
    fn match_bag_a_and_b_opt_with_ab() {
        // a?|b? #= b/2
        let rbe = Rbe::and(
            vec![Rbe::symbol('a', 1, Max::IntMax(1)),
            Rbe::symbol('b', 0, Max::IntMax(1))]
        );
        debug!("Before assert!");
        assert_eq!(rbe.match_bag(&Bag::from(['a', 'b']), false), Ok(()));
    }

    #[test]
    fn no_match_bag_a_and_b_opt_with_b_2() {
        // a?|b? #= b/2
        let rbe = Rbe::and(
            vec![Rbe::symbol('a', 1, Max::IntMax(1)),
            Rbe::symbol('b', 0, Max::IntMax(1))]
        );
        assert!(rbe.match_bag(&Bag::from(['b', 'b']), false).is_err());
    }

    #[test]
    fn no_match_bag_a_and_b_opt_with_c() {
        // a?|b? #= a
        let rbe = Rbe::and(
            vec![Rbe::symbol('a', 1, Max::IntMax(1)),
            Rbe::symbol('b', 1, Max::IntMax(1))]
        );
        assert!(rbe.match_bag(&Bag::from(['c']), false).is_err());
    }

    #[test]
    fn test_serialize_rbe() {
        
        let rbe = Rbe::symbol("foo".to_string(),1,Max::IntMax(2));
        let expected = 
r#"!Symbol
value: foo
card:
  min: 1
  max: !IntMax 2
"#;
        let rbe: String = serde_yaml::to_string(&rbe).unwrap();
        assert_eq!(rbe, expected);
    } 

    #[test]
    fn test_deserialize_rbe() {
        let str = r#"{ 
            "Symbol": { 
                "value": "foo", 
                "card": {"min": 1, "max": { "IntMax": 2}} 
            }
        }"#;
        let expected = Rbe::symbol("foo".to_string(),1,Max::IntMax(2));
        let rbe: Rbe<String> = serde_json::from_str(str).unwrap();
        assert_eq!(rbe, expected);
    }


}

