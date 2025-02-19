use super::rdf::Object;
use super::rdf::Rdf;

pub enum Matcher<R: Rdf> {
    Any,
    Variable(String),
    Term(Object<R>),
}

impl<R: Rdf> PartialEq for Matcher<R> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Matcher::Any, _) | (_, Matcher::Any) => true,
            (Matcher::Variable(_), _) | (_, Matcher::Variable(_)) => true,
            (Matcher::Term(t1), Matcher::Term(t2)) => t1 == t2,
        }
    }
}

impl<I, R> From<I> for Matcher<R>
where
    R: Rdf,
    I: Into<Object<R>>,
    I: Clone, // TODO: check this
{
    fn from(value: I) -> Self {
        Matcher::Term(value.into())
    }
}
