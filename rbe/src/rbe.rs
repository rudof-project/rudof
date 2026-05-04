use crate::{Bag, Cardinality, Max, Min, deriv_error::DerivError, deriv_n};
use crate::{Interval, RbePrettyPrinter};
use core::hash::Hash;
use itertools::cloned;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt;
use std::fmt::{Debug, Display};
// use tracing::trace;

/// Simple Implementation of Regular Bag Expressions
/// The implementation is based on [Brzozowski derivatives of regular expressions](https://dl.acm.org/doi/10.1145/321239.321249),
/// adapted to bags and cardinalities.
///
/// The main idea is that we can compute the derivative of a regular bag expression with respect to a symbol, and this derivative will represent the remaining expression after
/// consuming that symbol. By iterating this process for all symbols in a bag, we can determine if the bag matches the original expression by checking if the final derivative
/// is nullable (i.e., can match the empty bag).
///
/// The Rbe enum represents the different types of regular bag expressions, including failure cases, empty expressions, symbols with cardinalities, conjunctions (And), disjunctions (Or),
/// and repetitions (Star, Plus, Repeat).
///
/// The match_bag method uses the deriv_bag method to compute the derivative of the expression with respect to the input bag and checks if the resulting expression is nullable
/// to determine if the match is successful. The implementation also includes error handling through the DerivError enum, which captures various failure scenarios during the derivative computation.
///
/// This implementation allows for efficient matching of bags against complex regular bag expressions, leveraging the power of derivatives to handle the combinatorial nature of the problem.
/// The Rbe struct is designed to be flexible and extensible, allowing for various operations such as mapping over symbols, pretty-printing, and more. The use of generics allows it to work with any type of symbol that implements the necessary traits (Hash, Eq, Display). Overall, this implementation provides a robust foundation for working with regular bag expressions in Rust.
///
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
    pub fn match_bag_deriv(&self, bag: &Bag<A>, open: bool) -> Result<(), DerivError<A>> {
        tracing::trace!("Matching bag {bag} against RBE {self} with open={open}");
        let deriv = self.deriv_bag(bag, open, &self.symbols());
        tracing::trace!("Deriv of RBE {self} with bag {bag} and open={open} is {deriv}");
        match &deriv {
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
            },
        }
    }

    pub fn has_repeated_symbols(&self) -> bool {
        match self {
            Rbe::Fail { .. } => false,
            Rbe::Empty => false,
            Rbe::Symbol { .. } => false,
            Rbe::And { values } | Rbe::Or { values } => values.iter().any(|v| v.has_repeated_symbols()),
            Rbe::Star { value } | Rbe::Plus { value } => value.has_repeated_symbols(),
            Rbe::Repeat { .. } => true,
        }
    }

    pub fn is_fail(&self) -> bool {
        matches!(self, Rbe::Fail { .. })
    }

    pub fn interval(&self, bag: &Bag<A>) -> Interval {
        // trace!("Computing interval of RBE {self} with bag {bag}");
        match self {
            Rbe::Fail { error: _ } => Interval::fail(),
            Rbe::Empty => Interval::zero_any(),
            Rbe::Symbol { value, card } => {
                let wa = bag.contains(value);
                let n = Max::IntMax(card.min.value);
                let int = Interval::new(card.max.div_up(&wa), n.div_down(&wa));
                // trace!("Symbol {value} with cardinality {card} and bag {bag} has interval {int}");
                int
            },
            Rbe::And { values } => {
                let and = values
                    .iter()
                    .fold(Interval::zero_any(), |acc, v| acc.intersection(&v.interval(bag)));
                // trace!("And {self} with bag {bag} is {and}");
                and
            },
            Rbe::Or { values } => {
                // Minkowski sum: every branch must have a valid scale factor.
                // If any branch interval is empty the whole Or is unsatisfiable.
                let or = values.iter().fold(Interval::zero_zero(), |acc, v| {
                    if acc.is_empty() {
                        acc
                    } else {
                        let iv = v.interval(bag);
                        if iv.is_empty() { Interval::fail() } else { acc.addition(&iv) }
                    }
                });
                // trace!("Or {self} with bag {bag} is {or}");
                or
            },
            Rbe::Star { value } => {
                if self.no_symbols_in_bag(bag) {
                    Interval::zero_any()
                } else {
                    let interval = value.interval(bag);
                    if interval.is_empty() {
                        interval
                    } else {
                        Interval::one_any()
                    }
                }
            },
            Rbe::Plus { value } => {
                if self.no_symbols_in_bag(bag) {
                    // A single nullable repetition satisfies Plus with an empty bag.
                    if value.nullable() { Interval::zero_any() } else { Interval::zero_zero() }
                } else {
                    let interval = value.interval(bag);
                    if interval.is_empty() {
                        interval
                    } else {
                        Interval::new(Max::IntMax(1), interval.m)
                    }
                }
            },
            Rbe::Repeat { value: _, card: _ } => {
                // Having repetitions on expressions breaks the single-occurrence bag expression
                // This case should be handled by detecting repetitions and invoking the derivatives algorithm
                todo!()
            },
        }
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
            },
            Rbe::And { values } => {
                values.iter().for_each(|v| v.symbols_aux(set));
            },
            Rbe::Or { values } => {
                values.iter().for_each(|v| v.symbols_aux(set));
            },
            Rbe::Plus { value } => {
                value.symbols_aux(set);
            },
            Rbe::Star { value } => {
                value.symbols_aux(set);
            },
            Rbe::Repeat { value, card: _ } => {
                value.symbols_aux(set);
            },
        }
    }

    fn no_symbols_in_bag(&self, bag: &Bag<A>) -> bool {
        self.symbols().iter().all(|symbol| bag.contains(symbol) == 0)
    }

    pub fn deriv_bag(&self, bag: &Bag<A>, open: bool, controlled: &HashSet<A>) -> Rbe<A> {
        let mut current = (*self).clone();
        let mut processed = Bag::new();
        for (x, count) in bag.iter() {
            for _ in 0..count {
                let deriv = current.deriv(x, 1, open, controlled);
                // trace!("Deriv of RBE {current} with symbol {x} and open={open} is {deriv}");
                match deriv {
                    Rbe::Fail { error } => {
                        current = Rbe::Fail {
                            error: DerivError::DerivBagError {
                                error_msg: format!("{error}"),
                                processed: Box::new(processed.clone()),
                                bag: Box::new((*bag).clone()),
                                expr: Box::new((*self).clone()),
                                current: Box::new(current.clone()),
                                value: (*x).clone(),
                                open,
                            },
                        };
                        break;
                    },
                    _ => {
                        processed.insert((*x).clone());
                        current = deriv;
                    },
                }
            }
            if matches!(current, Rbe::Fail { .. }) {
                break;
            }
        }
        current
    }

    pub fn nullable(&self) -> NullableResult {
        match &self {
            Rbe::Fail { .. } => false,
            Rbe::Empty => true,
            Rbe::Symbol { card, .. } if card.nullable() => true,
            Rbe::Symbol { .. } => false,
            Rbe::And { values } => values.iter().all(|v| v.nullable()),
            Rbe::Or { values } => values.iter().any(|v| v.nullable()),
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
                        error: DerivError::UnexpectedEmpty { x: (*x).clone(), open },
                    }
                }
            },
            Rbe::Symbol { ref value, ref card } => {
                if *x == *value {
                    if card.max == Max::IntMax(0) {
                        Rbe::Fail {
                            error: DerivError::MaxCardinalityZeroFoundValue { x: (*x).clone() },
                        }
                    } else if card.max.greater_or_equal(n) {
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
            },
            Rbe::And { ref values } => Self::deriv_and(values, x, n, open, controlled),
            Rbe::Or { ref values } => Self::mk_or_values(values.iter().map(|rbe| rbe.deriv(x, n, open, controlled))),
            Rbe::Plus { ref value } => {
                let d = value.deriv(x, n, open, controlled);
                Self::mk_and(&d, &Rbe::Star { value: value.clone() })
            },
            Rbe::Repeat { ref value, ref card } if card.is_0_0() => {
                let d = value.deriv(x, n, open, controlled);
                if d.nullable() {
                    Rbe::Fail {
                        error: DerivError::CardinalityZeroZeroDeriv { symbol: x.clone() },
                    }
                } else {
                    Rbe::Empty
                }
            },
            Rbe::Repeat { ref value, ref card } => {
                let d = value.deriv(x, n, open, controlled);
                let card = card.minus(n);
                let rest = Self::mk_range(value, &card);
                Self::mk_and(&d, &rest)
            },
            Rbe::Star { ref value } => {
                let d = value.deriv(x, n, open, controlled);
                Self::mk_and(&d, &Rbe::Star { value: value.clone() })
            },
        }
    }

    fn deriv_and(values: &Vec<Rbe<A>>, x: &A, n: usize, open: bool, controlled: &HashSet<A>) -> Rbe<A> {
        let mut or_values: Vec<Rbe<A>> = Vec::new();
        let mut failures = crate::deriv_error::Failures::new();

        for vs in deriv_n(cloned((*values).iter()).collect(), |value: &Rbe<A>| {
            let d = value.deriv(x, n, open, controlled);
            match d {
                Rbe::Fail { error } => {
                    failures.push(value.clone(), error);
                    None
                },
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
            },
        }
    }

    fn bigger(min: Min, max: &Max) -> bool {
        match max {
            Max::Unbounded => false,
            Max::IntMax(max) => min.value > *max,
        }
    }

    pub fn map<B>(&self, f: &dyn Fn(&A) -> B) -> Rbe<B>
    where
        B: Hash + Eq + Display,
    {
        match &self {
            Rbe::Fail { error } => panic!("Cannot map over Fail: {error}"),
            Rbe::Empty => Rbe::Empty,
            Rbe::Symbol { value, card } => Rbe::Symbol {
                value: f(value),
                card: card.clone(),
            },
            Rbe::And { values } => Rbe::And {
                values: values.iter().map(|v| v.map(f)).collect(),
            },
            Rbe::Or { values } => Rbe::Or {
                values: values.iter().map(|v| v.map(f)).collect(),
            },
            Rbe::Star { value } => Rbe::Star {
                value: Box::new(value.map(f)),
            },
            Rbe::Plus { value } => Rbe::Plus {
                value: Box::new(value.map(f)),
            },
            Rbe::Repeat { value, card } => Rbe::Repeat {
                value: Box::new(value.map(f)),
                card: card.clone(),
            },
        }
    }

    pub fn pretty(&self, width: usize) -> String {
        let pretty_printer = RbePrettyPrinter::new();

        pretty_printer.print(self, width)
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
            Rbe::And { values } => {
                let parts: Vec<String> = values
                    .iter()
                    .map(|v| {
                        if matches!(v, Rbe::Or { .. }) {
                            format!("({v:?})")
                        } else {
                            format!("{v:?}")
                        }
                    })
                    .collect();
                write!(dest, "({})", parts.join(";"))
            },
            Rbe::Or { values } => {
                write!(
                    dest,
                    "({})",
                    values.iter().map(|v| format!("{v:?}")).collect::<Vec<_>>().join("|")
                )
            },
            Rbe::Star { value } => write!(dest, "({value:?})*"),
            Rbe::Plus { value } => write!(dest, "({value:?})+"),
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
            Rbe::And { values } => {
                let parts: Vec<String> = values
                    .iter()
                    .map(|v| {
                        if matches!(v, Rbe::Or { .. }) {
                            format!("({v})")
                        } else {
                            format!("{v}")
                        }
                    })
                    .collect();
                write!(dest, "{}", parts.join(";"))
            },
            Rbe::Or { values } => {
                write!(
                    dest,
                    "{}",
                    values.iter().map(|v| format!("{v}")).collect::<Vec<_>>().join("|")
                )
            },
            Rbe::Star { value } => write!(dest, "({value})*"),
            Rbe::Plus { value } => write!(dest, "({value})+"),
            Rbe::Repeat { value, card } => write!(dest, "({value}){card}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RbeStruct;
    use tracing_test::traced_test;
    // use indoc::indoc;
    // use test_log::test;

    fn sym<A>(x: A, min: usize, max: Max) -> Rbe<A>
    where
        A: Hash + Eq + Display + Clone + Debug,
    {
        RbeStruct::symbol(x, min, max).inner_rbe().clone()
    }

    fn rbe_and<A>(vs: Vec<Rbe<A>>) -> Rbe<A>
    where
        A: Hash + Eq + Display + Clone + Debug,
    {
        Rbe::And { values: vs }
    }

    fn rbe_or<A>(vs: Vec<Rbe<A>>) -> Rbe<A>
    where
        A: Hash + Eq + Display + Clone + Debug,
    {
        Rbe::Or { values: vs }
    }

    fn rbe_star<A>(v: Rbe<A>) -> Rbe<A>
    where
        A: Hash + Eq + Display,
    {
        Rbe::Star { value: Box::new(v) }
    }

    fn rbe_plus<A>(v: Rbe<A>) -> Rbe<A>
    where
        A: Hash + Eq + Display,
    {
        Rbe::Plus { value: Box::new(v) }
    }

    #[test]
    fn deriv_a_1_1_and_b_opt_with_a() {
        // a?|b? #= b/2
        let rbe = rbe_and(vec![sym('a', 1, Max::IntMax(1)), sym('b', 0, Max::IntMax(1))]);
        let expected = rbe_and(vec![sym('a', 0, Max::IntMax(0)), sym('b', 0, Max::IntMax(1))]);
        assert_eq!(rbe.deriv(&'a', 1, false, &HashSet::from(['a', 'b'])), expected);
    }

    #[test]
    fn deriv_symbol() {
        let rbe = sym('x', 1, Max::IntMax(1));
        let d = rbe.deriv(&'x', 1, true, &HashSet::new());
        assert_eq!(d, sym('x', 0, Max::IntMax(0)));
    }

    #[test]
    fn symbols() {
        let rbe = rbe_and(vec![sym('x', 1, Max::IntMax(1)), sym('y', 1, Max::IntMax(1))]);
        let expected = HashSet::from(['x', 'y']);
        assert_eq!(rbe.symbols(), expected);
    }

    #[test]
    fn symbols2() {
        let rbe = rbe_and(vec![
            rbe_or(vec![sym('x', 1, Max::IntMax(1)), sym('y', 2, Max::Unbounded)]),
            sym('y', 1, Max::IntMax(1)),
        ]);
        let expected = HashSet::from(['x', 'y']);
        assert_eq!(rbe.symbols(), expected);
    }

    #[test]
    fn match_bag_y1_4_y_2() {
        let rbe = sym('y', 1, Max::IntMax(4));
        assert_eq!(rbe.match_bag_deriv(&Bag::from(['y', 'y']), false), Ok(()));
    }

    #[test]
    fn match_bag_a_opt_or_b_opt_with_a() {
        let rbe = rbe_or(vec![sym('a', 0, Max::IntMax(1)), sym('b', 0, Max::IntMax(1))]);
        assert_eq!(rbe.match_bag_deriv(&Bag::from(['a']), false), Ok(()));
    }

    #[test]
    fn match_bag_a_opt_or_b_opt_with_b() {
        let rbe = rbe_or(vec![sym('a', 0, Max::IntMax(1)), sym('b', 0, Max::IntMax(1))]);
        assert_eq!(rbe.match_bag_deriv(&Bag::from(['b']), false), Ok(()));
    }

    #[test]
    fn match_bag_a_opt_and_b_opt_with_ba() {
        let rbe = rbe_and(vec![sym('a', 0, Max::IntMax(1)), sym('b', 0, Max::IntMax(1))]);
        assert_eq!(rbe.match_bag_deriv(&Bag::from(['b', 'a']), false), Ok(()));
    }

    #[test]
    fn match_bag_a_and_b_opt_with_ab() {
        let rbe = rbe_and(vec![sym('a', 1, Max::IntMax(1)), sym('b', 0, Max::IntMax(1))]);
        assert_eq!(rbe.match_bag_deriv(&Bag::from(['a', 'b']), false), Ok(()));
    }

    #[test]
    fn no_match_bag_a_and_b_opt_with_b_2() {
        let rbe = rbe_and(vec![sym('a', 1, Max::IntMax(1)), sym('b', 0, Max::IntMax(1))]);
        assert!(rbe.match_bag_deriv(&Bag::from(['b', 'b']), false).is_err());
    }

    #[test]
    fn no_match_bag_a_and_b_opt_with_c() {
        let rbe = rbe_and(vec![sym('a', 1, Max::IntMax(1)), sym('b', 1, Max::IntMax(1))]);
        assert!(rbe.match_bag_deriv(&Bag::from(['c']), false).is_err());
    }

    #[traced_test]
    #[test]
    fn match_bag_a_and_b_star_with_a_b() {
        let rbe = rbe_star(rbe_and(vec![sym('a', 1, Max::IntMax(1)), sym('b', 1, Max::IntMax(1))]));
        assert_eq!(rbe.match_bag_deriv(&Bag::from(['a', 'b']), true), Ok(()));
    }

    #[traced_test]
    #[test]
    fn no_match_bag_a_and_b_star_with_a() {
        let rbe = rbe_star(rbe_and(vec![sym('a', 1, Max::IntMax(1)), sym('b', 1, Max::IntMax(1))]));
        assert!(rbe.match_bag_deriv(&Bag::from(['a']), true).is_err());
    }

    #[traced_test]
    #[test]
    fn no_match_bag_a_and_b_star_with_b() {
        let rbe = rbe_star(rbe_and(vec![sym('a', 1, Max::IntMax(1)), sym('b', 1, Max::IntMax(1))]));
        assert!(rbe.match_bag_deriv(&Bag::from(['b']), true).is_err());
    }

    #[traced_test]
    #[test]
    fn match_bag_a_and_b_plus_with_a_b() {
        let rbe = rbe_plus(rbe_and(vec![sym('a', 1, Max::IntMax(1)), sym('b', 1, Max::IntMax(1))]));
        assert_eq!(rbe.match_bag_deriv(&Bag::from(['a', 'b']), true), Ok(()));
    }

    #[traced_test]
    #[test]
    fn match_group_a_and_b_star_or_c_with_a2_b2() {
        let rbe = rbe_or(vec![
            rbe_star(rbe_and(vec![sym('a', 1, Max::IntMax(1)), sym('b', 1, Max::IntMax(1))])),
            sym('c', 1, Max::IntMax(1)),
        ]);
        assert_eq!(rbe.match_bag_deriv(&Bag::from(['a', 'a', 'b', 'b']), false), Ok(()));
    }

    #[traced_test]
    #[test]
    fn match_group_a_and_b_star_or_c_with_a_b() {
        let rbe = rbe_or(vec![
            rbe_star(rbe_and(vec![sym('a', 1, Max::IntMax(1)), sym('b', 1, Max::IntMax(1))])),
            sym('c', 1, Max::IntMax(1)),
        ]);
        assert_eq!(rbe.match_bag_deriv(&Bag::from(['a', 'b']), false), Ok(()));
    }

    #[traced_test]
    #[test]
    fn match_a_25_with_a_2() {
        let rbe = sym('a', 2, Max::IntMax(5));
        assert_eq!(rbe.match_bag_deriv(&Bag::from(['a', 'a']), false), Ok(()));
    }

    /* I comment this test because it fails with
       Result::unwrap()` on an `Err` value: Error { inner: UnsupportedType(Some("Rbe")) }
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
        let rbe: String = toml::to_string(&rbe).unwrap();
        assert_eq!(rbe, expected);
    }*/

    #[test]
    fn display_or_inside_and_adds_parens() {
        let rbe: Rbe<char> = rbe_and(vec![
            rbe_or(vec![sym('a', 1, Max::IntMax(1)), sym('b', 1, Max::IntMax(1))]),
            sym('c', 1, Max::IntMax(1)),
        ]);
        assert_eq!(format!("{rbe}"), "(a|b);c");
    }

    #[test]
    fn display_and_inside_or_no_extra_parens() {
        let rbe: Rbe<char> = rbe_or(vec![
            rbe_and(vec![sym('a', 1, Max::IntMax(1)), sym('b', 1, Max::IntMax(1))]),
            sym('c', 1, Max::IntMax(1)),
        ]);
        assert_eq!(format!("{rbe}"), "a;b|c");
    }

    #[test]
    fn display_nested_or_inside_and_deep() {
        let rbe: Rbe<char> = rbe_and(vec![
            rbe_or(vec![
                rbe_and(vec![sym('a', 1, Max::IntMax(1)), sym('b', 1, Max::IntMax(1))]),
                sym('c', 1, Max::IntMax(1)),
            ]),
            sym('d', 1, Max::IntMax(1)),
        ]);
        assert_eq!(format!("{rbe}"), "(a;b|c);d");
    }

    #[test]
    fn test_deserialize_rbe() {
        let str = r#"{
            "Symbol": {
                "value": "foo",
                "card": {"min": 1, "max": 2 }
            }
        }"#;
        let expected = sym("foo".to_string(), 1, Max::IntMax(2));
        let rbe: Rbe<String> = serde_json::from_str(str).unwrap();
        assert_eq!(rbe, expected);
    }
}
