use crate::{Bag, Cardinality, Max, Min, Rbe, deriv_error::DerivError};
use core::hash::Hash;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashSet;
use std::fmt::{self, Debug, Display};

pub struct RbeStruct<A>
where
    A: Hash + Eq + Display,
{
    rbe: Rbe<A>,
    symbols: HashSet<A>,
    has_repeats: bool,
}

impl<A> RbeStruct<A>
where
    A: Hash + Eq + Display + Clone + Debug,
{
    pub fn symbols(&self) -> &HashSet<A> {
        &self.symbols
    }

    pub fn has_repeats(&self) -> bool {
        self.has_repeats
    }

    pub fn match_bag_interval(&self, bag: &Bag<A>, open: bool) -> Result<(), DerivError<A>> {
        if self.has_repeats || open {
            self.rbe.match_bag_deriv(bag, open)
        } else {
            let extra_symbols = self.extra_symbols(bag);
            if !extra_symbols.is_empty() {
                return Err(DerivError::ExtraSymbolsClosed {
                    extra_symbols: extra_symbols.into_iter().map(|s| s.to_string()).collect(),
                });
            }
            let interval = self.rbe.interval(bag);
            if interval.contains(1) {
                Ok(())
            } else {
                Err(DerivError::IntervalFailed { v: 1, interval })
            }
        }
    }

    /// Returns the set of symbols in the bag that are not in the RBE's symbol set.
    fn extra_symbols(&self, bag: &Bag<A>) -> HashSet<A> {
        bag.iter()
            .filter(|(sym, _)| !self.symbols.contains(sym))
            .map(|(sym, _)| sym.clone())
            .collect()
    }

    pub fn empty() -> RbeStruct<A> {
        RbeStruct {
            rbe: Rbe::Empty,
            has_repeats: false,
            symbols: HashSet::new(),
        }
    }

    pub fn symbol(x: A, min: usize, max: Max) -> RbeStruct<A> {
        RbeStruct {
            rbe: Rbe::Symbol {
                value: x.clone(),
                card: Cardinality {
                    min: Min::from(min),
                    max,
                },
            },
            has_repeats: false,
            symbols: HashSet::from([x]),
        }
    }

    pub fn or<I>(values: I) -> RbeStruct<A>
    where
        I: IntoIterator<Item = RbeStruct<A>>,
    {
        let items: Vec<RbeStruct<A>> = values.into_iter().collect();
        let has_repeats = items.iter().any(|v| v.has_repeats) || Self::has_symbol_overlap(&items);
        let symbols: HashSet<A> = items.iter().flat_map(|v| v.symbols.iter().cloned()).collect();
        let rbe_values: Vec<Rbe<A>> = items.into_iter().map(|v| v.rbe).collect();
        RbeStruct {
            rbe: Rbe::Or { values: rbe_values },
            has_repeats,
            symbols,
        }
    }

    pub fn and<I>(values: I) -> RbeStruct<A>
    where
        I: IntoIterator<Item = RbeStruct<A>>,
    {
        let items: Vec<RbeStruct<A>> = values.into_iter().collect();
        let has_repeats = items.iter().any(|v| v.has_repeats) || Self::has_symbol_overlap(&items);
        let symbols: HashSet<A> = items.iter().flat_map(|v| v.symbols.iter().cloned()).collect();
        let rbe_values: Vec<Rbe<A>> = items.into_iter().map(|v| v.rbe).collect();
        RbeStruct {
            rbe: Rbe::And { values: rbe_values },
            has_repeats,
            symbols,
        }
    }

    pub fn opt(v: RbeStruct<A>) -> RbeStruct<A> {
        RbeStruct {
            rbe: Rbe::Or {
                values: vec![v.rbe, Rbe::Empty],
            },
            has_repeats: v.has_repeats,
            symbols: v.symbols,
        }
    }

    pub fn plus(v: RbeStruct<A>) -> RbeStruct<A> {
        RbeStruct {
            rbe: Rbe::Plus { value: Box::new(v.rbe) },
            has_repeats: v.has_repeats,
            symbols: v.symbols,
        }
    }

    pub fn star(v: RbeStruct<A>) -> RbeStruct<A> {
        RbeStruct {
            rbe: Rbe::Star { value: Box::new(v.rbe) },
            has_repeats: v.has_repeats,
            symbols: v.symbols,
        }
    }

    pub fn repeat(v: RbeStruct<A>, min: usize, max: Max) -> RbeStruct<A> {
        RbeStruct {
            rbe: Rbe::Repeat {
                value: Box::new(v.rbe),
                card: Cardinality::from(Min::from(min), max),
            },
            has_repeats: v.has_repeats,
            symbols: v.symbols,
        }
    }

    pub fn inner_rbe(&self) -> &Rbe<A> {
        &self.rbe
    }

    pub fn nullable(&self) -> bool {
        self.rbe.nullable()
    }

    /// Returns `true` if any symbol appears in more than one item's symbol set.
    fn has_symbol_overlap(items: &[RbeStruct<A>]) -> bool {
        let mut seen: HashSet<&A> = HashSet::new();
        for item in items {
            for sym in &item.symbols {
                if !seen.insert(sym) {
                    return true;
                }
            }
        }
        false
    }
}

impl<A> Clone for RbeStruct<A>
where
    A: Hash + Eq + Display + Clone + Debug,
{
    fn clone(&self) -> Self {
        RbeStruct {
            rbe: self.rbe.clone(),
            symbols: self.symbols.clone(),
            has_repeats: self.has_repeats,
        }
    }
}

impl<A> PartialEq for RbeStruct<A>
where
    A: Hash + Eq + Display + Clone + Debug,
{
    fn eq(&self, other: &Self) -> bool {
        self.rbe == other.rbe
    }
}

impl<A> Eq for RbeStruct<A> where A: Hash + Eq + Display + Clone + Debug {}

impl<A> Default for RbeStruct<A>
where
    A: Hash + Eq + Display + Clone + Debug,
{
    fn default() -> Self {
        Self::empty()
    }
}

impl<A> Debug for RbeStruct<A>
where
    A: Hash + Eq + Display + Clone + Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.rbe, f)
    }
}

impl<A> Display for RbeStruct<A>
where
    A: Hash + Eq + Display + Clone + Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.rbe, f)
    }
}

impl<A> From<Rbe<A>> for RbeStruct<A>
where
    A: Hash + Eq + Display + Clone + Debug,
{
    fn from(rbe: Rbe<A>) -> Self {
        match rbe {
            Rbe::Empty => Self::empty(),
            Rbe::Symbol { value, card } => RbeStruct {
                symbols: HashSet::from([value.clone()]),
                has_repeats: false,
                rbe: Rbe::Symbol { value, card },
            },
            Rbe::And { values } => Self::and(values.into_iter().map(Self::from)),
            Rbe::Or { values } => Self::or(values.into_iter().map(Self::from)),
            Rbe::Star { value } => Self::star(Self::from(*value)),
            Rbe::Plus { value } => Self::plus(Self::from(*value)),
            Rbe::Repeat { value, card } => {
                let inner = Self::from(*value);
                let min = card.min.value;
                let max = card.max.clone();
                Self::repeat(inner, min, max)
            },
            Rbe::Fail { .. } => Self::empty(),
        }
    }
}

impl<A> Serialize for RbeStruct<A>
where
    A: Hash + Eq + Display + Clone + Debug + Serialize,
{
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.rbe.serialize(serializer)
    }
}

impl<'de, A> Deserialize<'de> for RbeStruct<A>
where
    A: Hash + Eq + Display + Clone + Debug + Deserialize<'de>,
{
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let rbe = Rbe::<A>::deserialize(deserializer)?;
        Ok(Self::from(rbe))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- symbol ---

    #[test]
    fn symbol_no_repeats_and_singleton_symbols() {
        let s = RbeStruct::symbol('a', 1, Max::IntMax(1));
        assert!(!s.has_repeats());
        assert_eq!(s.symbols(), &HashSet::from(['a']));
    }

    // --- empty ---

    #[test]
    fn empty_no_repeats_and_empty_symbols() {
        let e: RbeStruct<char> = RbeStruct::empty();
        assert!(!e.has_repeats());
        assert!(e.symbols().is_empty());
    }

    // --- or ---

    #[test]
    fn or_disjoint_symbols_no_repeats() {
        let r = RbeStruct::or(vec![
            RbeStruct::symbol('a', 1, Max::IntMax(1)),
            RbeStruct::symbol('b', 1, Max::IntMax(1)),
        ]);
        assert!(!r.has_repeats());
        assert_eq!(r.symbols(), &HashSet::from(['a', 'b']));
    }

    #[test]
    fn or_overlapping_symbols_has_repeats() {
        // 'a' appears in both branches → has_repeats must be true
        let r = RbeStruct::or(vec![
            RbeStruct::symbol('a', 1, Max::IntMax(1)),
            RbeStruct::symbol('a', 2, Max::IntMax(3)),
        ]);
        assert!(r.has_repeats());
    }

    #[test]
    fn or_three_branches_partial_overlap_has_repeats() {
        // 'b' appears in branch 1 and branch 3
        let r = RbeStruct::or(vec![
            RbeStruct::symbol('a', 1, Max::IntMax(1)),
            RbeStruct::symbol('b', 1, Max::IntMax(1)),
            RbeStruct::symbol('b', 1, Max::IntMax(2)),
        ]);
        assert!(r.has_repeats());
        assert_eq!(r.symbols(), &HashSet::from(['a', 'b']));
    }

    #[test]
    fn or_propagates_inner_has_repeats() {
        // inner already has repeats; outer adds a disjoint symbol
        let inner = RbeStruct::or(vec![
            RbeStruct::symbol('a', 1, Max::IntMax(1)),
            RbeStruct::symbol('a', 1, Max::IntMax(1)),
        ]);
        let outer = RbeStruct::or(vec![inner, RbeStruct::symbol('b', 1, Max::IntMax(1))]);
        assert!(outer.has_repeats());
    }

    #[test]
    fn or_single_element_no_repeats() {
        let r = RbeStruct::or(vec![RbeStruct::symbol('x', 1, Max::IntMax(1))]);
        assert!(!r.has_repeats());
        assert_eq!(r.symbols(), &HashSet::from(['x']));
    }

    // --- and ---

    #[test]
    fn and_disjoint_symbols_no_repeats() {
        let r = RbeStruct::and(vec![
            RbeStruct::symbol('a', 1, Max::IntMax(1)),
            RbeStruct::symbol('b', 1, Max::IntMax(1)),
        ]);
        assert!(!r.has_repeats());
        assert_eq!(r.symbols(), &HashSet::from(['a', 'b']));
    }

    #[test]
    fn and_overlapping_symbols_has_repeats() {
        // same symbol in both branches of And → both must be satisfied → repeat
        let r = RbeStruct::and(vec![
            RbeStruct::symbol('a', 1, Max::IntMax(1)),
            RbeStruct::symbol('a', 1, Max::IntMax(1)),
        ]);
        assert!(r.has_repeats());
    }

    #[test]
    fn and_three_branches_partial_overlap_has_repeats() {
        let r = RbeStruct::and(vec![
            RbeStruct::symbol('a', 1, Max::IntMax(1)),
            RbeStruct::symbol('b', 1, Max::IntMax(1)),
            RbeStruct::symbol('b', 1, Max::IntMax(2)),
        ]);
        assert!(r.has_repeats());
        assert_eq!(r.symbols(), &HashSet::from(['a', 'b']));
    }

    #[test]
    fn and_propagates_inner_has_repeats() {
        let inner = RbeStruct::and(vec![
            RbeStruct::symbol('a', 1, Max::IntMax(1)),
            RbeStruct::symbol('a', 1, Max::IntMax(1)),
        ]);
        let outer = RbeStruct::and(vec![inner, RbeStruct::symbol('b', 1, Max::IntMax(1))]);
        assert!(outer.has_repeats());
    }

    #[test]
    fn and_single_element_no_repeats() {
        let r = RbeStruct::and(vec![RbeStruct::symbol('x', 1, Max::IntMax(1))]);
        assert!(!r.has_repeats());
        assert_eq!(r.symbols(), &HashSet::from(['x']));
    }

    // --- opt ---

    #[test]
    fn opt_preserves_symbols_and_no_repeats() {
        let r = RbeStruct::opt(RbeStruct::symbol('a', 1, Max::IntMax(1)));
        assert_eq!(r.symbols(), &HashSet::from(['a']));
        assert!(!r.has_repeats());
    }

    #[test]
    fn opt_preserves_inner_has_repeats() {
        let inner = RbeStruct::or(vec![
            RbeStruct::symbol('a', 1, Max::IntMax(1)),
            RbeStruct::symbol('a', 1, Max::IntMax(1)),
        ]);
        let r = RbeStruct::opt(inner);
        assert!(r.has_repeats());
    }
}
