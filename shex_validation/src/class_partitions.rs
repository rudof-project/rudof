//! Class-based, feasibility-pruned enumeration of neighbourhood partitions.
//!
//! Replaces the plain k-partition enumeration of `k_partitions.rs` (k^n bucket assignments
//! guarded only by key membership) with a search organised from cheap to expensive:
//!
//! 1. **Classes**: neighbourhood values with the same *eligible bucket set* (buckets whose
//!    expressions mention the value's key) are grouped; a class of n values over e eligible
//!    buckets contributes C(n+e-1, e-1) count distributions instead of e^n assignments.
//! 2. **Distribution search**: depth-first over classes (fewest eligible buckets first),
//!    assigning each class a count vector over its eligible buckets. After each commitment,
//!    every bucket is tested with [`rbe::RbeTable::feasible_neighs`] against its *candidate
//!    pool* — the values of classes that still can reach it. Pools shrink as classes commit
//!    elsewhere, so infeasible branches are refuted before enumeration descends into them
//!    (cross-bucket propagation the per-bucket guard in the engine cannot see).
//! 3. **Expansion**: each surviving distribution is expanded into the concrete partitions
//!    realising it (multiset permutations per class, odometer across classes), which the
//!    engine verifies with the exact derivative matcher as before.
//!
//! Soundness of pruning: for any complete partition P consistent with the committed counts,
//! each bucket's subset is contained in that bucket's candidate pool; `feasible_neighs`
//! refutes a pool only when no sub-bag of it can match, hence only branches containing no
//! valid partition are cut. Values eligible for no bucket are dropped, exactly as the
//! previous enumerator did (the engine keeps genuinely invalid values upstream so that the
//! partition fails; values whose key no bucket mentions are ignored here).
//!
//! See docs/dev/feasibility-model.md §5 (step 4) and the Jena implementation it ports.

use crate::Partitions;
use rbe::{Context, Key, RbeTable, Ref, Value};
use std::collections::HashMap;

/// Creates an iterator over the assignments of `neighs` to the triple expressions in
/// `exprs` that are not refuted by the feasibility analysis. Drop-in replacement for
/// [`crate::partitions_iter`]: emits a subset of its partitions containing every partition
/// that the exact matcher accepts (enumeration order differs).
pub fn class_partitions_iter<'a, T, K, V, R, Ctx>(
    neighs: &'a [(K, V, Ctx)],
    exprs: &'a HashMap<T, Vec<RbeTable<K, V, R, Ctx>>>,
) -> ClassPartitionIterator<'a, T, K, V, R, Ctx>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    T: std::hash::Hash + Eq + Clone,
{
    ClassPartitionIterator::new(neighs, exprs)
}

struct Class<K, V, Ctx> {
    values: Vec<(K, V, Ctx)>,
    /// Indexes into the bucket vector, ascending.
    eligible: Vec<usize>,
}

pub struct ClassPartitionIterator<'a, T, K, V, R, Ctx>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    T: std::hash::Hash + Eq + Clone,
{
    buckets: Vec<(&'a T, &'a Vec<RbeTable<K, V, R, Ctx>>)>,
    classes: Vec<Class<K, V, Ctx>>,
    /// counts[level] = committed count vector over classes[level].eligible; None above level.
    counts: Vec<Option<Vec<usize>>>,
    level: usize,
    /// Expansion state: perms[i] assigns classes[i].values to bucket indexes (a multiset
    /// permutation of the committed counts), advanced odometer-style. Non-empty while
    /// expanding a surviving distribution.
    perms: Option<Vec<Vec<usize>>>,
    exhausted: bool,
}

impl<'a, T, K, V, R, Ctx> ClassPartitionIterator<'a, T, K, V, R, Ctx>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    T: std::hash::Hash + Eq + Clone,
{
    fn new(neighs: &'a [(K, V, Ctx)], exprs: &'a HashMap<T, Vec<RbeTable<K, V, R, Ctx>>>) -> Self {
        let buckets: Vec<(&T, &Vec<RbeTable<K, V, R, Ctx>>)> = exprs.iter().collect();

        // Eligibility mirrors the previous enumerator: a bucket is eligible for a value
        // when some of its expressions mention the value's key.
        let bucket_keys: Vec<Vec<&K>> = buckets
            .iter()
            .map(|(_, rbes)| rbes.iter().flat_map(|rbe| rbe.keys()).collect())
            .collect();

        let mut class_map: HashMap<Vec<usize>, Vec<(K, V, Ctx)>> = HashMap::new();
        for (k, v, ctx) in neighs {
            let eligible: Vec<usize> = (0..buckets.len()).filter(|b| bucket_keys[*b].contains(&k)).collect();
            if eligible.is_empty() {
                // No bucket mentions this key: ignored, as in the previous enumerator.
                continue;
            }
            class_map
                .entry(eligible)
                .or_default()
                .push((k.clone(), v.clone(), ctx.clone()));
        }
        let mut classes: Vec<Class<K, V, Ctx>> = class_map
            .into_iter()
            .map(|(eligible, values)| Class { values, eligible })
            .collect();
        // Most-constrained classes first: fewer eligible buckets, then more values.
        classes.sort_by(|a, b| {
            a.eligible
                .len()
                .cmp(&b.eligible.len())
                .then(b.values.len().cmp(&a.values.len()))
        });

        let counts = vec![None; classes.len()];
        let mut iter = ClassPartitionIterator {
            buckets,
            classes,
            counts,
            level: 0,
            perms: None,
            exhausted: false,
        };
        // Initial feasibility: refute before enumerating anything.
        if !iter.feasible_now() {
            iter.exhausted = true;
        }
        iter
    }

    /// The candidate pool of a bucket: values of classes that can still reach it —
    /// unassigned classes with the bucket eligible, and committed classes sending it a
    /// non-zero count. Over-approximates the bucket's subset in any completion consistent
    /// with the committed counts.
    fn bucket_pool(&self, b: usize) -> Vec<(K, V, Ctx)> {
        let mut pool = Vec::new();
        for (i, class) in self.classes.iter().enumerate() {
            let Some(pos) = class.eligible.iter().position(|e| *e == b) else {
                continue;
            };
            let reachable = match &self.counts[i] {
                None => true,
                Some(c) => c[pos] > 0,
            };
            if reachable {
                pool.extend(class.values.iter().cloned());
            }
        }
        pool
    }

    /// Whether every bucket can still be satisfied by its candidate pool.
    fn feasible_now(&self) -> bool {
        for (b, (_, rbes)) in self.buckets.iter().enumerate() {
            let pool = self.bucket_pool(b);
            for rbe in rbes.iter() {
                if !rbe.feasible_neighs(&pool) {
                    return false;
                }
            }
        }
        true
    }

    /// First composition (lexicographically greatest) of the class size over its buckets.
    fn first_composition(&mut self, lvl: usize) {
        let size = self.classes[lvl].values.len();
        let e = self.classes[lvl].eligible.len();
        let mut c = vec![0usize; e];
        c[0] = size;
        self.counts[lvl] = Some(c);
    }

    /// Advances counts[lvl] to the next composition; clears it and returns false on exhaustion.
    fn next_composition(&mut self, lvl: usize) -> bool {
        let c = self.counts[lvl].as_mut().expect("composition to advance");
        let e = c.len();
        // Decreasing lexicographic order: find the rightmost position (before the last)
        // with a non-zero count, move one unit right, and pack the tail greedily left.
        for i in (0..e.saturating_sub(1)).rev() {
            if c[i] > 0 {
                let right_sum: usize = c[i + 1..].iter().sum();
                c[i] -= 1;
                for x in c[i + 1..].iter_mut() {
                    *x = 0;
                }
                c[i + 1] = right_sum + 1;
                return true;
            }
        }
        self.counts[lvl] = None;
        false
    }

    fn init_perms(&mut self) {
        let mut perms = Vec::with_capacity(self.classes.len());
        for (i, class) in self.classes.iter().enumerate() {
            let c = self.counts[i].as_ref().expect("complete distribution");
            let mut p = Vec::with_capacity(class.values.len());
            for (pos, count) in c.iter().enumerate() {
                for _ in 0..*count {
                    p.push(class.eligible[pos]);
                }
            }
            p.sort_unstable(); // smallest multiset permutation
            perms.push(p);
        }
        self.perms = Some(perms);
    }

    /// Advances the perms odometer; false when all permutations have been produced.
    fn advance_perms(&mut self) -> bool {
        let perms = self.perms.as_mut().expect("expansion in progress");
        for p in perms.iter_mut() {
            if next_permutation(p) {
                return true;
            }
            p.sort_unstable(); // wrapped: reset and carry
        }
        false
    }

    fn current_partition(&self) -> Partitions<T, K, V, R, Ctx> {
        let perms = self.perms.as_ref().expect("expansion in progress");
        let mut subsets: Vec<Vec<(K, V, Ctx)>> = vec![Vec::new(); self.buckets.len()];
        for (i, class) in self.classes.iter().enumerate() {
            for (vi, value) in class.values.iter().enumerate() {
                subsets[perms[i][vi]].push(value.clone());
            }
        }
        self.buckets
            .iter()
            .zip(subsets)
            .map(|((t, rbes), subset)| ((*t).clone(), (*rbes).clone(), subset))
            .collect()
    }
}

impl<T, K, V, R, Ctx> Iterator for ClassPartitionIterator<'_, T, K, V, R, Ctx>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    T: std::hash::Hash + Eq + Clone,
{
    type Item = Partitions<T, K, V, R, Ctx>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.exhausted {
            return None;
        }
        // Continue expanding the current distribution
        if self.perms.is_some() {
            if self.advance_perms() {
                return Some(self.current_partition());
            }
            self.perms = None;
            if self.level == 0 && self.classes.is_empty() {
                self.exhausted = true;
                return None;
            }
            self.level = self.level.saturating_sub(1);
            if self.classes.is_empty() {
                self.exhausted = true;
                return None;
            }
        }
        // Depth-first search for the next feasible complete distribution
        loop {
            if self.level == self.classes.len() {
                self.init_perms();
                return Some(self.current_partition());
            }
            let have = if self.counts[self.level].is_none() {
                self.first_composition(self.level);
                true
            } else {
                self.next_composition(self.level)
            };
            let have = have && {
                let mut ok = self.feasible_now();
                while !ok {
                    if !self.next_composition(self.level) {
                        break;
                    }
                    ok = self.feasible_now();
                }
                ok
            };
            if have {
                self.level += 1;
            } else {
                self.counts[self.level] = None;
                if self.level == 0 {
                    self.exhausted = true;
                    return None;
                }
                self.level -= 1;
            }
        }
    }
}

/// Next lexicographic permutation of a multiset; false if the array is the last one.
fn next_permutation(a: &mut [usize]) -> bool {
    if a.len() < 2 {
        return false;
    }
    let mut i = a.len() - 2;
    loop {
        if a[i] < a[i + 1] {
            break;
        }
        if i == 0 {
            return false;
        }
        i -= 1;
    }
    let mut j = a.len() - 1;
    while a[j] <= a[i] {
        j -= 1;
    }
    a.swap(i, j);
    a[i + 1..].reverse();
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use rbe::{MatchCond, Max, Pending, RbeStruct, SingleCond, rbe_error::RbeError};
    use std::collections::{HashMap, HashSet};

    /// Local newtype so the rbe marker traits can be implemented (orphan rule).
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
    struct C(char);
    impl std::fmt::Display for C {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    impl rbe::Key for C {}
    impl rbe::Value for C {}
    impl rbe::Ref for C {}
    impl rbe::Context for C {}

    type Cond = MatchCond<C, C, C, C>;
    type Table = RbeTable<C, C, C, C>;

    fn any(name: &str) -> Cond {
        MatchCond::single(
            SingleCond::new()
                .with_name(name)
                .with_cond(|_v: &C, _c: &C| Ok(Pending::empty())),
        )
    }

    fn is(name: &str, expected: char) -> Cond {
        MatchCond::single(SingleCond::new().with_name(name).with_cond(move |v: &C, _c: &C| {
            if v.0 == expected {
                Ok(Pending::empty())
            } else {
                Err(RbeError::MsgError {
                    msg: format!("{v} != {expected}"),
                })
            }
        }))
    }

    /// Bucket 'A' = { p .{1,1} ; q .{0,1} }, bucket 'B' = { p [a]{1,1} }.
    fn buckets() -> HashMap<char, Vec<Table>> {
        let mut ta = Table::new();
        let c1 = ta.add_component(C('p'), &any("any_p"));
        let c2 = ta.add_component(C('q'), &any("any_q"));
        ta.with_rbe(RbeStruct::and(vec![
            RbeStruct::symbol(c1, 1, Max::IntMax(1)),
            RbeStruct::symbol(c2, 0, Max::IntMax(1)),
        ]));
        let mut tb = Table::new();
        let c3 = tb.add_component(C('p'), &is("is_a", 'a'));
        tb.with_rbe(RbeStruct::symbol(c3, 1, Max::IntMax(1)));
        HashMap::from([('A', vec![ta]), ('B', vec![tb])])
    }

    fn canonical(parts: &Partitions<char, C, C, C, C>) -> Vec<(char, Vec<(char, char)>)> {
        let mut result: Vec<(char, Vec<(char, char)>)> = parts
            .iter()
            .map(|(t, _, subset)| {
                let mut vs: Vec<(char, char)> = subset.iter().map(|(k, v, _)| (k.0, v.0)).collect();
                vs.sort_unstable();
                (*t, vs)
            })
            .collect();
        result.sort();
        result
    }

    /// A partition is valid when every bucket's expressions accept its subset.
    fn is_valid(parts: &Partitions<char, C, C, C, C>) -> bool {
        parts.iter().all(|(_, rbes, subset)| {
            rbes.iter().all(|rbe| match rbe.matches(subset.clone()) {
                Ok(iter) => iter.into_iter().any(|r| r.is_ok()),
                Err(_) => false,
            })
        })
    }

    /// Differential contract against the previous enumerator: the class-based iterator
    /// emits a subset of the k-partition space that contains every valid partition.
    #[test]
    fn differential_against_k_partitions() {
        let exprs = buckets();
        let neighs: Vec<(C, C, C)> = vec![
            (C('p'), C('a'), C(' ')),
            (C('p'), C('b'), C(' ')),
            (C('q'), C('x'), C(' ')),
            (C('z'), C('z'), C(' ')),
        ];

        let new_parts: Vec<_> = class_partitions_iter(&neighs, &exprs).collect();
        let old_parts: Vec<_> = crate::partitions_iter(&neighs, &exprs).collect();

        let old_set: HashSet<_> = old_parts.iter().map(canonical).collect();
        let new_set: HashSet<_> = new_parts.iter().map(canonical).collect();

        for p in &new_set {
            assert!(old_set.contains(p), "invented partition: {p:?}");
        }
        let old_valid: HashSet<_> = old_parts.iter().filter(|p| is_valid(p)).map(canonical).collect();
        let new_valid: HashSet<_> = new_parts.iter().filter(|p| is_valid(p)).map(canonical).collect();
        assert_eq!(old_valid, new_valid, "valid partitions must be preserved");
        assert!(!new_valid.is_empty(), "the example admits a valid partition");
        assert!(
            new_parts.len() <= old_parts.len(),
            "pruning must not enumerate more than the Cartesian space"
        );
    }

    /// Infeasible instance: bucket B demands a 'p' with value 'a' but none exists.
    /// The class-based iterator refutes without enumerating; the valid sets agree (empty).
    #[test]
    fn refutes_without_enumeration() {
        let exprs = buckets();
        let neighs: Vec<(C, C, C)> = vec![(C('p'), C('b'), C(' ')), (C('q'), C('x'), C(' '))];
        let new_parts: Vec<_> = class_partitions_iter(&neighs, &exprs).collect();
        assert!(new_parts.is_empty(), "refuted upfront: B cannot be satisfied");
        let old_valid = crate::partitions_iter(&neighs, &exprs).filter(is_valid).count();
        assert_eq!(old_valid, 0);
    }

    /// Empty neighbourhood: one all-empty partition, matching the previous enumerator,
    /// unless some bucket cannot accept the empty bag.
    #[test]
    fn empty_neighbourhood() {
        let exprs = buckets();
        let neighs: Vec<(C, C, C)> = vec![];
        // Bucket A and B both require a 'p': the empty distribution is refuted.
        let new_parts: Vec<_> = class_partitions_iter(&neighs, &exprs).collect();
        assert!(new_parts.is_empty());
    }
}
