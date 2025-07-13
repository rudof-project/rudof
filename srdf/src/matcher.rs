/// A type that matches any RDF term, subject, or predicate.
/// The `Any` type implements the `Matcher` trait, allowing it to be used in RDF operations that require matching.
#[derive(Debug, Clone, Eq)]
pub struct Any;

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

// pub enum Matcher<R: Rdf> {
//     Variable(String),
//     Term(R::Term),
// }

// impl<R: Rdf> PartialEq for Matcher<R> {
//     fn eq(&self, other: &Self) -> bool {
//         match (self, other) {
//             (Matcher::Variable(_), _) | (_, Matcher::Variable(_)) => true,
//             (Matcher::Term(t1), Matcher::Term(t2)) => t1 == t2,
//         }
//     }
// }

// impl<R: Rdf> From<Any> for Matcher<R> {
//     #[inline]
//     fn from(_value: Any) -> Self {
//         Matcher::Variable("_".to_string())
//     }
// }

// impl<I, R> From<I> for Matcher<R>
// where
//     R: Rdf,
//     I: Into<R::Term>,
//     I: Clone, // TODO: check this
// {
//     fn from(value: I) -> Self {
//         Matcher::Term(value.into())
//     }
// }

// impl<R: Rdf> std::fmt::Display for Matcher<R> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Matcher::Variable(var) => write!(f, "?{}", var),
//             Matcher::Term(term) => write!(f, "{}", term),
//         }
//     }
// }
