use std::io::Read;

use super::Literal;
use super::Term;

pub trait Sparql {
    type QuerySolution: QuerySolution;
    type SparqlError: std::error::Error + 'static;

    /// Make a SPARQL query and return the solutions.
    fn make_sparql_query(
        &self,
        query: String,
    ) -> Result<Vec<Self::QuerySolution>, Self::SparqlError>;

    /// Execute an SPARQL SELECT query.
    fn select(&self, query: String) -> Result<Vec<Self::QuerySolution>, Self::SparqlError> {
        self.make_sparql_query(query)
    }

    /// Execute an SPARQL ASK query.
    fn ask(&self, query: String) -> Result<bool, Self::SparqlError> {
        self.make_sparql_query(query)?
            .first()
            .and_then(|solution| solution.get(0))
            .and_then(Term::literal)
            .and_then(Literal::as_bool)
            .ok_or_else(|| todo!())
    }
}

pub enum QueryResultsFormat {
    Json,
}

pub trait QuerySolutionParser {
    type QuerySolution: QuerySolution;
    type Error: std::error::Error + 'static;

    fn parse<R: Read>(
        format: QueryResultsFormat,
        reader: R,
    ) -> Result<Vec<Self::QuerySolution>, Self::Error>;
}

pub trait QuerySolution {
    type Value: Term;

    /// Returns the value of the solution at the given index.
    fn get(&self, index: usize) -> Option<&Self::Value>;
}
