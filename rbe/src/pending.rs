use indexmap::{IndexMap, IndexSet, map::Entry};
use std::fmt::{Debug, Display};
use std::hash::Hash;

/// Indicates a map of values `V` that depend on some references `R`, each
/// reference annotated with the set of keys `K` that are pending for it.
#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub struct Pending<K, V, R>
where
    V: Hash + Eq,
    R: Hash + Eq,
    K: Hash + Eq,
{
    pending_map: IndexMap<V, IndexMap<R, IndexSet<K>>>,
}

impl<K, V, R> Pending<K, V, R>
where
    K: Hash + Eq + Clone + Debug,
    V: Hash + Eq + Clone + Debug,
    R: Hash + Eq + Clone + Debug,
{
    pub fn new() -> Pending<K, V, R> {
        Pending {
            pending_map: IndexMap::new(),
        }
    }

    pub fn empty() -> Pending<K, V, R> {
        Pending::new()
    }

    pub fn from_pair(v: V, r: R) -> Pending<K, V, R> {
        let mut pending = Pending::new();
        pending.insert(v, r);
        pending
    }

    pub fn get(&self, v: &V) -> Option<IndexSet<R>> {
        self.pending_map
            .get(v)
            .map(|r_map| r_map.keys().cloned().collect::<IndexSet<R>>())
    }

    pub fn len(&self) -> usize {
        let mut counter = 0;
        for rs in self.pending_map.values() {
            counter += rs.len();
        }
        counter
    }

    pub fn contains(&self, v: &V, r: &R) -> bool {
        self.pending_map.get(v).is_some_and(|rs| rs.contains_key(r))
    }

    pub fn merge(&mut self, other: Pending<K, V, R>) {
        for (v, other_rs) in other.pending_map.into_iter() {
            let self_rs = self.pending_map.entry(v).or_default();
            for (r, other_ks) in other_rs.into_iter() {
                self_rs.entry(r).or_default().extend(other_ks);
            }
        }
    }

    /// Registers `r` as a pending reference for `v`. Leaves any existing set
    /// of pending keys for `(v, r)` untouched, and creates an empty one
    /// otherwise.
    pub fn insert(&mut self, v: V, r: R) {
        let r_map = self.pending_map.entry(v).or_default();
        if let Entry::Vacant(vacant_r) = r_map.entry(r) {
            vacant_r.insert(IndexSet::new());
        }
    }

    /// Registers `k` as a pending key for `v` depending on `r`.
    pub fn insert_with_key(&mut self, v: V, r: R, k: K) {
        self.pending_map.entry(v).or_default().entry(r).or_default().insert(k);
    }

    pub fn insert_values<T: IntoIterator<Item = (R, K)>>(&mut self, v: V, iter: T) {
        for (r, k) in iter {
            self.insert_with_key(v.clone(), r, k)
        }
    }

    pub fn insert_from_iter<T: IntoIterator<Item = (V, R, K)>>(&mut self, iter: T) {
        for (v, r, k) in iter {
            self.insert_with_key(v, r, k)
        }
    }

    pub fn is_empty(&self) -> bool {
        self.pending_map.is_empty()
    }

    pub fn from<T, RS>(iter: T) -> Pending<K, V, R>
    where
        T: IntoIterator<Item = (V, RS)>,
        RS: IntoIterator<Item = (R, K)>,
    {
        let mut result = Pending::new();
        for (v, rs) in iter {
            result.insert_values(v, rs)
        }
        result
    }

    pub fn iter(&self) -> PendingIterator<'_, V, R, K> {
        PendingIterator {
            pending_iter: self.pending_map.iter(),
            current_v: None,
            current_r: None,
        }
    }

    /// Inserts `k` into the annotating key set of every existing `(V, R)` entry.
    /// Use this after receiving a pending result from a condition to stamp the
    /// symbol (predicate) that triggered the match onto all pending references.
    pub fn annotate_key(&mut self, k: &K) {
        for r_map in self.pending_map.values_mut() {
            for ks in r_map.values_mut() {
                ks.insert(k.clone());
            }
        }
    }

    /// Iterates over all `(V, R, K-set)` triples, yielding the value, the
    /// reference, and the full set of annotating keys associated with that pair.
    pub fn iter_vr(&self) -> impl Iterator<Item = (&V, &R, &IndexSet<K>)> {
        self.pending_map
            .iter()
            .flat_map(|(v, r_map)| r_map.iter().map(move |(r, ks)| (v, r, ks)))
    }

    fn select_v(&self) -> Option<V> {
        self.pending_map.first().map(|(v, _)| v.clone())
    }

    pub fn pop(&mut self) -> Option<(V, R, K)> {
        match self.select_v() {
            Some(v) => match self.pending_map.get_mut(&v) {
                Some(rs) => match rs.last_mut() {
                    Some((r, ks)) => {
                        let r = r.clone();
                        let k = ks.pop().unwrap_or_else(|| {
                            panic!("Internal error in pending map: no pending keys for value {v:?} and reference {r:?}")
                        });
                        if ks.is_empty() {
                            rs.swap_remove(&r);
                        }
                        if rs.is_empty() {
                            self.pending_map.swap_remove(&v);
                        }
                        Some((v, r, k))
                    },
                    None => {
                        panic!("Internal error in pending map: Cannot pop from value {v:?}");
                    },
                },
                None => {
                    panic!("Internal error in pending map: Key {v:?} without value?");
                },
            },
            None => None,
        }
    }
}

pub struct PendingIterator<'a, V, R, K>
where
    V: Hash + Eq,
    R: Hash + Eq,
{
    pending_iter: indexmap::map::Iter<'a, V, IndexMap<R, IndexSet<K>>>,
    current_v: Option<(&'a V, indexmap::map::Iter<'a, R, IndexSet<K>>)>,
    current_r: Option<(&'a R, indexmap::set::Iter<'a, K>)>,
}

impl<'a, V, R, K> Iterator for PendingIterator<'a, V, R, K>
where
    V: Hash + Eq,
    R: Hash + Eq,
{
    type Item = (&'a V, &'a R, &'a K);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some((r, k_it)) = self.current_r.as_mut()
                && let Some(k) = k_it.next()
            {
                let v = self.current_v.as_ref().unwrap().0;
                return Some((v, *r, k));
            }

            self.current_r = None;

            if let Some((_, r_it)) = self.current_v.as_mut()
                && let Some((r, ks)) = r_it.next()
            {
                self.current_r = Some((r, ks.iter()));
                continue;
            }
            self.current_v = None;

            match self.pending_iter.next() {
                Some((v, rs)) => {
                    self.current_v = Some((v, rs.iter()));
                },
                None => return None,
            }
        }
    }
}

impl<K, V, R> Display for Pending<K, V, R>
where
    K: Hash + Eq + Debug + Display,
    V: Hash + Eq + Debug + Display,
    R: Hash + Eq + Debug + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pending: {{")?;
        for (v, rs) in self.pending_map.iter() {
            write!(f, "{v}@")?;
            for (r, ks) in rs.iter() {
                write!(f, "{r}(")?;
                for k in ks.iter() {
                    write!(f, "{k} ")?;
                }
                write!(f, ") ")?;
            }
            write!(f, ", ")?;
        }
        write!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use crate::Pending;
    use indexmap::IndexSet;

    #[test]
    fn test_from() {
        let pending: Pending<char, char, i32> =
            Pending::from(vec![('a', vec![(1, 'x'), (2, 'y')]), ('b', vec![(3, 'z')])]);
        let expected = IndexSet::from_iter([1, 2]);
        assert_eq!(pending.get(&'a'), Some(expected));
    }

    #[test]
    fn test_pending_merge() {
        let mut pending1: Pending<char, char, i32> =
            Pending::from(vec![('a', vec![(1, 'x'), (2, 'y')]), ('b', vec![(3, 'z')])]);
        let pending2: Pending<char, char, i32> = Pending::from(vec![('a', vec![(3, 'w')]), ('c', vec![(4, 'v')])]);
        let expected: Pending<char, char, i32> = Pending::from(vec![
            ('a', vec![(1, 'x'), (2, 'y'), (3, 'w')]),
            ('b', vec![(3, 'z')]),
            ('c', vec![(4, 'v')]),
        ]);

        pending1.merge(pending2);
        assert_eq!(pending1, expected);
    }

    #[test]
    fn test_pending_iter() {
        let pending: Pending<char, char, i32> =
            Pending::from(vec![('a', vec![(1, 'x'), (2, 'y')]), ('b', vec![(3, 'z')])]);
        let values: IndexSet<(&char, &i32, &char)> = pending.iter().collect();
        let expected = IndexSet::from([(&'a', &1, &'x'), (&'a', &2, &'y'), (&'b', &3, &'z')]);
        assert_eq!(values, expected);
    }

    #[test]
    fn test_pop() {
        let mut pending: Pending<char, char, i32> =
            Pending::from(vec![('a', vec![(1, 'x'), (2, 'y')]), ('b', vec![(3, 'z')])]);
        let (v1, r1, k1) = pending.pop().unwrap();
        let (v2, r2, k2) = pending.pop().unwrap();
        let (v3, r3, k3) = pending.pop().unwrap();
        assert_eq!(pending.pop(), None);

        let mut new_pending: Pending<char, char, i32> = Pending::new();
        new_pending.insert_with_key(v1, r1, k1);
        new_pending.insert_with_key(v2, r2, k2);
        new_pending.insert_with_key(v3, r3, k3);

        let expected: Pending<char, char, i32> =
            Pending::from(vec![('a', vec![(1, 'x'), (2, 'y')]), ('b', vec![(3, 'z')])]);
        assert_eq!(new_pending, expected);
    }

    #[test]
    fn test_insert_keeps_existing_pending_keys() {
        let mut pending: Pending<char, char, i32> = Pending::new();

        // insert(v, r) registers (a, 1) with an empty set of pending keys.
        pending.insert('a', 1);
        assert!(pending.contains(&'a', &1));
        assert_eq!(pending.get(&'a'), Some(IndexSet::from([1])));

        // A pending key is recorded for (a, 1).
        pending.insert_with_key('a', 1, 'x');

        // Calling insert(v, r) again must not clear the pending keys
        // already associated with (a, 1).
        pending.insert('a', 1);
        let values: IndexSet<(&char, &i32, &char)> = pending.iter().collect();
        assert_eq!(values, IndexSet::from([(&'a', &1, &'x')]));
    }
}
