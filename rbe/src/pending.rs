use serde_derive::{Deserialize, Serialize};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Pending<V, R>
where
    V: Hash + Eq,
    R: Hash + Eq,
{
    pending_map: HashMap<V, HashSet<R>>,
}

impl<V, R> Pending<V, R>
where
    V: Hash + Eq + Clone + Debug,
    R: Hash + Eq + Clone + Debug,
{
    pub fn new() -> Pending<V, R> {
        Pending {
            pending_map: HashMap::new(),
        }
    }

    pub fn get(&self, k: &V) -> Option<&HashSet<R>> {
        self.pending_map.get(k)
    }

    pub fn len(&self) -> usize {
        let mut counter = 0;
        for key in self.pending_map.keys() {
            counter += self.pending_map.get(key).unwrap().len();
        }
        counter
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
                vacant.insert(HashSet::from([r]));
            }
        }
    }

    pub fn insert_values<T: IntoIterator<Item = R>>(&mut self, v: V, iter: T) {
        match self.pending_map.entry(v) {
            Entry::Occupied(mut v) => {
                v.get_mut().extend(iter);
            }
            Entry::Vacant(vacant) => {
                vacant.insert(HashSet::from_iter(iter));
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
        if let Some(v) = self.pending_map.keys().next() {
            Some((*v).clone())
        } else {
            None
        }
    }

    fn select_r(rs: &HashSet<R>) -> Option<R> {
        if let Some(r) = rs.iter().next() {
            Some((*r).clone())
        } else {
            None
        }
    }

    fn pop_hash_set(rs: &mut HashSet<R>) -> Option<R> {
        if let Some(r) = Self::select_r(&rs) {
            rs.remove(&r);
            Some(r)
        } else {
            None
        }
    }

    pub fn pop(&mut self) -> Option<(V, R)> {
        match self.select_v() {
            Some(v) => {
                let cloned_v = v.clone();
                match self.pending_map.entry(v) {
                    Entry::Occupied(mut occupied) => {
                        let rs = occupied.get_mut();
                        if let Some(r) = Self::pop_hash_set(rs) {
                            if rs.is_empty() {
                                self.pending_map.remove(&cloned_v);
                            }
                            Some((cloned_v, r))
                        } else {
                            panic!("Internal error: Couldn't pop r from hash_set: {rs:?} ")
                        }
                    }
                    Entry::Vacant(vac) => {
                        panic!("Internal error. HashMap Should contain a value for key. {vac:?}")
                    }
                }
            }
            None => None,
        }
    }
}

pub struct PendingIterator<'a, V, R>
where
    V: Hash + Eq,
{
    pending_iter: std::collections::hash_map::Iter<'a, V, HashSet<R>>,
    current_state: Option<(&'a V, std::collections::hash_set::Iter<'a, R>)>,
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
    use std::collections::HashSet;

    #[test]
    fn test_from() {
        let pending = Pending::from(vec![('a', vec![1, 2]), ('b', vec![3])]);
        let expected = HashSet::from_iter([1, 2].into_iter());
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
        let hash_set: HashSet<(&char, &i32)> = pending.iter().collect();
        let expected = HashSet::from([(&'a', &1), (&'b', &3), (&'a', &2)]);
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
