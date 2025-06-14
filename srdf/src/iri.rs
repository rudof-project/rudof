use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;

pub trait Iri: Debug + Display + Hash + Eq + Ord + Clone {
    fn as_str(&self) -> &str;
}
