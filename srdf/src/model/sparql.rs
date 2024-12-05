use super::Literal;
use super::Term;

pub trait Sparql {
    type Value: Term;
    type QuerySolution: QuerySolution<Self::Value>;
    type SparqlError: std::error::Error + 'static;

    /// Make a SPARQL query and return the solutions.
    fn make_sparql_query(&self, query: &str)
        -> Result<Vec<Self::QuerySolution>, Self::SparqlError>;

    /// Execute an SPARQL SELECT query.
    fn select(&self, query: &str) -> Result<Vec<Self::QuerySolution>, Self::SparqlError> {
        self.make_sparql_query(query)
    }

    /// Execute an SPARQL ASK query.
    fn ask(&self, query: &str) -> Result<bool, Self::SparqlError> {
        self.make_sparql_query(query)?
            .first()
            .and_then(|solution| solution.get(0))
            .and_then(Self::Value::literal)
            .and_then(Literal::as_bool)
            .ok_or_else(|| todo!())
    }
}

pub trait QuerySolution<R> {
    /// Returns the value of the solution at the given index.
    fn get(&self, index: usize) -> Option<&R>;
}
