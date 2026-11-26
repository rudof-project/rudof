//! A sound refutation test over partially-determined bags for regular bag expressions.
//!
//! A *partial bag* gives every symbol an interval `[lo, hi]`: `lo` occurrences are already
//! committed and at most `hi` are attainable. [`Rbe::feasible`] returns `false` only if *no*
//! bag between `lo` and `hi` (pointwise) is accepted by the expression: it is a cheap
//! (linear in the expression) *necessary* condition, not a decision procedure — bags that
//! pass must still be verified by the exact matcher.
//!
//! The predicate is computed in two modes. In *exact* mode the sub-expression must match
//! exactly once, so `Or` alternatives are exclusive (the unchosen branches must be
//! committable to zero) and symbol upper bounds apply. In *iterated* mode (under `Star`,
//! `Plus` or a `Repeat`) only the monotone consequences survive summation over iterations:
//! `Or` branches may mix, upper bounds vanish, but `And` co-occurrence survives at the
//! occupancy level — if one symbol of a group is occupied, its non-nullable siblings must
//! be occupiable. What it deliberately ignores: count coupling between symbols of a
//! repeated group (`(a b)+` with unequal counts passes) and divisibility.
//!
//! This is a port of the feasibility layer described in `docs/src/internals/feasibility-model.md`
//! (§1 tables and soundness proof), originally implemented and proved for Apache Jena.

use crate::rbe_struct::RbeStruct;
use crate::{Max, Rbe};
use core::hash::Hash;
use std::collections::HashMap;
use std::fmt::{Debug, Display};

impl<A> RbeStruct<A>
where
    A: Hash + Eq + Display + Clone + Debug,
{
    /// See [`Rbe::feasible`].
    pub fn feasible(&self, lo: &HashMap<A, usize>, hi: &HashMap<A, usize>) -> bool {
        self.rbe().feasible(lo, hi)
    }
}

impl<A> Rbe<A>
where
    A: Hash + Eq + Display + Clone + Debug,
{
    /// Whether some bag `c` with `lo <= c <= hi` (pointwise; missing entries are 0) could
    /// be accepted by this expression. `false` is definitive: no such bag is accepted.
    /// `true` is not: the exact matcher decides complete bags.
    pub fn feasible(&self, lo: &HashMap<A, usize>, hi: &HashMap<A, usize>) -> bool {
        Feasibility { lo, hi }.fx(self)
    }
}

struct Feasibility<'a, A>
where
    A: Hash + Eq + Display + Clone + Debug,
{
    lo: &'a HashMap<A, usize>,
    hi: &'a HashMap<A, usize>,
}

impl<A> Feasibility<'_, A>
where
    A: Hash + Eq + Display + Clone + Debug,
{
    fn lo(&self, a: &A) -> usize {
        *self.lo.get(a).unwrap_or(&0)
    }

    fn hi(&self, a: &A) -> usize {
        *self.hi.get(a).unwrap_or(&0)
    }

    /// The sub-expression's slice can be committed to all-zero: nothing assigned yet.
    fn zero(&self, e: &Rbe<A>) -> bool {
        match e {
            Rbe::Fail { .. } | Rbe::Empty => true,
            Rbe::Symbol { value, .. } => self.lo(value) == 0,
            Rbe::And { values } | Rbe::Or { values } => values.iter().all(|v| self.zero(v)),
            Rbe::Star { value } | Rbe::Plus { value } | Rbe::Repeat { value, .. } => self.zero(value),
        }
    }

    /// Necessary condition for some completion slice to be accepted exactly once.
    fn fx(&self, e: &Rbe<A>) -> bool {
        match e {
            Rbe::Fail { .. } => false,
            Rbe::Empty => true,
            Rbe::Symbol { value, card } => {
                let lo_ok = match card.max {
                    Max::Unbounded => true,
                    Max::IntMax(n) => self.lo(value) <= n,
                };
                lo_ok && self.hi(value) >= card.min.value
            },
            Rbe::And { values } => values.iter().all(|v| self.fx(v)),
            Rbe::Or { values } => values
                .iter()
                .enumerate()
                .any(|(i, v)| self.fx(v) && values.iter().enumerate().all(|(j, other)| j == i || self.zero(other))),
            Rbe::Star { value } => self.zero(value) || (self.fi(value) && self.once(value)),
            Rbe::Plus { value } => self.fi(value) && self.once(value),
            Rbe::Repeat { value, card } => {
                if card.max == Max::IntMax(0) {
                    self.zero(value)
                } else if card.min.value == 0 && card.max == Max::IntMax(1) {
                    self.zero(value) || self.fx(value)
                } else if card.min.value == 0 {
                    self.zero(value) || (self.fi(value) && self.once(value))
                } else {
                    // min >= 1: any accepted slice is a sum of >= 1 iteration bags
                    self.fi(value) && self.once(value)
                }
            },
        }
    }

    /// Necessary condition for some completion slice to be a sum of `q >= 1` bags each
    /// accepted by the sub-expression: the monotone weakening of `fx`.
    fn fi(&self, e: &Rbe<A>) -> bool {
        match e {
            Rbe::Fail { .. } => false,
            Rbe::Empty => true,
            Rbe::Symbol { value, card } => {
                (card.min.value == 0 || self.hi(value) >= card.min.value)
                    && (card.max != Max::IntMax(0) || self.lo(value) == 0)
            },
            Rbe::And { values } => values.iter().all(|v| self.fi(v)),
            // Iterations may choose different branches: no exclusivity, but a branch with
            // committed occurrences must itself be iterable.
            Rbe::Or { values } => values.iter().all(|v| self.zero(v) || self.fi(v)),
            Rbe::Star { value } => self.zero(value) || self.fi(value),
            Rbe::Plus { value } => self.fi(value) && self.once(value),
            Rbe::Repeat { value, card } => {
                if card.max == Max::IntMax(0) {
                    self.zero(value)
                } else if card.min.value == 0 {
                    self.zero(value) || self.fi(value)
                } else {
                    self.fi(value) && self.once(value)
                }
            },
        }
    }

    /// Necessary condition for a single non-empty iteration to fit within `hi`.
    fn once(&self, e: &Rbe<A>) -> bool {
        match e {
            Rbe::Fail { .. } => false,
            Rbe::Empty => true,
            Rbe::Symbol { value, card } => card.min.value == 0 || self.hi(value) >= card.min.value,
            Rbe::And { values } => values.iter().all(|v| self.once(v)),
            Rbe::Or { values } => values.iter().any(|v| self.once(v)),
            Rbe::Star { value: _ } => true,
            Rbe::Plus { value } => self.once(value),
            Rbe::Repeat { value, card } => card.min.value == 0 || self.once(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Cardinality, Max, Rbe};
    use std::collections::HashMap;

    fn sym(c: char, min: usize, max: Max) -> Rbe<char> {
        Rbe::Symbol {
            value: c,
            card: Cardinality::from(crate::Min::from(min), max),
        }
    }

    fn counts(pairs: &[(char, usize)]) -> HashMap<char, usize> {
        pairs.iter().cloned().collect()
    }

    // { a{1,2} | a+ ; q } — the shared-predicate OneOf screw case: committing an
    // occurrence to the second branch's `a` requires an occupiable `q`.
    fn blowup() -> Rbe<char> {
        Rbe::Or {
            values: vec![
                sym('a', 1, Max::IntMax(2)),
                Rbe::And {
                    values: vec![sym('b', 1, Max::Unbounded), sym('q', 1, Max::IntMax(1))],
                },
            ],
        }
    }

    #[test]
    fn or_branch_refuted_without_cooccurring_symbol() {
        // One occurrence committed to the second branch's 'b', but no 'q' is attainable:
        // branch 2 fails (hi(q) = 0), branch 1 fails (branch 2 not committable to zero).
        assert!(!blowup().feasible(&counts(&[('b', 1)]), &counts(&[('b', 3), ('q', 0)])));
        // With one attainable 'q' the same commitment is feasible.
        assert!(blowup().feasible(&counts(&[('b', 1)]), &counts(&[('b', 3), ('q', 1)])));
    }

    #[test]
    fn exact_upper_bound_refutes() {
        // Everything on branch 1: lo(a) = 3 > 2 refutes it; branch 2 has lo(a)=... zero.
        assert!(!blowup().feasible(&counts(&[('a', 3)]), &counts(&[('a', 3)])));
        assert!(blowup().feasible(&counts(&[('a', 2)]), &counts(&[('a', 2)])));
    }

    #[test]
    fn mandatory_symbol_needs_candidates() {
        let e = Rbe::And {
            values: vec![sym('a', 1, Max::IntMax(1)), sym('b', 1, Max::IntMax(1))],
        };
        assert!(!e.feasible(&HashMap::new(), &counts(&[('a', 1)])));
        assert!(e.feasible(&HashMap::new(), &counts(&[('a', 1), ('b', 1)])));
    }

    #[test]
    fn or_exclusivity_does_not_survive_repetition() {
        // (a | b) rejects a mixed commitment; (a | b)+ accepts it.
        let a_or_b = Rbe::Or {
            values: vec![sym('a', 1, Max::IntMax(1)), sym('b', 1, Max::IntMax(1))],
        };
        let mixed_lo = counts(&[('a', 1), ('b', 1)]);
        let mixed_hi = counts(&[('a', 1), ('b', 1)]);
        assert!(!a_or_b.feasible(&mixed_lo, &mixed_hi));
        let plus = Rbe::Plus {
            value: Box::new(a_or_b),
        };
        assert!(plus.feasible(&mixed_lo, &mixed_hi));
    }

    #[test]
    fn count_coupling_is_deliberately_ignored() {
        // (a ; b)+ requires #a = #b, but the occupancy abstraction accepts (2,1):
        // the exact matcher remains the decider for surviving bags.
        let e = Rbe::Plus {
            value: Box::new(Rbe::And {
                values: vec![sym('a', 1, Max::IntMax(1)), sym('b', 1, Max::IntMax(1))],
            }),
        };
        assert!(e.feasible(&counts(&[('a', 2), ('b', 1)]), &counts(&[('a', 2), ('b', 1)])));
    }
}
