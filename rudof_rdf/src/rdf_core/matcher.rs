/// A trait for pattern matching against RDF components.
///
/// This trait enables flexible matching of RDF terms, subjects, and predicates
/// in queries and triple patterns. It combines matching logic with value
/// extraction, allowing both specific matches and wildcard patterns.
pub trait Matcher<T>: PartialEq<T> {
    /// Returns the underlying value if this matcher represents a specific value.
    ///
    /// Returns `None` for wildcard matchers that match any value,
    /// and `Some(value)` for matchers that represent a specific RDF component.
    fn value(&self) -> Option<&T>;
}

/// A wildcard matcher that matches any RDF component.
///
/// `Any` implements the `Matcher` trait to enable pattern matching in SPARQL-like
/// queries where certain positions in a triple pattern can match any value.
#[derive(Debug, Clone, Eq)]
pub struct Any;

impl<T> Matcher<T> for Any {
    /// Always returns `None` since `Any` matches everything without a specific value.
    fn value(&self) -> Option<&T> {
        None
    }
}

impl<T> PartialEq<T> for Any {
    /// Implements equality comparison where `Any` always equals any value.
    fn eq(&self, _other: &T) -> bool {
        true
    }
}
