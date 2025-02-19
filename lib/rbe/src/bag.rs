//! A set whose elements can be repeated. The set tracks how many times each element appears
//!
use hashbag::{HashBag, SetIter};
use serde::{de::SeqAccess, ser::SerializeSeq, Deserialize, Deserializer, Serialize, Serializer};
use std::{
    fmt::{self, Debug, Display},
    hash::{Hash, Hasher},
    marker::PhantomData,
};

#[derive(Clone, PartialEq, Eq, Default)]
pub struct Bag<T>
where
    T: Hash + Eq + PartialEq,
{
    bag: HashBag<T>,
}

impl<T: Hash + Eq> Bag<T> {
    #[inline]
    pub fn new() -> Bag<T> {
        Bag {
            bag: HashBag::new(),
        }
    }

    pub fn insert(&mut self, value: T) -> usize {
        self.bag.insert(value)
    }

    pub fn insert_many(&mut self, value: T, n: usize) -> usize {
        self.bag.insert_many(value, n)
    }

    pub fn contains(&self, value: &T) -> usize {
        self.bag.contains(value)
    }

    pub fn len(&self) -> usize {
        self.bag.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bag.is_empty()
    }

    pub fn iter(&self) -> SetIter<'_, T> {
        self.bag.set_iter()
    }
}

impl<T: Hash + Eq + PartialEq> Display for Bag<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v: Vec<String> = self
            .bag
            .set_iter()
            .map(|(t, n)| format!("{}/{}", t, n))
            .collect();
        write!(f, "Bag [{}]", v.join(", "))
    }
}

/*impl<T> Default for Bag<T> {
    fn default() -> Self {
        Bag::new()
    }
}*/

impl<T> Debug for Bag<T>
where
    T: Hash + Eq + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v: Vec<String> = self
            .bag
            .set_iter()
            .map(|(t, n)| format!("{:?}/{}", t, n))
            .collect();
        write!(f, "Bag [{}]", v.join(", "))
    }
}

impl<T> FromIterator<T> for Bag<T>
where
    T: Eq + Hash,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut bag = Bag::new();
        for t in iter {
            bag.insert(t);
        }
        bag
    }
}

impl<T, const N: usize> From<[T; N]> for Bag<T>
where
    T: Eq + Hash,
{
    fn from(arr: [T; N]) -> Self {
        let mut bag = Bag::new();
        for x in arr {
            bag.insert(x);
        }
        bag
    }
}

impl<T> Serialize for Bag<T>
where
    T: Hash + Eq + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut bag = serializer.serialize_seq(Some(self.len()))?;

        for (entry, count) in self.iter() {
            bag.serialize_element(&(entry, count))?;
        }

        bag.end()
    }
}

impl<'de, T> Deserialize<'de> for Bag<T>
where
    T: Deserialize<'de> + Eq + Hash,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(BagVisitor::new())
    }
}

use serde::de::Visitor;
struct BagVisitor<T>
where
    T: Hash + Eq,
{
    marker: PhantomData<fn() -> Bag<T>>,
}

impl<T> BagVisitor<T>
where
    T: Hash + Eq,
{
    fn new() -> Self {
        BagVisitor {
            marker: PhantomData,
        }
    }
}

impl<'de, T> Visitor<'de> for BagVisitor<T>
where
    T: Hash + Eq + Deserialize<'de>,
{
    type Value = Bag<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a Bag")
    }

    fn visit_seq<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: SeqAccess<'de>,
    {
        let mut bag: Bag<T> = Bag::new();
        while let Some(entry) = access.next_element::<(T, usize)>()? {
            let (t, n) = entry;
            bag.insert_many(t, n);
        }
        Ok(bag)
    }
}

// Todo: I am not sure if this is idiomatic or performant (probably neither...)
impl<T: Hash + Eq> Hash for Bag<T> {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        let vec = Vec::from_iter(self.bag.iter());
        vec.hash(hasher)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bag_test() {
        let mut bag = Bag::new();
        bag.insert("a");
        bag.insert("b");
        bag.insert("b");
        assert_eq!(bag.contains(&"b"), 2);
    }

    #[test]
    fn deser_test() {
        let str = r#"[ ["a",2],["b",2],["a",1]]"#;
        let bag: Bag<char> = serde_json::from_str(str).unwrap();
        assert_eq!(bag, Bag::from(['a', 'a', 'a', 'b', 'b']));
    }

    #[test]
    fn bag_from_iter() {
        let bag = Bag::from_iter(vec!['a', 'b', 'a']);
        assert_eq!(bag.contains(&'a'), 2);
        assert_eq!(bag.contains(&'b'), 1);
        assert_eq!(bag.contains(&'c'), 0);
    }
}
