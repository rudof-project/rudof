//! A set whose elements can be repeated. The set tracks how many times each element appears
//! 
use std::hash::Hash;
use hashbag::{HashBag, SetIter};


pub struct Bag<T> {
    bag: HashBag<T>
}

impl <T: Hash + Eq> Bag<T> {

    #[inline]
    pub fn new() -> Bag<T> {
        Bag { bag : HashBag::new() }
    }

    pub fn add(&mut self, value: T) -> usize {
        let n = self.bag.insert(value);
        n
    }

    pub fn contains(&self, value: T) -> usize {
        self.bag.contains(&value)
    }

    pub fn len(&self) -> usize {
        self.bag.len()
    }

    pub fn iter(&self) -> SetIter<'_,T> {
        self.bag.set_iter()
    }

}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bag_test() {
        let mut bag = Bag::new();
        bag.add("a");
        bag.add("b");
        bag.add("b");
        assert_eq!(bag.contains("b"), 2);
    }
}
