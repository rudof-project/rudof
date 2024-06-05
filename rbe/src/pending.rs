use indexmap::{map::Entry, IndexMap, IndexSet};
use std::fmt::Debug;
use std::hash::Hash;

/// Indicates a map of values `V` that depend on some references `R`
#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub struct Pending<V, R>
where
    V: Hash + Eq,
    R: Hash + Eq,
{
    pending_map: IndexMap<V, IndexSet<R>>,
}

impl<V, R> Pending<V, R>
where
    V: Hash + Eq + Clone + Debug,
    R: Hash + Eq + Clone + Debug,
{
    pub fn new() -> Pending<V, R> {
        Pending {
            pending_map: IndexMap::new(),
        }
    }

    pub fn empty() -> Pending<V, R> {
        Pending::new()
    }

    pub fn from_pair(v: V, r: R) -> Pending<V, R> {
        let mut pending_map = IndexMap::new();
        pending_map.insert(v, IndexSet::from([r]));
        Pending { pending_map }
    }

    pub fn get(&self, k: &V) -> Option<&IndexSet<R>> {
        self.pending_map.get(k)
    }

    pub fn len(&self) -> usize {
        let mut counter = 0;
        for key in self.pending_map.keys() {
            counter += self.pending_map.get(key).unwrap().len();
        }
        counter
    }

    pub fn contains(&self, v: &V, r: &R) -> bool {
        if let Some(rs) = self.pending_map.get(v) {
            rs.contains(r)
        } else {
            false
        }
    }

    pub fn merge(&mut self, other: Pending<V, R>) {
        for (k, vs) in other.pending_map.into_iter() {
            match self.pending_map.entry(k) {
                Entry::Occupied(mut v) => v.get_mut().extend(vs),
                Entry::Vacant(vacant) => {
                    vacant.insert(vs);
                }
            }
        }
    }

    pub fn insert(&mut self, v: V, r: R) {
        match self.pending_map.entry(v) {
            Entry::Occupied(mut v) => {
                v.get_mut().insert(r);
            }
            Entry::Vacant(vacant) => {
                vacant.insert(IndexSet::from([r]));
            }
        }
    }

    pub fn insert_values<T: IntoIterator<Item = R>>(&mut self, v: V, iter: T) {
        match self.pending_map.entry(v) {
            Entry::Occupied(mut v) => {
                v.get_mut().extend(iter);
            }
            Entry::Vacant(vacant) => {
                vacant.insert(IndexSet::from_iter(iter));
            }
        }
    }

    pub fn insert_from_iter<T: IntoIterator<Item = (V, R)>>(&mut self, iter: T) {
        for (v, r) in iter {
            self.insert(v, r)
        }
    }

    pub fn is_empty(&self) -> bool {
        self.pending_map.is_empty()
    }

    pub fn from<T: IntoIterator<Item = (V, RS)>, RS: IntoIterator<Item = R>>(
        iter: T,
    ) -> Pending<V, R> {
        let mut result = Pending::new();
        for (v, rs) in iter {
            result.insert_values(v, rs)
        }
        result
    }

    pub fn iter(&self) -> PendingIterator<'_, V, R> {
        PendingIterator {
            pending_iter: self.pending_map.iter(),
            current_state: None,
        }
    }

    fn select_v(&self) -> Option<V> {
        self.pending_map.first().map(|(v, _)| v.clone())
    }

    pub fn pop(&mut self) -> Option<(V, R)> {
        match self.select_v() {
            Some(v) => match self.pending_map.get_mut(&v) {
                Some(rs) => match rs.pop() {
                    Some(r) => {
                        if rs.is_empty() {
                            self.pending_map.swap_remove(&v);
                        }
                        Some((v.clone(), r.clone()))
                    }
                    None => {
                        panic!("Internal error in penidng map: Cannot pop from value {v:?}");
                    }
                },
                None => {
                    panic!("Internal error in pending map: Key {v:?} without value?");
                }
            },
            None => None,
        }
    }
}

pub struct PendingIterator<'a, V, R>
where
    V: Hash + Eq,
{
    pending_iter: indexmap::map::Iter<'a, V, IndexSet<R>>,
    current_state: Option<(&'a V, indexmap::set::Iter<'a, R>)>,
}

impl<'a, V, R> Iterator for PendingIterator<'a, V, R>
where
    V: Hash + Eq,
{
    type Item = (&'a V, &'a R);

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.current_state {
            None => match self.pending_iter.next() {
                None => None,
                Some((v, rs)) => {
                    self.current_state = Some((v, rs.into_iter()));
                    self.next()
                }
            },
            Some((v, it)) => match it.next() {
                None => {
                    self.current_state = None;
                    match self.pending_iter.next() {
                        None => None,
                        Some((v, rs)) => {
                            self.current_state = Some((v, rs.iter()));
                            self.next()
                        }
                    }
                }
                Some(r) => Some((v, r)),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Pending;
    use indexmap::IndexSet;

    #[test]
    fn test_from() {
        let pending = Pending::from(vec![('a', vec![1, 2]), ('b', vec![3])]);
        let expected = IndexSet::from_iter([1, 2]);
        assert_eq!(pending.get(&'a'), Some(&expected));
    }

    #[test]
    fn test_pending_merge() {
        let mut pending1 = Pending::from(vec![('a', vec![1, 2]), ('b', vec![3])]);
        let pending2 = Pending::from(vec![('a', vec![3, 4]), ('c', vec![4])]);
        let expected = Pending::from(vec![
            ('a', vec![1, 2, 3, 4]),
            ('c', vec![4]),
            ('b', vec![3]),
        ]);

        pending1.merge(pending2);
        assert_eq!(pending1, expected);
    }

    #[test]
    fn test_pending_iter() {
        let pending = Pending::from(vec![('a', vec![1, 2]), ('b', vec![3])]);
        let hash_set: IndexSet<(&char, &i32)> = pending.iter().collect();
        let expected = IndexSet::from([(&'a', &1), (&'b', &3), (&'a', &2)]);
        assert_eq!(hash_set, expected);
    }

    #[test]
    fn test_pop() {
        let mut pending = Pending::from(vec![('a', vec![1, 2]), ('b', vec![3])]);
        let (v1, r1) = pending.pop().unwrap();
        println!("After pop1: {pending:?}...popped: {v1:?},{r1:?}");
        let (v2, r2) = pending.pop().unwrap();
        println!("After pop2: {pending:?}...popped: {v2:?},{r2:?}");
        let (v3, r3) = pending.pop().unwrap();
        println!("After pop3: {pending:?}...popped: {v3:?},{r3:?}");
        let final_pop = pending.pop();
        assert_eq!(final_pop, None);
        let mut new_pending = Pending::new();
        new_pending.insert(v1, r1);
        new_pending.insert(v2, r2);
        new_pending.insert(v3, r3);
        let expected = Pending::from(vec![('a', vec![1, 2]), ('b', vec![3])]);
        assert_eq!(new_pending, expected);
    }
}
