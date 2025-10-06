/// A trait for matching RDF terms, subjects, or predicates.
/// This trait is used to define how to match RDF components in queries.
/// It can be implemented for specific types or used with the `Any` type to match any term.
///
/// Implementations of this trait should provide a way to check if a term matches a specific value
/// or to retrieve the value of the term if it matches.
///
/// The `Matcher` trait is used in various RDF operations, such as querying triples or filtering
/// based on specific criteria. It allows for flexible and dynamic matching of RDF components.
///
pub trait Matcher<T>: PartialEq<T> {
    fn value(&self) -> Option<T>;
}

impl<T> Matcher<T> for Any {
    fn value(&self) -> Option<T> {
        None
    }
}

impl<T> PartialEq<T> for Any {
    fn eq(&self, _other: &T) -> bool {
        true
    }
}

/// A type that matches any RDF term, subject, or predicate.
/// The `Any` type implements the `Matcher` trait, allowing it to be used in RDF operations that require matching.
#[derive(Debug, Clone, Eq)]
pub struct Any;
