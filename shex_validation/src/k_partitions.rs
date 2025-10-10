//! Utilities for generating k-partitions of a set with predicates on each subset.
//!
//! This is used in ShEx validation to partition the neighborhood of a node according to the
//! triple expressions of a shape definition.
//! Each subset of the partition must satisfy the predicate associated to the corresponding
//! triple expression.

use rbe::{Key, RbeTable, Ref, Value};
use std::collections::{HashMap, HashSet};

pub type Partitions<T, K, V, R> = Vec<Partition<T, K, V, R>>;
pub type Partition<T, K, V, R> = (T, Vec<RbeTable<K, V, R>>, Vec<(K, V)>);

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
        let current = if items.is_empty() {
            None
        } else {
            Some(vec![0; items.len()])
        };
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

/// Creates an iterator of all possible combinations of neighbours that can be assigned to eash triple expression in the `triple_exprs` map
pub fn partitions_iter<'a, T, K, V, R>(
    neighs: &'a [(K, V)],
    exprs: &'a HashMap<T, Vec<RbeTable<K, V, R>>>,
) -> impl Iterator<Item = Partitions<T, K, V, R>> + 'a
where
    K: Key,
    V: Value,
    R: Ref,
    T: std::hash::Hash + Eq + Clone,
{
    let conditions = build_conditions(exprs).collect::<Vec<_>>();
    let iter_partitions = KPartitionIteratorMultiPredicate::new(neighs.to_owned(), conditions);
    iter_partitions.map(|partition| {
        partition
            .into_iter()
            .zip(exprs.iter())
            .map(|(subset, (key, rbes))| (key.clone(), rbes.clone(), subset))
            .collect()
    })
}

fn build_conditions<'a, T, K, V, R>(
    triple_exprs: &'a HashMap<T, Vec<RbeTable<K, V, R>>>,
) -> impl Iterator<Item = impl Fn(&Vec<(K, V)>) -> bool> + 'a
where
    K: Key,
    V: Value,
    R: Ref,
    T: std::hash::Hash + Eq + Clone,
{
    triple_exprs.values().map(|rbes| {
        let preds: Vec<K> = rbes
            .iter()
            .flat_map(|rbe| rbe.keys().cloned())
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();
        move |subset: &Vec<(K, V)>| subset.iter().all(|(p, _)| preds.contains(p))
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

    #[test]
    fn test_k_partitions_preds() {
        // Example neighborhood data
        let data: Vec<(char, i32)> = vec![('P', 1), ('P', 2), ('Q', 1), ('Q', 2)];

        // Example triple expressions with predicates
        let triple_exprs =
            HashMap::from([('A', vec!['P', 'Q']), ('B', vec!['P']), ('C', vec!['Q'])]);
        let predicates = build_predicates(&triple_exprs).collect::<Vec<_>>();
        for (i, partition) in
            KPartitionIteratorMultiPredicate::new(data.clone(), predicates).enumerate()
        {
            println!("{}: {:?}", i, partition);
        }
        /*
        0: [[('P', 1), ('P', 2), ('Q', 1), ('Q', 2)], [], []]
        1: [[('P', 2), ('Q', 1), ('Q', 2)], [('P', 1)], []]
        2: [[('P', 1), ('Q', 1), ('Q', 2)], [('P', 2)], []]
        3: [[('Q', 1), ('Q', 2)], [('P', 1), ('P', 2)], []]
        4: [[('P', 1), ('P', 2), ('Q', 2)], [], [('Q', 1)]]
        5: [[('P', 2), ('Q', 2)], [('P', 1)], [('Q', 1)]]
        6: [[('P', 1), ('Q', 2)], [('P', 2)], [('Q', 1)]]
        7: [[('Q', 2)], [('P', 1), ('P', 2)], [('Q', 1)]]
        8: [[('P', 1), ('P', 2), ('Q', 1)], [], [('Q', 2)]]
        9: [[('P', 2), ('Q', 1)], [('P', 1)], [('Q', 2)]]
        10: [[('P', 1), ('Q', 1)], [('P', 2)], [('Q', 2)]]
        11: [[('Q', 1)], [('P', 1), ('P', 2)], [('Q', 2)]]
        12: [[('P', 1), ('P', 2)], [], [('Q', 1), ('Q', 2)]]
        13: [[('P', 2)], [('P', 1)], [('Q', 1), ('Q', 2)]]
        14: [[('P', 1)], [('P', 2)], [('Q', 1), ('Q', 2)]]
        15: [[], [('P', 1), ('P', 2)], [('Q', 1), ('Q', 2)]]
        */
    }
}
