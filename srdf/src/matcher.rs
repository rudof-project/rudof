pub struct Any;

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
