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

/// A matcher for one value, or for any value: the `Option` form of [`Any`].
///
/// `Matcher` is a trait, so a triple pattern whose positions are only known at run
/// time (each one either a value or a wildcard) cannot pick between [`Any`] and a
/// concrete term without a branch per position. `AnyOr` carries that choice as data
/// instead, so such a pattern is one `triples_matching` call.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnyOr<T>(Option<T>);

impl<T> AnyOr<T> {
    /// Matches `value`, or any value if it is `None`.
    pub fn new(value: Option<T>) -> Self {
        AnyOr(value)
    }

    /// Matches any value, like [`Any`].
    pub fn any() -> Self {
        AnyOr(None)
    }
}

impl<T: PartialEq> Matcher<T> for AnyOr<T> {
    /// The value to match, or `None` when matching any.
    fn value(&self) -> Option<&T> {
        self.0.as_ref()
    }
}

impl<T: PartialEq> PartialEq<T> for AnyOr<T> {
    /// Equal to `other` if that is the value being matched, and to anything when
    /// matching any value.
    fn eq(&self, other: &T) -> bool {
        self.0.as_ref().is_none_or(|value| value == other)
    }
}
