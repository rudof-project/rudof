//! A set whose elements can be repeated. The set tracks how many times each element appears
//!
use itertools::Itertools;
use serde_derive::{Serialize,Deserialize};
use std::{fmt::{Display, Debug}, hash::Hash, collections::HashMap};


#[derive(Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Bag1<K,V>
where K: Hash + Eq + PartialEq,
{
    bag: HashMap<K,V>,
}

impl<K, V> Bag1<K,V> 
where K: Hash + Eq
{
    #[inline]
    pub fn new() -> Bag1<K,V> {
        Bag1 {
            bag: HashMap::new(),
        }
    }

    pub fn insert(&mut self, value: (K,V)) -> Option<V>{
        let (k,v) = value;
        let result = self.bag.insert(k, v);
        result
    }

    pub fn contains(&self, key: &K) -> bool {
        self.bag.get(key).is_some()
    }

    pub fn len(&self) -> usize {
        self.bag.len()
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, K, V> {
        self.bag.iter()
    }

}

impl <K: Hash + Eq + PartialEq, V> Display for Bag1<K,V>
where
    K: Display,
    V: Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v: Vec<String> = self
            .bag
            .iter()
            .map(|(k, v)| format!("{}/{}", k, v))
            .intersperse_with(|| ",".to_string())
            .collect();
        write!(f, "Bag [{}]", v.join(", "))
    }
}

impl <K,V> From<Vec<(K,V)>> for Bag1<K,V> 
where K: Hash + Eq + Debug,
      V: Debug
{
    fn from(v: Vec<(K,V)>) -> Self { 
        let mut hm = HashMap::new();
        for (k,v) in v {
            hm.insert(k,v);
        }
        Bag1 { bag: hm }
    }
}

/*impl <'a, K, V> IntoIterator for Bag1<K,V> 
    where K: Hash + Eq + Debug,
    V: Debug {
    type Item = (K,V);
    type IntoIter = Bag1Iterator<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        Bag1Iterator {
            bag: self,
            inner_iter: self.bag.iter(),
        }
    }
}
struct Bag1Iterator<'a, K, V> 
where K: Hash + Eq + Debug,
      V: Debug {
    bag: Bag1<K,V>, 
    inner_iter: std::slice::Iter<'a, (K,V)>
}

impl <'a, K,V> Iterator for Bag1Iterator<'a, K, V> 
where K: Hash + Eq + Debug, 
      V: Debug 
{
    type Item = (K,V);
    fn next(&mut self) -> Option<(K,V)> {
        match self.inner_iter.next() {
            Some(r) => Some(*r),
            None => None
        }
    }
}*/

impl <K,V> Debug for Bag1<K,V>
where
    K: Hash + Eq + Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v: Vec<String> = self
            .bag
            .iter()
            .map(|(k, v)| format!("{:?}/{:?}", k, v))
            .intersperse_with(|| ",".to_string())
            .collect();
        write!(f, "Bag [{}]", v.join(", "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bag_test() {
        let mut bag = Bag1::new();
        bag.insert(("a",1));
        bag.insert(("b",1));
        bag.insert(("b",2));
        assert_eq!(bag.contains(&"b"), true);
    }

    #[test]
    fn bag_from_vec_test() {
        let mut bag = Bag1::from(vec![("a",1), ("b",23)]);
        assert_eq!(bag.contains(&"a"), true);
    }

}
