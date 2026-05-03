//! Utilities for generating k-partitions of a set with predicates on each subset.
//!
//! This is used in ShEx validation to partition the neighborhood of a node according to the
//! triple expressions of a shape definition.
//! Each subset of the partition must satisfy the predicate associated to the corresponding
//! triple expression.

use rbe::{Context, Key, RbeTable, Ref, Value};
use std::collections::{HashMap, HashSet};

pub type Partitions<T, K, V, R, Ctx> = Vec<Partition<T, K, V, R, Ctx>>;
pub type Partition<T, K, V, R, Ctx> = (T, Vec<RbeTable<K, V, R, Ctx>>, Vec<(K, V, Ctx)>);

/// Iterator over k-partitions of a set with predicates on each subset.
pub struct KPartitionIteratorMultiPredicate<T, F> {
    items: Vec<T>,
    k: usize,
    current: Option<Vec<usize>>,
    predicates: Vec<F>,
}

impl<T: Clone, F> KPartitionIteratorMultiPredicate<T, F>
where
    F: Fn(&Vec<T>) -> bool,
{
    pub fn new(items: Vec<T>, predicates: Vec<F>) -> Self {
        let k = predicates.len();
        // Items rejected as singletons by every predicate can never be placed in any group;
        // exclude them so the remaining items still produce a valid (possibly all-empty) partition.
        let items: Vec<T> = items
            .into_iter()
            .filter(|item| predicates.iter().any(|p| p(&vec![item.clone()])))
            .collect();
        let current = Some(vec![0; items.len()]);
        Self {
            items,
            k,
            current,
            predicates,
        }
    }
}

impl<T: Clone, F> Iterator for KPartitionIteratorMultiPredicate<T, F>
where
    F: Fn(&Vec<T>) -> bool,
{
    type Item = Vec<Vec<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let assignment = self.current.as_ref()?;

            // Build current partition
            let mut partitions = vec![Vec::new(); self.k];
            for (item, &partition_idx) in self.items.iter().zip(assignment.iter()) {
                partitions[partition_idx].push(item.clone());
            }

            // Increment to next assignment
            let mut next_assignment = assignment.clone();
            let mut carry = true;
            for digit in next_assignment.iter_mut() {
                if carry {
                    *digit += 1;
                    if *digit < self.k {
                        carry = false;
                    } else {
                        *digit = 0;
                    }
                }
            }

            self.current = if carry { None } else { Some(next_assignment) };

            // Check each subset with its corresponding predicate
            let all_valid = partitions
                .iter()
                .zip(self.predicates.iter())
                .all(|(subset, predicate)| predicate(subset));

            if all_valid {
                return Some(partitions);
            }
        }
    }
}

/// Creates an iterator of all possible combinations of neighbours `neighs`
/// that can be assigned to each triple expression in the `exprs` map
pub fn partitions_iter<'a, T, K, V, R, Ctx>(
    neighs: &'a [(K, V, Ctx)],
    exprs: &'a HashMap<T, Vec<RbeTable<K, V, R, Ctx>>>,
) -> impl Iterator<Item = Partitions<T, K, V, R, Ctx>> + 'a
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    T: std::hash::Hash + Eq + Clone,
{
    // Build a vector of predicates, one for each triple expression, that checks if a given subset of neighbours satisfies the conditions of the triple expression
    let conditions = build_conditions(exprs).collect::<Vec<_>>();
    // Create an iterator over all possible partitions of the neighbours whose predicates are included in the conditions vector
    let iter_partitions = KPartitionIteratorMultiPredicate::new(neighs.to_owned(), conditions);
    iter_partitions.map(|partition| {
        partition
            .into_iter()
            .zip(exprs.iter())
            .map(|(subset, (key, rbes))| (key.clone(), rbes.clone(), subset))
            .collect()
    })
}

/// Builds a vector of predicates, one for each triple expression,
/// that checks if a given subset of neighbours satisfies the conditions
/// of the triple expression.
/// Each predicate checks if all the predicates in the triple expression
/// are present in the subset of neighbours.
fn build_conditions<'a, T, K, V, R, Ctx>(
    triple_exprs: &'a HashMap<T, Vec<RbeTable<K, V, R, Ctx>>>,
) -> impl Iterator<Item = impl Fn(&Vec<(K, V, Ctx)>) -> bool> + 'a
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
    T: std::hash::Hash + Eq + Clone,
{
    triple_exprs.values().map(|rbes| {
        // Collect all predicates from the triple expression into a set
        let preds: Vec<K> = rbes
            .iter()
            .flat_map(|rbe| rbe.keys().cloned())
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();
        // Return a predicate that checks if all predicates in the triple expression
        // are present in the subset of neighbours
        move |subset: &Vec<(K, V, Ctx)>| subset.iter().all(|(p, _, _)| preds.contains(p))
    })
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::KPartitionIteratorMultiPredicate;

    fn build_predicates<'a>(
        triple_exprs: &'a HashMap<char, Vec<char>>,
    ) -> impl Iterator<Item = impl Fn(&Vec<(char, i32)>) -> bool> + 'a {
        triple_exprs.values().map(|preds| {
            let preds = preds.clone();
            move |subset: &Vec<(char, i32)>| subset.iter().all(|(p, _)| preds.contains(p))
        })
    }

    fn always_true<T>(_: &Vec<T>) -> bool {
        true
    }

    #[tracing_test::traced_test]
    #[test]
    fn test_k_partitions_preds() {
        let data: Vec<(char, i32)> = vec![('P', 1), ('P', 2), ('Q', 1), ('Q', 2)];
        let triple_exprs = HashMap::from([('A', vec!['P', 'Q']), ('B', vec!['P']), ('C', vec!['Q'])]);
        let predicates = build_predicates(&triple_exprs).collect::<Vec<_>>();
        let mut count = 0;
        for (i, partition) in KPartitionIteratorMultiPredicate::new(data.clone(), predicates).enumerate() {
            println!("{}: {:?}", i, partition);
            count += 1;
        }
        /*
        0: [[('P', 1), ('P', 2), ('Q', 1), ('Q', 2)], [], []]
        1: [[('P', 2), ('Q', 1), ('Q', 2)], [], [('P', 1)]]
        2: [[('P', 1), ('Q', 1), ('Q', 2)], [], [('P', 2)]]
        3: [[('Q', 1), ('Q', 2)], [], [('P', 1), ('P', 2)]]
        4: [[('P', 1), ('P', 2), ('Q', 2)], [('Q', 1)], []]
        5: [[('P', 2), ('Q', 2)], [('Q', 1)], [('P', 1)]]
        6: [[('P', 1), ('Q', 2)], [('Q', 1)], [('P', 2)]]
        7: [[('Q', 2)], [('Q', 1)], [('P', 1), ('P', 2)]]
        8: [[('P', 1), ('P', 2), ('Q', 1)], [('Q', 2)], []]
        9: [[('P', 2), ('Q', 1)], [('Q', 2)], [('P', 1)]]
        10: [[('P', 1), ('Q', 1)], [('Q', 2)], [('P', 2)]]
        11: [[('Q', 1)], [('Q', 2)], [('P', 1), ('P', 2)]]
        12: [[('P', 1), ('P', 2)], [('Q', 1), ('Q', 2)], []]
        13: [[('P', 2)], [('Q', 1), ('Q', 2)], [('P', 1)]]
        14: [[('P', 1)], [('Q', 1), ('Q', 2)], [('P', 2)]]
        15: [[], [('Q', 1), ('Q', 2)], [('P', 1), ('P', 2)]]
        */
        assert_eq!(count, 16);
    }

    #[tracing_test::traced_test]
    #[test]
    fn test_k_partitions_preds_empty() {
        let data: Vec<(char, i32)> = vec![('R', 1)];
        let triple_exprs = HashMap::from([('A', vec!['P', 'Q']), ('B', vec!['P']), ('C', vec!['Q'])]);
        let predicates = build_predicates(&triple_exprs).collect::<Vec<_>>();
        let mut count = 0;
        for (i, partition) in KPartitionIteratorMultiPredicate::new(data.clone(), predicates).enumerate() {
            println!("{}: {:?}", i, partition);
            count += 1;
        }
        /*
        0: [[], [], []]
        */
        assert_eq!(count, 1);
    }

    #[test]
    fn test_k2_n2_no_filter_count() {
        let data = vec![1, 2];
        let predicates: Vec<fn(&Vec<i32>) -> bool> = vec![always_true, always_true];
        let count = KPartitionIteratorMultiPredicate::new(data, predicates).count();
        assert_eq!(count, 4); // 2^2
    }

    #[test]
    fn test_k2_n3_no_filter_count() {
        let data = vec![1, 2, 3];
        let predicates: Vec<fn(&Vec<i32>) -> bool> = vec![always_true, always_true];
        let count = KPartitionIteratorMultiPredicate::new(data, predicates).count();
        assert_eq!(count, 8); // 2^3
    }

    #[test]
    fn test_k3_n2_no_filter_count() {
        let data = vec![1, 2];
        let predicates: Vec<fn(&Vec<i32>) -> bool> = vec![always_true, always_true, always_true];
        let count = KPartitionIteratorMultiPredicate::new(data, predicates).count();
        assert_eq!(count, 9); // 3^2
    }

    #[test]
    fn test_k1_single_partition() {
        let data = vec![1, 2, 3];
        let predicates: Vec<fn(&Vec<i32>) -> bool> = vec![always_true];
        let partitions: Vec<_> = KPartitionIteratorMultiPredicate::new(data.clone(), predicates).collect();
        assert_eq!(partitions.len(), 1);
        assert_eq!(partitions[0], vec![data]);
    }

    // With one item and two groups the two partitions are [[item], []] and [[], [item]].
    #[test]
    fn test_single_item_exact_partitions() {
        let data = vec![42];
        let predicates: Vec<fn(&Vec<i32>) -> bool> = vec![always_true, always_true];
        let partitions: Vec<_> = KPartitionIteratorMultiPredicate::new(data, predicates).collect();
        assert_eq!(partitions.len(), 2);
        assert_eq!(partitions[0], vec![vec![42], vec![]]);
        assert_eq!(partitions[1], vec![vec![], vec![42]]);
    }

    // Predicates that require group 0 to contain only evens and group 1 to contain only odds
    // force a unique valid partition.
    #[test]
    fn test_predicate_filters_items() {
        let data = vec![1, 2, 3, 4];
        let is_even: fn(&Vec<i32>) -> bool = |v| v.iter().all(|x| x % 2 == 0);
        let is_odd: fn(&Vec<i32>) -> bool = |v| v.iter().all(|x| x % 2 != 0);
        let partitions: Vec<_> = KPartitionIteratorMultiPredicate::new(data, vec![is_even, is_odd]).collect();
        assert_eq!(partitions.len(), 1);
        assert!(
            partitions[0][0].iter().all(|x| x % 2 == 0),
            "group 0 should contain only evens"
        );
        assert!(
            partitions[0][1].iter().all(|x| x % 2 != 0),
            "group 1 should contain only odds"
        );
    }

    // A predicate that always rejects non-empty subsets allows only partitions where that bucket
    // stays empty — forcing all items into the other bucket.
    #[test]
    fn test_reject_nonempty_forces_single_bucket() {
        let data = vec![1, 2, 3];
        let always_true_fn: fn(&Vec<i32>) -> bool = always_true;
        let reject_nonempty: fn(&Vec<i32>) -> bool = |v| v.is_empty();
        let partitions: Vec<_> =
            KPartitionIteratorMultiPredicate::new(data.clone(), vec![always_true_fn, reject_nonempty]).collect();
        assert_eq!(partitions.len(), 1);
        assert_eq!(partitions[0][0], data);
        assert!(partitions[0][1].is_empty());
    }
}
