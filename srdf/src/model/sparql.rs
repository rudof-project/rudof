use std::fmt::Display;

use super::Literal;
use super::Term;

pub trait Sparql {
    type QuerySolution: QuerySolution<Self::Object>;
    type Object: Term;
    type SparqlError: Display;

    fn make_sparql_query(&self, query: &str)
        -> Result<Vec<Self::QuerySolution>, Self::SparqlError>;

    fn select(&self, query: &str) -> Result<Vec<Self::QuerySolution>, Self::SparqlError> {
        self.make_sparql_query(query)
    }

    fn ask(&self, query: &str) -> Result<bool, Self::SparqlError> {
        self.make_sparql_query(query)?
            .first()
            .and_then(|solution| solution.get(0))
            .and_then(Term::literal)
            .and_then(Literal::as_bool)
            .ok_or_else(|| todo!())
    }
}

pub trait QuerySolution<R> {
    fn get(&self, index: usize) -> Option<&R>;
}
