use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;
use serde_derive::{Serialize, Deserialize};

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Pending<V,R> 
where V: Hash + Eq 
{
    pending_map: HashMap<V,Vec<R>>
}

impl <V,R> Pending<V,R>
where V: Hash + Eq + Debug, 
      R: Debug
{

    pub fn new() -> Pending<V,R> {
        Pending {
            pending_map: HashMap::new()
        }
    }

    pub fn get(&self, k: &V) -> Option<&Vec<R>> {
        self.pending_map.get(k)
    }

    pub fn merge(mut self, other: Pending<V,R>) -> Self {
       for (k, mut vs) in other.pending_map.into_iter() {
             self
             .pending_map.entry(k)
             .and_modify(|v| { v.append(&mut vs); })
             .or_insert(vs);
       };
       self
    }

    pub fn insert(&mut self, v: V, r: R) {
        let mut vv = vec![r];
        self
        .pending_map
        .entry(v)
        .and_modify(|vs| {
            vs.append(&mut vv)
        })
        .or_insert(vv); 
    }

    pub fn from<T: IntoIterator<Item=(V,Vec<R>)>> (iter: T) -> Pending<V,R> {
        let mut pm = HashMap::new();
        for (v,mut r) in iter {
            pm.entry(v)
             .and_modify(|ps: &mut Vec<R>| {
                ps.append(&mut r); 
            })
             .or_insert(r);
        }
        Pending { pending_map: pm }
    }
}

#[cfg(test)]
mod tests {
    use crate::Pending;

    #[test]
    fn test_from() {
        let pending = Pending::from(vec![('a', vec![1,2]), ('b', vec![3])]);
        let expected = vec![1,2];
        assert_eq!(pending.get(&'a'), Some(&expected));
    }

    #[test]
    fn test_pending_merge() {
        let mut pending1 = Pending::from(vec![('a', vec![1,2]), ('b', vec![3])]);
        let pending2 = Pending::from(vec![('a', vec![3,4]), ('c', vec![4])]);
        let expected = Pending::from(vec![('a', vec![1,2,3,4]), ('c', vec![4]), ('b', vec![3])]);

        pending1 = pending1.merge(pending2);
        assert_eq!(pending1, expected);
    }
}